// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link } reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
// Value/type re-exports follow the local `@semio/js` imports below (single binding per symbol).
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type {
  Attribute,
  Benchmark,
  Camera,
  ConnectionSide,
  Coordinate,
  Entity,
  FieldSpec,
  GraphRootKind,
  Kit,
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
} from "@semio/js";
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
  DESIGN_ARTIFACT_FIELD_SPECS,
  DESIGN_OPERATION_SPECS,
  Edit,
  EventBus,
  Family,
  File,
  Folder,
  Graph,
  Group,
  KIT_ARTIFACT_FIELD_SPECS,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  KIT_OPERATION_SPECS,
  kitReadPointKey,
  Layer,
  openKit,
  Operation,
  Piece,
  PiecesOperations,
  Port,
  Prop,
  Quality,
  Representation,
  Session,
  Stat,
  Tag,
  theKitReadPoint,
  TheKit,
  Type,
} from "@semio/js";
import type { ReactNode } from "react";
import * as React from "react";
// #endregion ⚛️Imports

// #region 🧷JsPublicExports
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
  DESIGN_ARTIFACT_FIELD_SPECS,
  DESIGN_OPERATION_SPECS,
  Edit,
  EventBus,
  Family,
  File,
  Folder,
  Graph,
  Group,
  Kit,
  KIT_ARTIFACT_FIELD_SPECS,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  KIT_OPERATION_SPECS,
  kitReadPointKey,
  Layer,
  openKit,
  Operation,
  Piece,
  PiecesOperations,
  Port,
  Prop,
  Quality,
  Representation,
  Session,
  Stat,
  Tag,
  theKitReadPoint,
  TheKit,
  Type,
};
export type {
  Attribute,
  Benchmark,
  Camera,
  Coordinate,
  GraphRootKind,
  Location,
  Place,
  Plane,
  Point,
  Side,
  Vector,
};
// #endregion 🧷JsPublicExports

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
  /** @emoji 📡 When set, {@link Kit#bus} {@code subscribeKind}; when omitted, only mount + {@link FieldReadState#refresh} pull fresh data. */
  eventKind?: string;
  /** @emoji 🪝  source; re-invoked each render — keep stable via {@link React#useCallback}. */
  get: () => E | null;
}>;

/**
 * @emoji 🪝 Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
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
    }, [refresh, entity?.id, entity?.kit]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      const kit = e.kit;
      if (eventKind != null && eventKind !== "") return kit.bus.subscribeKind(eventKind, () => void refresh());
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
 * @emoji 🪝 Same as {@link bindFieldToReact} but wires {@link defineField} so callers share {@link FieldSpec} with tooling/docs.
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
    }, [refresh, entity?.id, entity?.kit]);

    React.useEffect(() => {
      const e = entityRef.current;
      if (e == null) return;
      const kit = e.kit;
      if (eventKind != null && eventKind !== "") return kit.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [entity, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝FieldBind

// #region 🪝OpBind
/** @emoji 🎛️ UI-facing operation lifecycle for {@link bindOpToReact} (idle → pending → settled). */
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
 * @emoji 🪝 Binds an entity operation to `[run, status]`; `run` reads latest entity via {@code get} ref (no sync external store).
 * @typeParam E — Concrete {@link } subclass anchor.
 * @typeParam Args — Operation arguments after the entity receiver.
 */
export function bindOpToReact<E extends Entity, Args extends unknown[] = []>(impl: (entity: E, ...args: Args) => Promise<SetResult>): (get: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useOp(get: () => E | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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
// #endregion 🪝OpBind

// #region 🪝KitFieldBind
/** @emoji 🪝 Kit-scoped field bind (uses {@link Kit#bus} like {@link bindFieldToReact}). */
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
      if (eventKind != null && eventKind !== "") return k.bus.subscribeKind(eventKind, () => void refresh());
      return undefined;
    }, [kit, eventKind, refresh]);

    return { value, loading, error, refresh };
  };
}
// #endregion 🪝KitFieldBind

// #region 🪝KitOpBind
/** @emoji 🪝 Binds a {@link Kit} operation to `[run, status]`. */
export function bindKitOperationToReact<Args extends unknown[] = []>(impl: (kit: Kit, ...args: Args) => Promise<SetResult>): (getKit: () => Kit | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useKitOp(getKit: () => Kit | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(getKit);
    getRef.current = getKit;
    const [status, setStatus] = React.useState<OperationStatus>({ kind: "idle" });

    const run = React.useCallback(
      async (...args: Args) => {
        const k = getRef.current();
        if (k == null) {
          const result: SetResult = { ok: false, error: { kind: "Disposed", message: "No kit in React context.", field: undefined, entity: undefined } };
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
// #endregion 🪝KitOpBind

// #region 🫳ShellHost
/** @emoji 🪟 Sketchpad kit-store factory signature (host wiring; store shape is host-owned). */
export type SketchpadKitStoreFactory = (kit: Kit) => Promise<unknown>;

/** @emoji 🪟 Which persistence-backed kit open paths are available in the host shell. */
export type SketchpadKitKindAvailability = Readonly<Record<"temporary" | "file" | "folder" | "remote", boolean>>;

/** @emoji 🧭 Tab-shell kit id for routing (not the GraphQL {@link Kit#readId} async field). */
export type ActiveKitTabValue = Readonly<{ id: string }>;

export const ActiveKitTabContext = React.createContext<ActiveKitTabValue | null>(null);

/** @emoji 🧭 Binds the active tab kit id for sketchpad routing and machine events. */
export function ActiveKitTabContextProvider(props: { kitTabId: string; children: ReactNode }): React.ReactElement {
  const v = React.useMemo<ActiveKitTabValue>(() => ({ id: props.kitTabId }), [props.kitTabId]);
  return React.createElement(ActiveKitTabContext.Provider, { value: v }, props.children);
}

/** @emoji 🧭 Reads {@link ActiveKitTabContextProvider} as `{ id }` for legacy call-site shape. */
export function useActiveKitTab(): ActiveKitTabValue | null {
  return React.useContext(ActiveKitTabContext);
}

/** @emoji 🧭 True when {@link ActiveKitTabContextProvider} is mounted above. */
export function useIsInActiveKitTab(): boolean {
  return React.useContext(ActiveKitTabContext) != null;
}

/** @emoji 🔌 Optional WASM host bindings (store + client) parallel to {@link KitContextProvider}. */
export type KitWasmHostState = Readonly<{ kitTabId: string; store: unknown; kitClient: unknown | null }>;

const KitWasmHostContext = React.createContext<KitWasmHostState | null>(null);

/** @emoji 🔌 Reads {@link KitWasmMountProvider} host bindings (never a synthetic runtime umbrella). */
export function useKitWasmHost(): KitWasmHostState | null {
  return React.useContext(KitWasmHostContext);
}

export type KitWasmMountProviderProps = Readonly<{
  kitId?: string;
  store: unknown;
  kitClient?: unknown;
  kit?: Kit | null;
  children: ReactNode;
}>;

/** @emoji 🔌 Publishes host store/client and optionally wraps {@link KitContextProvider} when {@code kit} is known. */
export function KitWasmMountProvider(props: KitWasmMountProviderProps): React.ReactElement {
  const host = React.useMemo<KitWasmHostState>(
    () => ({ kitTabId: props.kitId ?? "", store: props.store, kitClient: props.kitClient ?? null }),
    [props.kitId, props.store, props.kitClient],
  );
  const inner =
    props.kit != null ? React.createElement(KitContextProvider, { kit: props.kit }, props.children) : props.children;
  return React.createElement(KitWasmHostContext.Provider, { value: host }, inner);
}

const KitAlternativeSelectionContext = React.createContext<Readonly<{ kitId: string }> | null>(null);

/** @emoji 🌿 Local alternative selection scope for sketchpad footer (host VCS wiring may replace reads later). */
export function KitAlternativeSelectionProvider(props: { kitId: string; children: ReactNode }): React.ReactElement {
  const v = React.useMemo(() => ({ kitId: props.kitId }), [props.kitId]);
  return React.createElement(KitAlternativeSelectionContext.Provider, { value: v }, props.children);
}

/** @emoji 🌿 `[selectedId, setSelectedId]` for the current {@link KitAlternativeSelectionProvider}. */
export function useKitAlternativeSelection(): readonly [string | null, (next: string | null) => void] {
  const ctx = React.useContext(KitAlternativeSelectionContext);
  const kitId = ctx?.kitId ?? null;
  const [selected, setSelected] = React.useState<string | null>(null);
  React.useEffect(() => {
    setSelected(null);
  }, [kitId]);
  return [selected, setSelected] as const;
}

/** @emoji 🌿 Stub list until VCS alternatives are bound to {@link Kit} GraphQL reads in the host. */
export function useKitAlternatives(): readonly unknown[] {
  return React.useMemo(() => [], []);
}
// #endregion 🫳ShellHost

// #region 🎭Contexts
// #region 🎒Kit
const KitContext = React.createContext<Kit | null>(null);

export type KitContextProviderProps = Readonly<{
  kit: Kit;
  initialReadPoint?: KitReadPoint;
  children: ReactNode;
}>;

/** @emoji 🧭 Provides {@link Kit}; keeps {@link KitReadPoint} in React state and applies it with {@link Kit#setReadPoint}. */
export function KitContextProvider(props: KitContextProviderProps): React.ReactElement {
  const [readPoint, setReadPointState] = React.useState<KitReadPoint>(props.initialReadPoint ?? theKitReadPoint);
  React.useEffect(() => {
    props.kit.setReadPoint(readPoint);
  }, [props.kit, readPoint]);
  return React.createElement(KitContext.Provider, { value: props.kit }, props.children);
}

/** @emoji 🧭 Requires {@link KitContextProvider}; returns the GraphQL {@link Kit} handle. */
export function useKit(): Kit {
  const k = React.useContext(KitContext);
  if (k == null) throw new Error("semio/react: useKit requires <KitContextProvider>.");
  return k;
}

/** @emoji 🌐 WIP {@link Graph} from {@link Kit#wip} (no extra provider). */
export function useWipGraph(): Graph {
  const kit = useKit();
  return React.useMemo(() => kit.wip(), [kit]);
}

/** @emoji 🌐 Authoritative {@link Graph} from {@link Kit#authoritative}. */
export function useAuthoritativeGraph(): Graph {
  const kit = useKit();
  return React.useMemo(() => kit.authoritative(), [kit]);
}

/** @emoji 🗂️ Root {@link Session} from {@link Kit#session}. */
export function useSession(): Session {
  const kit = useKit();
  return React.useMemo(() => kit.session(), [kit]);
}

// #endregion 🎒Kit

// #region 🌐GraphContext
export type GraphContextValue = Readonly<{ root: GraphRootKind }>;

const GraphRootContext = React.createContext<GraphContextValue | null>(null);

/** @emoji 🌐 Binds {@link GraphRootKind} for {@link useGraph}. */
export function GraphContextProvider(props: { root: GraphRootKind; children: ReactNode }): React.ReactElement {
  const v = React.useMemo<GraphContextValue>(() => ({ root: props.root }), [props.root]);
  return React.createElement(GraphRootContext.Provider, { value: v }, props.children);
}

/** @emoji 🌐 {@link Graph} for the current {@link GraphContextProvider} {@code root}. */
export function useGraph(): Graph {
  const kit = useKit();
  const ctx = React.useContext(GraphRootContext);
  if (ctx == null) throw new Error("semio/react: useGraph requires <GraphContextProvider root=\"wip\"|\"authoritative\">.");
  return React.useMemo(() => new Graph(kit, ctx.root), [kit, ctx.root]);
}

// #endregion 🌐GraphContext

// #region 📐Design
export type DesignContext = Readonly<{ designId: string }>;
const DesignContext = React.createContext<DesignContext | null>(null);
export function DesignContextProvider(props: { designId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(DesignContext.Provider, { value: { designId: props.designId } }, props.children);
}
export function useDesign(): Design | null {
  const kit = useKit();
  const ctx = React.useContext(DesignContext);
  return ctx == null ? null : kit.design(ctx.designId);
}
// #endregion 📐Design

// #endregion 🎭Contexts

// #region 🪢Contexts
export type PieceContext = Readonly<{ designId: string; pieceId: string }>;
const PieceContext = React.createContext<PieceContext | null>(null);
export function PieceContextProvider(props: PieceContext & { children: ReactNode }): React.ReactElement {
  return React.createElement(PieceContext.Provider, { value: { designId: props.designId, pieceId: props.pieceId } }, props.children);
}
export function usePiece(): Piece | null {
  const kit = useKit();
  const ctx = React.useContext(PieceContext);
  return ctx == null ? null : kit.design(ctx.designId).piece(ctx.pieceId);
}

export type TypeContext = Readonly<{ typeId: string }>;
const TypeContext = React.createContext<TypeContext | null>(null);
export function TypeContextProvider(props: { typeId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(TypeContext.Provider, { value: { typeId: props.typeId } }, props.children);
}
export function useType(): Type | null {
  const kit = useKit();
  const ctx = React.useContext(TypeContext);
  return ctx == null ? null : kit.type(ctx.typeId);
}

export type ConnectionContext = Readonly<{ designId: string; connectionId: string }>;
const ConnectionContext = React.createContext<ConnectionContext | null>(null);
export function ConnectionContextProvider(props: ConnectionContext & { children: ReactNode }): React.ReactElement {
  return React.createElement(ConnectionContext.Provider, { value: { designId: props.designId, connectionId: props.connectionId } }, props.children);
}
export function useConnection(): Connection | null {
  const kit = useKit();
  const ctx = React.useContext(ConnectionContext);
  return ctx == null ? null : kit.design(ctx.designId).connection(ctx.connectionId);
}

export type PortContext = Readonly<{ typeId: string; portId: string }>;
const PortContext = React.createContext<PortContext | null>(null);
export function PortContextProvider(props: PortContext & { children: ReactNode }): React.ReactElement {
  return React.createElement(PortContext.Provider, { value: { typeId: props.typeId, portId: props.portId } }, props.children);
}
export function usePort(): Port | null {
  const kit = useKit();
  const ctx = React.useContext(PortContext);
  return ctx == null ? null : kit.type(ctx.typeId).port(ctx.portId);
}

export type ConnectorContext = Readonly<{ typeId: string; connectorId: string }>;
const ConnectorContext = React.createContext<ConnectorContext | null>(null);
export function ConnectorContextProvider(props: ConnectorContext & { children: ReactNode }): React.ReactElement {
  return React.createElement(ConnectorContext.Provider, { value: { typeId: props.typeId, connectorId: props.connectorId } }, props.children);
}
export function useConnector(): Connector | null {
  const kit = useKit();
  const ctx = React.useContext(ConnectorContext);
  return ctx == null ? null : kit.type(ctx.typeId).connector(ctx.connectorId);
}

export type QualityContext = Readonly<{ qualityId: string }>;
const QualityContext = React.createContext<QualityContext | null>(null);
export function QualityContextProvider(props: { qualityId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(QualityContext.Provider, { value: { qualityId: props.qualityId } }, props.children);
}
export function useQuality(): Quality | null {
  const kit = useKit();
  const ctx = React.useContext(QualityContext);
  return ctx == null ? null : kit.quality(ctx.qualityId);
}

export type TagContext = Readonly<{ tagId: string }>;
const TagContext = React.createContext<TagContext | null>(null);
export function TagContextProvider(props: { tagId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(TagContext.Provider, { value: { tagId: props.tagId } }, props.children);
}
export function useTag(): Tag | null {
  const kit = useKit();
  const ctx = React.useContext(TagContext);
  return ctx == null ? null : kit.tag(ctx.tagId);
}

export type ConceptContext = Readonly<{ conceptId: string }>;
const ConceptContext = React.createContext<ConceptContext | null>(null);
export function ConceptContextProvider(props: { conceptId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(ConceptContext.Provider, { value: { conceptId: props.conceptId } }, props.children);
}
export function useConcept(): Concept | null {
  const kit = useKit();
  const ctx = React.useContext(ConceptContext);
  return ctx == null ? null : kit.concept(ctx.conceptId);
}

export type AuthorContext = Readonly<{ authorId: string }>;
const AuthorContext = React.createContext<AuthorContext | null>(null);
export function AuthorContextProvider(props: { authorId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(AuthorContext.Provider, { value: { authorId: props.authorId } }, props.children);
}
export function useAuthor(): Author | null {
  const kit = useKit();
  const ctx = React.useContext(AuthorContext);
  return ctx == null ? null : kit.author(ctx.authorId);
}

export type RepresentationContext = Readonly<{ typeId: string; representationId: string }>;
const RepresentationContext = React.createContext<RepresentationContext | null>(null);
export function RepresentationContextProvider(props: RepresentationContext & { children: ReactNode }): React.ReactElement {
  return React.createElement(RepresentationContext.Provider, { value: { typeId: props.typeId, representationId: props.representationId } }, props.children);
}
export function useRepresentation(): Representation | null {
  const kit = useKit();
  const ctx = React.useContext(RepresentationContext);
  return ctx == null ? null : kit.type(ctx.typeId).representation(ctx.representationId);
}
// #endregion 🪢Contexts

// #region 🔖EntityContextHelpers
/** @emoji 🧭 `{ id }` view of {@link DesignContext} for sketchpad routing (no entity fetch). */
export function useDesignContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(DesignContext);
  return ctx == null ? null : { id: ctx.designId };
}

/** @emoji 🧭 True when a {@link DesignContextProvider} is mounted above. */
export function useHasDesignContext(): boolean {
  return React.useContext(DesignContext) != null;
}

/** @emoji 🧭 `{ id }` view of {@link PieceContext} (piece id only). */
export function usePieceContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(PieceContext);
  return ctx == null ? null : { id: ctx.pieceId };
}

/** @emoji 🧭 True when {@link PieceContextProvider} is mounted above. */
export function useHasPieceContext(): boolean {
  return React.useContext(PieceContext) != null;
}

/** @emoji 🧭 `{ id }` view of {@link ConnectionContext}. */
export function useConnectionContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(ConnectionContext);
  return ctx == null ? null : { id: ctx.connectionId };
}

/** @emoji 🧭 True when {@link ConnectionContextProvider} is mounted above. */
export function useHasConnectionContext(): boolean {
  return React.useContext(ConnectionContext) != null;
}

/** @emoji 🧭 `{ id }` view of {@link TypeContext}. */
export function useTypeContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(TypeContext);
  return ctx == null ? null : { id: ctx.typeId };
}

/** @emoji 🧭 True when {@link TypeContextProvider} is mounted above. */
export function useHasTypeContext(): boolean {
  return React.useContext(TypeContext) != null;
}

/** @emoji 🧭 `{ id }` view of {@link QualityContext}. */
export function useQualityContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(QualityContext);
  return ctx == null ? null : { id: ctx.qualityId };
}

/** @emoji 🧭 True when {@link QualityContextProvider} is mounted above. */
export function useHasQualityContext(): boolean {
  return React.useContext(QualityContext) != null;
}

/** @emoji 🧭 `{ id }` view of {@link AuthorContext}. */
export function useAuthorContextRow(): Readonly<{ id: string }> | null {
  const ctx = React.useContext(AuthorContext);
  return ctx == null ? null : { id: ctx.authorId };
}

/** @emoji 🧭 True when {@link AuthorContextProvider} is mounted above. */
export function useHasAuthorContext(): boolean {
  return React.useContext(AuthorContext) != null;
}

/** @emoji 🧷 {@link PieceContextProvider} using the enclosing {@link DesignContextProvider} {@code designId}. */
export function PieceUnderActiveDesignProvider(props: { pieceId: string; children: ReactNode }): React.ReactElement {
  const d = React.useContext(DesignContext);
  if (d == null) {
    throw new Error("semio/react: PieceUnderActiveDesignProvider requires <DesignContextProvider designId=\"…\">.");
  }
  return React.createElement(PieceContext.Provider, { value: { designId: d.designId, pieceId: props.pieceId } }, props.children);
}

/** @emoji 🧷 {@link ConnectionContextProvider} using the enclosing {@link DesignContextProvider} {@code designId}. */
export function ConnectionUnderActiveDesignProvider(props: { connectionId: string; children: ReactNode }): React.ReactElement {
  const d = React.useContext(DesignContext);
  if (d == null) {
    throw new Error("semio/react: ConnectionUnderActiveDesignProvider requires <DesignContextProvider designId=\"…\">.");
  }
  return React.createElement(ConnectionContext.Provider, { value: { designId: d.designId, connectionId: props.connectionId } }, props.children);
}
// #endregion 🔖EntityContextHelpers

// #region 🪝IdStableEntityLists
/** @emoji 📚 Kit-level designs ordered by {@link Kit#readDesignIds} (handles from {@link Kit#design}). */
export function useKitDesignEntities(): FieldReadState<readonly Design[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Design[]>({
    getKit: () => kit,
    read: async (k) => (await k.readDesignIds()).map((id) => k.design(id)),
  })();
}

/** @emoji 📚 Kit-level types ordered by {@link Kit#readTypeIds}. */
export function useKitTypeEntities(): FieldReadState<readonly Type[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Type[]>({
    getKit: () => kit,
    read: async (k) => (await k.readTypeIds()).map((id) => k.type(id)),
  })();
}

/** @emoji 📚 Kit-level authors ordered by {@link Kit#readAuthorIds}. */
export function useKitAuthorEntities(): FieldReadState<readonly Author[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Author[]>({
    getKit: () => kit,
    read: async (k) => (await k.readAuthorIds()).map((id) => k.author(id)),
  })();
}

/** @emoji 📚 Kit-level qualities ordered by {@link Kit#readQualityIds}. */
export function useKitQualityEntities(): FieldReadState<readonly Quality[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Quality[]>({
    getKit: () => kit,
    read: async (k) => (await k.readQualityIds()).map((id) => k.quality(id)),
  })();
}

/** @emoji 📚 Kit-level tags ordered by {@link Kit#readTagIds}. */
export function useKitTagEntities(): FieldReadState<readonly Tag[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Tag[]>({
    getKit: () => kit,
    read: async (k) => (await k.readTagIds()).map((id) => k.tag(id)),
  })();
}

/** @emoji 📚 Kit-level concepts ordered by {@link Kit#readConceptIds}. */
export function useKitConceptEntities(): FieldReadState<readonly Concept[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly Concept[]>({
    getKit: () => kit,
    read: async (k) => (await k.readConceptIds()).map((id) => k.concept(id)),
  })();
}

/** @emoji 📚 Design pieces ordered by {@link Design#readPieceIds}. */
export function useDesignPieceEntities(): FieldReadState<readonly Piece[]> {
  const d = useDesign();
  return bindFieldToReact<Design, readonly Piece[]>({
    get: () => d,
    read: async (design) => (await design.readPieceIds()).map((id) => design.piece(id)),
  })();
}

/** @emoji 📚 Design connections ordered by {@link Design#readConnectionIds}. */
export function useDesignConnectionEntities(): FieldReadState<readonly Connection[]> {
  const d = useDesign();
  return bindFieldToReact<Design, readonly Connection[]>({
    get: () => d,
    read: async (design) => (await design.readConnectionIds()).map((id) => design.connection(id)),
  })();
}
// #endregion 🪝IdStableEntityLists

// #region 🪝HooksKit
// #region 📖KitReads
/** @emoji 📖 Live {@link Kit#readName} + {@code kitRenamed}. */
export function useKitName(): FieldReadState<string> {
  const kit = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readName(), eventKind: "kitRenamed" })();
}

/** @emoji 📖 Live {@link Kit#readDescription} + {@code changedDescription}. */
export function useKitDescription(): FieldReadState<string> {
  const kit = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readDescription(), eventKind: "changedDescription" })();
}

/** @emoji 📖 Live {@link Kit#readId}. */
export function useKitId(): FieldReadState<string> {
  const kit = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readId() })();
}

/** @emoji 📖 Live {@link Kit#readIcon}. */
export function useKitIcon(): FieldReadState<string> {
  const kit = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readIcon() })();
}

/** @emoji 📖 Live {@link Kit#readImage}. */
export function useKitImage(): FieldReadState<string> {
  const kit = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readImage() })();
}

/** @emoji 📖 Live {@link Kit#readTypeIds}. */
export function useKitTypeIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readTypeIds() })();
}

/** @emoji 📖 Live {@link Kit#readDesignIds}. */
export function useKitDesignIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readDesignIds() })();
}

/** @emoji 📖 Live {@link Kit#readAuthorIds}. */
export function useKitAuthorIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readAuthorIds() })();
}

/** @emoji 📖 Live {@link Kit#readQualityIds}. */
export function useKitQualityIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readQualityIds() })();
}

/** @emoji 📖 Live {@link Kit#readTagIds}. */
export function useKitTagIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readTagIds() })();
}

/** @emoji 📖 Live {@link Kit#readConceptIds}. */
export function useKitConceptIds(): FieldReadState<readonly string[]> {
  const kit = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readConceptIds() })();
}

/** @emoji 🧾 Exposes {@link Kit#ensureChangeId} as a stable callback. */
export function useEnsureKitChangeId(): () => Promise<string> {
  const kit = useKit();
  return React.useCallback(() => kit.ensureChangeId(), [kit]);
}
// #endregion 📖KitReads

// #region ✍️KitWrites
/** @emoji ✍️ {@link Kit#rename}. */
export function useRenameKit(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, newName) => k.rename(newName))(() => kit);
}

/** @emoji ✍️ {@link Kit#changeDescription}. */
export function useChangeKitDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, d) => k.changeDescription(d))(() => kit);
}

/** @emoji ✍️ {@link Kit#createTag}. */
export function useCreateTag(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createTag(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTag}. */
export function useDeleteTag(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.deleteTag(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTags}. */
export function useDeleteTags(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[readonly string[]]>((k, ids) => k.deleteTags(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createConcept}. */
export function useCreateConcept(): readonly [(name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) => k.createConcept(n, d, i, o))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcept}. */
export function useDeleteConcept(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.deleteConcept(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcepts}. */
export function useDeleteConcepts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[readonly string[]]>((k, ids) => k.deleteConcepts(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createQuality}. */
export function useCreateQuality(): readonly [(key: string, value?: string | null, unit?: string | null, definition?: string | null, description?: string | null, icon?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, key, value, unit, definition, description, icon) =>
    k.createQuality(key, value, unit, definition, description, icon),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQuality}. */
export function useDeleteQuality(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.deleteQuality(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQualities}. */
export function useDeleteQualities(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[readonly string[]]>((k, ids) => k.deleteQualities(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createType}. */
export function useCreateType(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createType(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteType}. */
export function useDeleteType(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.deleteType(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTypes}. */
export function useDeleteTypes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[readonly string[]]>((k, ids) => k.deleteTypes(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createDesign}. */
export function useCreateDesign(): readonly [(name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) => k.createDesign(n, d, i, im, u))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesign}. */
export function useDeleteDesign(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.deleteDesign(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesigns}. */
export function useDeleteDesigns(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[readonly string[]]>((k, ids) => k.deleteDesigns(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#saveChange}. */
export function useSaveKitChange(): readonly [() => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[]>(async (k) => {
    await k.saveChange();
    return { ok: true };
  })(() => kit);
}

/** @emoji ✍️ {@link Kit#createCheckpoint}. */
export function useCreateCheckpoint(): readonly [(message: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, message) => k.createCheckpoint(message))(() => kit);
}

/** @emoji ✍️ {@link Kit#startAlternative}. */
export function useStartAlternative(): readonly [(name?: string | null) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string | null | undefined]>((k, name) => k.startAlternative(name ?? undefined))(() => kit);
}

/** @emoji ✍️ {@link Kit#integrateAlternative}. */
export function useIntegrateAlternative(): readonly [(alternativeId: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string]>((k, id) => k.integrateAlternative(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#login}. */
export function useLogin(): readonly [(username: string, passwordHash: string, hubUrl?: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[string, string, string | undefined]>((k, u, p, h) => k.login(u, p, h))(() => kit);
}

/** @emoji ✍️ {@link Kit#logout}. */
export function useLogout(): readonly [() => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[]>((k) => k.logout())(() => kit);
}

/** @emoji ✍️ {@link Kit#sessionStart}. */
export function useStartSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[]>((k) => k.sessionStart())(() => kit);
}

/** @emoji ✍️ {@link Kit#sessionEnd}. */
export function useEndSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  return bindKitOperationToReact<[]>((k) => k.sessionEnd())(() => kit);
}

// #endregion 🪝HooksKit

// #region 🪝HooksDesign
// #region 📖DesignReads
/** @emoji 📖 Live {@link Design#readName}. */
export function useDesignName(): FieldReadState<string> {
  const entity = useDesign();
  return bindFieldToReact<Design, string>({ get: () => entity, read: (d) => d.readName() })();
}

/** @emoji 📖 Live {@link Design#readDescription} + {@code changedDescription}. */
export function useDesignDescription(): FieldReadState<string> {
  const entity = useDesign();
  return bindFieldToReact<Design, string>({ get: () => entity, read: (d) => d.readDescription(), eventKind: "changedDescription" })();
}

/** @emoji 📖 Live {@link Design#readPieceIds}. */
export function useDesignPieceIds(): FieldReadState<readonly string[]> {
  const entity = useDesign();
  return bindFieldToReact<Design, readonly string[]>({ get: () => entity, read: (d) => d.readPieceIds() })();
}

/** @emoji 📖 Live {@link Design#readConnectionIds}. */
export function useDesignConnectionIds(): FieldReadState<readonly string[]> {
  const entity = useDesign();
  return bindFieldToReact<Design, readonly string[]>({ get: () => entity, read: (d) => d.readConnectionIds() })();
}

/** @emoji 📖 Live {@link Design#readAttributeIds}. */
export function useDesignAttributeIds(): FieldReadState<readonly string[]> {
  const entity = useDesign();
  return bindFieldToReact<Design, readonly string[]>({ get: () => entity, read: (d) => d.readAttributeIds() })();
}

/** @emoji 📖 Live {@link Design#readQualitySum}. */
export function useDesignQualitySum(): FieldReadState<number> {
  const entity = useDesign();
  return bindFieldToReact<Design, number>({ get: () => entity, read: (d) => d.readQualitySum() })();
}
// #endregion 📖DesignReads

// #region ✍️DesignWrites
/** @emoji ✍️ {@link Design#rename}. */
export function useRenameDesign(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string]>((d, n) => d.rename(n))(() => entity);
}

/** @emoji ✍️ {@link Design#changeDescription}. */
export function useChangeDesignDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string]>((d, t) => d.changeDescription(t))(() => entity);
}

/** @emoji ✍️ {@link Design#flatten}. */
export function useFlattenDesign(): readonly [() => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, []>((d) => d.flatten())(() => entity);
}

/** @emoji ✍️ {@link Design#addAttribute}. */
export function useAddDesignAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string, string, string]>((d, k, v, def) => d.addAttribute(k, v, def))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttribute}. */
export function useRemoveDesignAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string]>((d, id) => d.removeAttribute(id))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttributes}. */
export function useRemoveDesignAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [readonly string[]]>((d, ids) => d.removeAttributes(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#addFixedPiece}. */
export function useAddFixedPiece(): readonly [(blueprintId: string, position: PositionInput, name?: string | null, description?: string | null) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string, PositionInput, string | null | undefined, string | null | undefined]>((d, bp, pos, n, desc) => d.addFixedPiece(bp, pos, n, desc))(() => entity);
}

/** @emoji ✍️ {@link Design#addChildPieceWithParentConnection}. */
export function useAddChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, name?: string | null, description?: string | null, position?: PositionInput | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesign();
  return bindOpToReact<Design, [string, string, string, string, string | null | undefined, string | null | undefined, PositionInput | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, n, desc, pos, sc) =>
    d.addChildPieceWithParentConnection(bp, pp, pc, cc, n, desc, pos, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#addHangingChildPieceWithParentConnection}. */
export function useAddHangingChildPieceWithParentConnection(): readonly [
  (blueprintId: string, parentPieceId: string, parentConnector: string, childConnector: string, position: PositionInput, name?: string | null, description?: string | null, scale?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesign();
  return bindOpToReact<Design, [string, string, string, string, PositionInput, string | null | undefined, string | null | undefined, number | null | undefined]>((d, bp, pp, pc, cc, pos, n, desc, sc) =>
    d.addHangingChildPieceWithParentConnection(bp, pp, pc, cc, pos, n, desc, sc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiece}. */
export function useDeleteDesignPiece(): readonly [(pieceId: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [string]>((d, id) => d.deletePiece(id))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePieces}. */
export function useDeleteDesignPieces(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [readonly string[]]>((d, ids) => d.deletePieces(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiecesAndConnections}. */
export function useDeleteDesignPiecesAndConnections(): readonly [(pieceIds: readonly string[], connectionIds: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesign();
  return bindOpToReact<Design, [readonly string[], readonly string[]]>((d, p, c) => d.deletePiecesAndConnections(p, c))(() => entity);
}
// #endregion ✍️DesignWrites
// #endregion 🪝HooksDesign

// #region 🧰Type
const useTypeRenameOp = bindOpToReact<Type, [string]>((t, newName) => t.rename(newName));
const useTypeChangeDescriptionOp = bindOpToReact<Type, [string]>((t, d) => t.changeDescription(d));
const useTypeChangeIconOp = bindOpToReact<Type, [string]>((t, i) => t.changeIcon(i));
const useTypeAddAttributeOp = bindOpToReact<Type, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTypeRemoveAttributeOp = bindOpToReact<Type, [string]>((t, id) => t.removeAttribute(id));
const useTypeRemoveAttributesOp = bindOpToReact<Type, [readonly string[]]>((t, ids) => t.removeAttributes(ids));
const useTypeCreatePortOp = bindOpToReact<Type, [string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, number | null | undefined]>((t, code, label, description, icon, order) =>
  t.createPort(code ?? null, label ?? null, description ?? null, icon ?? null, order ?? null),
);
const useTypeDeletePortOp = bindOpToReact<Type, [string]>((t, id) => t.deletePort(id));
const useTypeDeletePortsOp = bindOpToReact<Type, [readonly string[]]>((t, ids) => t.deletePorts(ids));
const useTypeAddConnectorOp = bindOpToReact<Type, [string, string | null | undefined, string | null | undefined, string | null | undefined]>((t, code, description, icon, portId) =>
  t.addConnector(code, description ?? null, icon ?? null, portId ?? null),
);
const useTypeRemoveConnectorOp = bindOpToReact<Type, [string]>((t, id) => t.removeConnector(id));
const useTypeRemoveConnectorsOp = bindOpToReact<Type, [readonly string[]]>((t, ids) => t.removeConnectors(ids));

/** @emoji 📖 Live {@link Type#readName}. */
export function useTypeName(): FieldReadState<string> {
  const entity = useType();
  return bindFieldToReact<Type, string>({ get: () => entity, read: (t) => t.readName() })();
}

/** @emoji 📖 Live {@link Type#readDescription}. */
export function useTypeDescription(): FieldReadState<string> {
  const entity = useType();
  return bindFieldToReact<Type, string>({ get: () => entity, read: (t) => t.readDescription() })();
}

/** @emoji 📖 Live {@link Type#readIcon}. */
export function useTypeIcon(): FieldReadState<string> {
  const entity = useType();
  return bindFieldToReact<Type, string>({ get: () => entity, read: (t) => t.readIcon() })();
}

/** @emoji 📖 Live {@link Type#readImage}. */
export function useTypeImage(): FieldReadState<string> {
  const entity = useType();
  return bindFieldToReact<Type, string>({ get: () => entity, read: (t) => t.readImage() })();
}

/** @emoji 📖 Live {@link Type#readUnit}. */
export function useTypeUnit(): FieldReadState<string> {
  const entity = useType();
  return bindFieldToReact<Type, string>({ get: () => entity, read: (t) => t.readUnit() })();
}

/** @emoji 📖 Bulky {@link Type#readConnectors}. */
export function useTypeConnectors(): FieldReadState<readonly { readonly id: string; readonly code: string; readonly name: string }[]> {
  const entity = useType();
  return bindFieldToReact<Type, readonly { readonly id: string; readonly code: string; readonly name: string }[]>({ get: () => entity, read: (t) => t.readConnectors() })();
}

/** @emoji 📖 Bulky {@link Type#readRepresentations}. */
export function useTypeRepresentations(): FieldReadState<readonly { readonly id: string }[]> {
  const entity = useType();
  return bindFieldToReact<Type, readonly { readonly id: string }[]>({ get: () => entity, read: (t) => t.readRepresentations() })();
}

/** @emoji 📖 Bulky {@link Type#readAttributes}. */
export function useTypeAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useType();
  return bindFieldToReact<Type, readonly Attribute[]>({ get: () => entity, read: (t) => t.readAttributes() })();
}

/** @emoji ✍️ {@link TypeOperationInput#rename}. */
export function useRenameType(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRenameOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeDescription}. */
export function useChangeTypeDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeIcon}. */
export function useChangeTypeIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeChangeIconOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addAttribute}. */
export function useAddTypeAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttribute}. */
export function useRemoveTypeAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttributes}. */
export function useRemoveTypeAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveAttributesOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#createPort}. */
export function useCreatePort(): readonly [(code?: string | null, label?: string | null, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeCreatePortOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePort}. */
export function useDeletePort(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeDeletePortOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePorts}. */
export function useDeletePorts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeDeletePortsOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addConnector}. */
export function useAddConnector(): readonly [(code: string, description?: string | null, icon?: string | null, portId?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeAddConnectorOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnector}. */
export function useRemoveConnector(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveConnectorOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnectors}. */
export function useRemoveConnectors(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useType();
  return useTypeRemoveConnectorsOp(() => e);
}
// #endregion 🧰Type

// #region 🔘Port
const usePortRenameOp = bindOpToReact<Port, [string, string | null | undefined]>((p, newCode, newLabel) => p.rename(newCode, newLabel));
const usePortChangeDescriptionOp = bindOpToReact<Port, [string]>((p, d) => p.changeDescription(d));
const usePortChangeIconOp = bindOpToReact<Port, [string]>((p, i) => p.changeIcon(i));
const usePortAddAttributeOp = bindOpToReact<Port, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePortRemoveAttributeOp = bindOpToReact<Port, [string]>((p, id) => p.removeAttribute(id));
const usePortRemoveAttributesOp = bindOpToReact<Port, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji 📖 Live {@link Port#readCode}. */
export function usePortCode(): FieldReadState<string> {
  const entity = usePort();
  return bindFieldToReact<Port, string>({ get: () => entity, read: (p) => p.readCode() })();
}

/** @emoji 📖 Live {@link Port#readLabel}. */
export function usePortLabel(): FieldReadState<string> {
  const entity = usePort();
  return bindFieldToReact<Port, string>({ get: () => entity, read: (p) => p.readLabel() })();
}

/** @emoji 📖 Live {@link Port#readOrder}. */
export function usePortOrder(): FieldReadState<number | null> {
  const entity = usePort();
  return bindFieldToReact<Port, number | null>({ get: () => entity, read: (p) => p.readOrder() })();
}

/** @emoji 📖 Live {@link Port#readName}. */
export function usePortName(): FieldReadState<string> {
  const entity = usePort();
  return bindFieldToReact<Port, string>({ get: () => entity, read: (p) => p.readName() })();
}

/** @emoji 📖 Live {@link Port#readDescription}. */
export function usePortDescription(): FieldReadState<string> {
  const entity = usePort();
  return bindFieldToReact<Port, string>({ get: () => entity, read: (p) => p.readDescription() })();
}

/** @emoji 📖 Live {@link Port#readIcon}. */
export function usePortIcon(): FieldReadState<string> {
  const entity = usePort();
  return bindFieldToReact<Port, string>({ get: () => entity, read: (p) => p.readIcon() })();
}

/** @emoji 📖 Bulky {@link Port#readAttributes}. */
export function usePortAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = usePort();
  return bindFieldToReact<Port, readonly Attribute[]>({ get: () => entity, read: (p) => p.readAttributes() })();
}

/** @emoji ✍️ {@link PortOperationInput#rename}. */
export function useRenamePort(): readonly [(newCode: string, newLabel?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRenameOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeDescription}. */
export function useChangePortDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeIcon}. */
export function useChangePortIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortChangeIconOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#addAttribute}. */
export function useAddPortAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttribute}. */
export function useRemovePortAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttributes}. */
export function useRemovePortAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePort();
  return usePortRemoveAttributesOp(() => e);
}
// #endregion 🔘Port

// #region 🔗Connector
const useConnectorRenameOp = bindOpToReact<Connector, [string]>((c, newCode) => c.rename(newCode));
const useConnectorChangeDescriptionOp = bindOpToReact<Connector, [string]>((c, d) => c.changeDescription(d));
const useConnectorChangeIconOp = bindOpToReact<Connector, [string]>((c, i) => c.changeIcon(i));

/** @emoji 📖 Live {@link Connector#readCode}. */
export function useConnectorCode(): FieldReadState<string> {
  const entity = useConnector();
  return bindFieldToReact<Connector, string>({ get: () => entity, read: (c) => c.readCode() })();
}

/** @emoji 📖 Live {@link Connector#readDescription}. */
export function useConnectorDescription(): FieldReadState<string> {
  const entity = useConnector();
  return bindFieldToReact<Connector, string>({ get: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Connector#readIcon}. */
export function useConnectorIcon(): FieldReadState<string> {
  const entity = useConnector();
  return bindFieldToReact<Connector, string>({ get: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Bulky {@link Connector#readAttributes}. */
export function useConnectorAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConnector();
  return bindFieldToReact<Connector, readonly Attribute[]>({ get: () => entity, read: (c) => c.readAttributes() })();
}

/** @emoji ✍️ {@link ConnectorOperationInput#rename}. */
export function useRenameConnector(): readonly [(newCode: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorRenameOp(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeDescription}. */
export function useChangeConnectorDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeIcon}. */
export function useChangeConnectorIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnector();
  return useConnectorChangeIconOp(() => e);
}
// #endregion 🔗Connector

// #region ✍️Author
/** @emoji 📖 Live {@link Author#readName}. */
export function useAuthorName(): FieldReadState<string> {
  const entity = useAuthor();
  return bindFieldToReact<Author, string>({ get: () => entity, read: (a) => a.readName() })();
}

/** @emoji 📖 Live {@link Author#readEmail}. */
export function useAuthorEmail(): FieldReadState<string> {
  const entity = useAuthor();
  return bindFieldToReact<Author, string>({ get: () => entity, read: (a) => a.readEmail() })();
}

/** @emoji 📖 Live {@link Author#readRank}. */
export function useAuthorRank(): FieldReadState<number | null> {
  const entity = useAuthor();
  return bindFieldToReact<Author, number | null>({ get: () => entity, read: (a) => a.readRank() })();
}

/** @emoji 📖 Live {@link Author#readDescription}. */
export function useAuthorDescription(): FieldReadState<string> {
  const entity = useAuthor();
  return bindFieldToReact<Author, string>({ get: () => entity, read: (a) => a.readDescription() })();
}

/** @emoji 📖 Live {@link Author#readIcon}. */
export function useAuthorIcon(): FieldReadState<string> {
  const entity = useAuthor();
  return bindFieldToReact<Author, string>({ get: () => entity, read: (a) => a.readIcon() })();
}

/** @emoji 📖 Live {@link Author#readRole}. */
export function useAuthorRole(): FieldReadState<string> {
  const entity = useAuthor();
  return bindFieldToReact<Author, string>({ get: () => entity, read: (a) => a.readRole() })();
}
// #endregion ✍️Author

// #region 💎Quality
const useQualityRenameOp = bindOpToReact<Quality, [string]>((q, k) => q.rename(k));
const useQualityChangeDescriptionOp = bindOpToReact<Quality, [string]>((q, d) => q.changeDescription(d));
const useQualityChangeIconOp = bindOpToReact<Quality, [string]>((q, i) => q.changeIcon(i));
const useQualityAddAttributeOp = bindOpToReact<Quality, [string, string, string]>((q, key, value, definition) => q.addAttribute(key, value, definition));
const useQualityRemoveAttributeOp = bindOpToReact<Quality, [string]>((q, id) => q.removeAttribute(id));
const useQualityRemoveAttributesOp = bindOpToReact<Quality, [readonly string[]]>((q, ids) => q.removeAttributes(ids));

/** @emoji 📖 Live {@link Quality#readKey}. */
export function useQualityKey(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readKey() })();
}

/** @emoji 📖 Live {@link Quality#readValue}. */
export function useQualityValue(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readValue() })();
}

/** @emoji 📖 Live {@link Quality#readUnit}. */
export function useQualityUnit(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readUnit() })();
}

/** @emoji 📖 Live {@link Quality#readDefinition}. */
export function useQualityDefinition(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readDefinition() })();
}

/** @emoji 📖 Live {@link Quality#readDescription}. */
export function useQualityDescription(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readDescription() })();
}

/** @emoji 📖 Live {@link Quality#readIcon}. */
export function useQualityIcon(): FieldReadState<string> {
  const entity = useQuality();
  return bindFieldToReact<Quality, string>({ get: () => entity, read: (q) => q.readIcon() })();
}

/** @emoji 📖 Live {@link Quality#readAttributes}. */
export function useQualityAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useQuality();
  return bindFieldToReact<Quality, readonly Attribute[]>({ get: () => entity, read: (q) => q.readAttributes() })();
}

/** @emoji 📖 Live {@link Quality#readBenchmarks}. */
export function useQualityBenchmarks(): FieldReadState<readonly Benchmark[]> {
  const entity = useQuality();
  return bindFieldToReact<Quality, readonly Benchmark[]>({ get: () => entity, read: (q) => q.readBenchmarks() })();
}

/** @emoji ✍️ {@link Quality#rename}. */
export function useRenameQuality(): readonly [(newKey: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRenameOp(() => e);
}

/** @emoji ✍️ {@link Quality#changeDescription}. */
export function useChangeQualityDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Quality#changeIcon}. */
export function useChangeQualityIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Quality#addAttribute}. */
export function useAddQualityAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttribute}. */
export function useRemoveQualityAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttributes}. */
export function useRemoveQualityAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useQuality();
  return useQualityRemoveAttributesOp(() => e);
}
// #endregion 💎Quality

// #region 🏷️Tag
const useTagRenameOp = bindOpToReact<Tag, [string]>((t, n) => t.rename(n));
const useTagChangeDescriptionOp = bindOpToReact<Tag, [string]>((t, d) => t.changeDescription(d));
const useTagChangeIconOp = bindOpToReact<Tag, [string]>((t, i) => t.changeIcon(i));
const useTagAddAttributeOp = bindOpToReact<Tag, [string, string, string]>((t, key, value, definition) => t.addAttribute(key, value, definition));
const useTagRemoveAttributeOp = bindOpToReact<Tag, [string]>((t, id) => t.removeAttribute(id));
const useTagRemoveAttributesOp = bindOpToReact<Tag, [readonly string[]]>((t, ids) => t.removeAttributes(ids));

/** @emoji 📖 Live {@link Tag#readName}. */
export function useTagName(): FieldReadState<string> {
  const entity = useTag();
  return bindFieldToReact<Tag, string>({ get: () => entity, read: (t) => t.readName() })();
}

/** @emoji 📖 Live {@link Tag#readDescription}. */
export function useTagDescription(): FieldReadState<string> {
  const entity = useTag();
  return bindFieldToReact<Tag, string>({ get: () => entity, read: (t) => t.readDescription() })();
}

/** @emoji 📖 Live {@link Tag#readIcon}. */
export function useTagIcon(): FieldReadState<string> {
  const entity = useTag();
  return bindFieldToReact<Tag, string>({ get: () => entity, read: (t) => t.readIcon() })();
}

/** @emoji 📖 Live {@link Tag#readOrder}. */
export function useTagOrder(): FieldReadState<number | null> {
  const entity = useTag();
  return bindFieldToReact<Tag, number | null>({ get: () => entity, read: (t) => t.readOrder() })();
}

/** @emoji 📖 Live {@link Tag#readAttributes}. */
export function useTagAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useTag();
  return bindFieldToReact<Tag, readonly Attribute[]>({ get: () => entity, read: (t) => t.readAttributes() })();
}

/** @emoji ✍️ {@link Tag#rename}. */
export function useRenameTag(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRenameOp(() => e);
}

/** @emoji ✍️ {@link Tag#changeDescription}. */
export function useChangeTagDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Tag#changeIcon}. */
export function useChangeTagIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Tag#addAttribute}. */
export function useAddTagAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttribute}. */
export function useRemoveTagAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttributes}. */
export function useRemoveTagAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTag();
  return useTagRemoveAttributesOp(() => e);
}
// #endregion 🏷️Tag

// #region 💡Concept
const useConceptRenameOp = bindOpToReact<Concept, [string]>((c, n) => c.rename(n));
const useConceptChangeDescriptionOp = bindOpToReact<Concept, [string]>((c, d) => c.changeDescription(d));
const useConceptChangeIconOp = bindOpToReact<Concept, [string]>((c, i) => c.changeIcon(i));
const useConceptAddAttributeOp = bindOpToReact<Concept, [string, string, string]>((c, key, value, definition) => c.addAttribute(key, value, definition));
const useConceptRemoveAttributeOp = bindOpToReact<Concept, [string]>((c, id) => c.removeAttribute(id));
const useConceptRemoveAttributesOp = bindOpToReact<Concept, [readonly string[]]>((c, ids) => c.removeAttributes(ids));

/** @emoji 📖 Live {@link Concept#readName}. */
export function useConceptName(): FieldReadState<string> {
  const entity = useConcept();
  return bindFieldToReact<Concept, string>({ get: () => entity, read: (c) => c.readName() })();
}

/** @emoji 📖 Live {@link Concept#readDescription}. */
export function useConceptDescription(): FieldReadState<string> {
  const entity = useConcept();
  return bindFieldToReact<Concept, string>({ get: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Concept#readIcon}. */
export function useConceptIcon(): FieldReadState<string> {
  const entity = useConcept();
  return bindFieldToReact<Concept, string>({ get: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Live {@link Concept#readOrder}. */
export function useConceptOrder(): FieldReadState<number | null> {
  const entity = useConcept();
  return bindFieldToReact<Concept, number | null>({ get: () => entity, read: (c) => c.readOrder() })();
}

/** @emoji 📖 Live {@link Concept#readAttributes}. */
export function useConceptAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConcept();
  return bindFieldToReact<Concept, readonly Attribute[]>({ get: () => entity, read: (c) => c.readAttributes() })();
}

/** @emoji ✍️ {@link Concept#rename}. */
export function useRenameConcept(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRenameOp(() => e);
}

/** @emoji ✍️ {@link Concept#changeDescription}. */
export function useChangeConceptDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Concept#changeIcon}. */
export function useChangeConceptIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Concept#addAttribute}. */
export function useAddConceptAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttribute}. */
export function useRemoveConceptAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttributes}. */
export function useRemoveConceptAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useConcept();
  return useConceptRemoveAttributesOp(() => e);
}
// #endregion 💡Concept

// #region 🎨Representation
/** @emoji 📖 Live {@link Representation#readUrl}. */
export function useRepresentationUrl(): FieldReadState<string> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, string>({ get: () => entity, read: (r) => r.readUrl() })();
}

/** @emoji 📖 Live {@link Representation#readDescription}. */
export function useRepresentationDescription(): FieldReadState<string> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, string>({ get: () => entity, read: (r) => r.readDescription() })();
}

/** @emoji 📖 Live {@link Representation#readTagIds} (plan “tags”). */
export function useRepresentationTags(): FieldReadState<readonly string[]> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, readonly string[]>({ get: () => entity, read: (r) => r.readTagIds() })();
}

/** @emoji 📖 Live {@link Representation#readQualityIds}; schema has {@code qualities}, not LOD. */
export function useRepresentationLod(): FieldReadState<readonly string[]> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, readonly string[]>({ get: () => entity, read: (r) => r.readQualityIds() })();
}

/** @emoji 📖 Live {@link Representation#readAttributes}. */
export function useRepresentationAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, readonly Attribute[]>({ get: () => entity, read: (r) => r.readAttributes() })();
}

/** @emoji 📖 Live {@link Representation#readFileId}. */
export function useRepresentationFileId(): FieldReadState<string> {
  const entity = useRepresentation();
  return bindFieldToReact<Representation, string>({ get: () => entity, read: (r) => r.readFileId() })();
}
// #endregion 🎨Representation

// #region 🧩Piece
/** @emoji 📖 Live {@link Piece#readName}. */
export function usePieceName(): FieldReadState<string> {
  const entity = usePiece();
  return bindFieldToReact<Piece, string>({ get: () => entity, read: (p) => p.readName() })();
}

/** @emoji 📖 Live {@link Piece#readDescription}. */
export function usePieceDescription(): FieldReadState<string> {
  const entity = usePiece();
  return bindFieldToReact<Piece, string>({ get: () => entity, read: (p) => p.readDescription() })();
}

/** @emoji 📖 Live {@link Piece#readIcon}. */
export function usePieceIcon(): FieldReadState<string> {
  const entity = usePiece();
  return bindFieldToReact<Piece, string>({ get: () => entity, read: (p) => p.readIcon() })();
}

/** @emoji 📖 Live {@link Piece#readScale}. */
export function usePieceScale(): FieldReadState<number | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, number | null>({ get: () => entity, read: (p) => p.readScale() })();
}

/** @emoji 📖 Live {@link Piece#readPosition}. */
export function usePiecePosition(): FieldReadState<Position | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Position | null>({ get: () => entity, read: (p) => p.readPosition() })();
}

/** @emoji 📖 Live {@link Piece#readFlatPosition}. */
export function usePieceFlatPosition(): FieldReadState<Position | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Position | null>({ get: () => entity, read: (p) => p.readFlatPosition() })();
}

/** @emoji 📖 Live {@link Piece#readPlane}. */
export function usePiecePlane(): FieldReadState<Plane | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Plane | null>({ get: () => entity, read: (p) => p.readPlane() })();
}

/** @emoji 📖 Live {@link Piece#readCenter}. */
export function usePieceCenter(): FieldReadState<Coordinate | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Coordinate | null>({ get: () => entity, read: (p) => p.readCenter() })();
}

/** @emoji 📖 Live {@link Piece#readFlatPlane}. */
export function usePieceFlatPlane(): FieldReadState<Plane | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Plane | null>({ get: () => entity, read: (p) => p.readFlatPlane() })();
}

/** @emoji 📖 Live {@link Piece#readFlatCenter}. */
export function usePieceFlatCenter(): FieldReadState<Coordinate | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, Coordinate | null>({ get: () => entity, read: (p) => p.readFlatCenter() })();
}

/** @emoji 📖 Live {@link Piece#readBlueprint}. */
export function usePieceBlueprint(): FieldReadState<PieceBlueprint | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, PieceBlueprint | null>({ get: () => entity, read: (p) => p.readBlueprint() })();
}

/** @emoji 📖 Live {@link Piece#readAttributes}. */
export function usePieceAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = usePiece();
  return bindFieldToReact<Piece, readonly Attribute[]>({ get: () => entity, read: (p) => p.readAttributes() })();
}

/** @emoji 📖 Live {@link Piece#readConnectionKind}. */
export function usePieceConnectionKind(): FieldReadState<"FIXED" | "CONNECTED" | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, "FIXED" | "CONNECTED" | null>({ get: () => entity, read: (p) => p.readConnectionKind() })();
}

/** @emoji 📖 Live {@link Piece#readParentPieceId}. */
export function usePieceParentPieceId(): FieldReadState<string | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, string | null>({ get: () => entity, read: (p) => p.readParentPieceId() })();
}

/** @emoji 📖 Live {@link Piece#readParentConnectionId}. */
export function usePieceParentConnectionId(): FieldReadState<string | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, string | null>({ get: () => entity, read: (p) => p.readParentConnectionId() })();
}

/** @emoji 📖 Live {@link Piece#readChildPieceIds}. */
export function usePieceChildPieceIds(): FieldReadState<readonly string[]> {
  const entity = usePiece();
  return bindFieldToReact<Piece, readonly string[]>({ get: () => entity, read: (p) => p.readChildPieceIds() })();
}

/** @emoji 📖 Live {@link Piece#readChildConnectionIds}. */
export function usePieceChildConnectionIds(): FieldReadState<readonly string[]> {
  const entity = usePiece();
  return bindFieldToReact<Piece, readonly string[]>({ get: () => entity, read: (p) => p.readChildConnectionIds() })();
}

/** @emoji 📖 Live {@link Piece#readDepth}. */
export function usePieceDepth(): FieldReadState<number | null> {
  const entity = usePiece();
  return bindFieldToReact<Piece, number | null>({ get: () => entity, read: (p) => p.readDepth() })();
}

const usePieceRenameOp = bindOpToReact<Piece, [string]>((p, n) => p.rename(n));
const usePieceChangeDescriptionOp = bindOpToReact<Piece, [string]>((p, d) => p.changeDescription(d));
const usePieceDragOp = bindOpToReact<Piece, [OffsetInput]>((p, o) => p.drag(o));
const usePieceMoveOp = bindOpToReact<Piece, [PositionInput]>((p, pos) => p.move(pos));
const usePieceFixOp = bindOpToReact<Piece, []>((p) => p.fix());
const usePieceChangeBlueprintOp = bindOpToReact<Piece, [string]>((p, id) => p.changeBlueprint(id));
const usePieceAddAttributeOp = bindOpToReact<Piece, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePieceRemoveAttributeOp = bindOpToReact<Piece, [string]>((p, id) => p.removeAttribute(id));
const usePieceRemoveAttributesOp = bindOpToReact<Piece, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji ✍️ {@link Piece#rename} bound to {@link PieceContext}. */
export function useRenamePiece(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRenameOp(() => e);
}

/** @emoji ✍️ {@link Piece#changeDescription}. */
export function useChangePieceDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Piece#drag}. */
export function useDragPiece(): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceDragOp(() => e);
}

/** @emoji ✍️ {@link Piece#move}. */
export function useMovePiece(): readonly [(position: PositionInput) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceMoveOp(() => e);
}

/** @emoji ✍️ {@link Piece#fix}. */
export function useFixPiece(): readonly [() => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceFixOp(() => e);
}

/** @emoji ✍️ {@link Piece#changeBlueprint}. */
export function useChangePieceBlueprint(): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceChangeBlueprintOp(() => e);
}

/** @emoji ✍️ {@link Piece#addAttribute}. */
export function useAddPieceAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttribute}. */
export function useRemovePieceAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttributes}. */
export function useRemovePieceAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePiece();
  return usePieceRemoveAttributesOp(() => e);
}
// #endregion 🧩Piece

// #region 🪢Pieces
/**
 * @emoji 🪝 Binds {@link PiecesOperations} batch mutations (not an {@link } — no cached kit state on the handle).
 * @typeParam Args — forwarded to the underlying {@link PiecesOperations} method after the ops handle.
 */
function bindPiecesOperationsOpToReact<Args extends unknown[]>(impl: (ops: PiecesOperations, ...args: Args) => Promise<SetResult>): (getOps: () => PiecesOperations | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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
            error: { kind: "Disposed", message: "No pieces batch scope (empty ids or missing kit).", field: undefined, entity: undefined },
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

const usePiecesDragOp = bindPiecesOperationsOpToReact((ops, o: OffsetInput) => ops.drag(o));
const usePiecesMoveOp = bindPiecesOperationsOpToReact((ops, o: OffsetInput) => ops.move(o));
const usePiecesFixOp = bindPiecesOperationsOpToReact((ops) => ops.fix());
const usePiecesChangeBlueprintOp = bindPiecesOperationsOpToReact((ops, id: string) => ops.changeBlueprint(id));

/** @emoji ✍️ {@link PiecesOperations#drag} for {@code design.pieces(ids)}. */
export function useDragPieces(designId: string, pieceIds: readonly string[]): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)), [kit, designId, pieceIds]);
  return usePiecesDragOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#move}. */
export function useMovePieces(designId: string, pieceIds: readonly string[]): readonly [(offset: OffsetInput) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)), [kit, designId, pieceIds]);
  return usePiecesMoveOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#fix}. */
export function useFixPieces(designId: string, pieceIds: readonly string[]): readonly [() => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)), [kit, designId, pieceIds]);
  return usePiecesFixOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#changeBlueprint}. */
export function useChangePiecesBlueprint(designId: string, pieceIds: readonly string[]): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const kit = useKit();
  const getOps = React.useCallback(() => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)), [kit, designId, pieceIds]);
  return usePiecesChangeBlueprintOp(getOps);
}
// #endregion 🪢Pieces

// #region ⛓️Connection
/** @emoji 📖 Live {@link Connection#readGap}. */
export function useConnectionGap(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readGap() })();
}

/** @emoji 📖 Live {@link Connection#readShift}. */
export function useConnectionShift(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readShift() })();
}

/** @emoji 📖 Live {@link Connection#readRise}. */
export function useConnectionRise(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readRise() })();
}

/** @emoji 📖 Live {@link Connection#readRotation}. */
export function useConnectionRotation(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readRotation() })();
}

/** @emoji 📖 Live {@link Connection#readTurn}. */
export function useConnectionTurn(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readTurn() })();
}

/** @emoji 📖 Live {@link Connection#readTilt}. */
export function useConnectionTilt(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readTilt() })();
}

/** @emoji 📖 Live {@link Connection#readU}. */
export function useConnectionU(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readU() })();
}

/** @emoji 📖 Live {@link Connection#readV}. */
export function useConnectionV(): FieldReadState<number | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, number | null>({ get: () => entity, read: (c) => c.readV() })();
}

/** @emoji 📖 Live {@link Connection#readConnected}. */
export function useConnectionConnected(): FieldReadState<ConnectionSide | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, ConnectionSide | null>({ get: () => entity, read: (c) => c.readConnected() })();
}

/** @emoji 📖 Live {@link Connection#readConnecting}. */
export function useConnectionConnecting(): FieldReadState<ConnectionSide | null> {
  const entity = useConnection();
  return bindFieldToReact<Connection, ConnectionSide | null>({ get: () => entity, read: (c) => c.readConnecting() })();
}

/** @emoji 📖 Live {@link Connection#readName}. */
export function useConnectionName(): FieldReadState<string> {
  const entity = useConnection();
  return bindFieldToReact<Connection, string>({ get: () => entity, read: (c) => c.readName() })();
}

/** @emoji 📖 Live {@link Connection#readDescription}. */
export function useConnectionDescription(): FieldReadState<string> {
  const entity = useConnection();
  return bindFieldToReact<Connection, string>({ get: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Connection#readIcon}. */
export function useConnectionIcon(): FieldReadState<string> {
  const entity = useConnection();
  return bindFieldToReact<Connection, string>({ get: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Live {@link Connection#readAttributes}. */
export function useConnectionAttributes(): FieldReadState<readonly Attribute[]> {
  const entity = useConnection();
  return bindFieldToReact<Connection, readonly Attribute[]>({ get: () => entity, read: (c) => c.readAttributes() })();
}
// #endregion ⛓️Connection

// #region ⚛️Embedded tests
// @emoji 🧹 Legacy InMemoryKitStore embedded block removed during single-source Kit migration; restore with GraphQL Kit stubs only.
// #endregion ⚛️Embedded tests


// #region 🧪Vitest
if (import.meta.vitest) {
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");
  const { describe, expect, it } = import.meta.vitest;
  const reactSrcPath = fileURLToPath(new URL("./index.tsx", import.meta.url));
  const reactSrc = readFileSync(reactSrcPath, "utf8");
  describe("semio/react kit binders", () => {
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
      /\bKitRuntime\b/,
      /\buseKitRuntimeSafe\s*\(/,
      /\buseKitScope\s*\(/,
      /\bKitScope\b/,
      /\bKitShellScopeProvider\b/,
      /\buseSyncExternalStore\b/,
    ];
    it("react index has no banned substrings as live code calls", () => {
      for (const re of mustNotMatchCode) {
        expect.soft(reactSrc.match(re)).toBeNull();
      }
    });
  });
}
// #endregion 🧪Vitest
