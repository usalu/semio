// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link Entity} reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
export type { JsonObject, JsonValue } from "@semio/js";
export {
  Author,
  Concept,
  Connection,
  Connector,
  createKitStoreWorker,
  defineField,
  defineFields,
  defineOperation,
  defineOperations,
  Design,
  EventBus,
  Family,
  FileEntity,
  FolderEntity,
  GroupEntity,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  Kit,
  LayerEntity,
  openKit,
  Piece,
  PiecesOperations,
  Port,
  PropEntity,
  Quality,
  Representation,
  StatEntity,
  Tag,
  Type,
} from "@semio/js";
export type {
  AttributeWire,
  BenchmarkWire,
  CameraWire,
  ChangeId,
  CoordinateInputWire,
  FieldSpec,
  KitBootstrapJson,
  KitOpenOptions,
  KitReadPoint,
  OffsetInputWire,
  OperationSpec,
  PlaneInputWire,
  PlaneWire,
  PointWire,
  PositionInputWire,
  PositionWire,
  SetError,
  SetErrorKind,
  SetResult,
  SideWire,
  Unsubscribe,
  VectorInputWire,
  VectorWire,
} from "@semio/js";
export { kitReadPointKey, theKitReadPoint } from "@semio/js";
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type { FieldSpec, KitReadPoint, SetError, SetResult } from "@semio/js";
import {
  Author,
  Concept,
  Connection,
  Connector,
  defineField,
  Design,
  Entity,
  Kit,
  Piece,
  Port,
  Quality,
  Tag,
  Type,
  theKitReadPoint,
} from "@semio/js";
import type { ReactNode } from "react";
import * as React from "react";
// #endregion ⚛️Imports

// #region 📐UiConstants
/** @emoji 📐 Icon column width shared by sketchpad-era layouts (UI-only, not domain). */
export const ICON_WIDTH = 24;

/** @emoji 📐 Numeric tolerance for float compares in canvas-era helpers (UI-only). */
export const TOLERANCE = 1e-6;

/** @emoji 🆔 Opaque id helper for client-only rows (crypto UUID when available). */
export function id(): string {
  const c = globalThis.crypto;
  if (c && typeof c.randomUUID === "function") return c.randomUUID();
  return `tmp-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 10)}`;
}
// #endregion 📐UiConstants

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
  /** @emoji 🪝 Entity source; re-invoked each render — keep stable via {@link React#useCallback}. */
  getEntity: () => E | null;
}>;

/**
 * @emoji 🪝 Binds one async entity read to React state; optional bus kind narrows refresh fan-in (no `useSyncExternalStore`).
 * @typeParam E — Concrete {@link Entity} subclass anchor.
 * @typeParam T — Parsed field value.
 */
export function bindFieldToReact<E extends Entity, T>(opts: FieldBindOptions<E, T>): () => FieldReadState<T> {
  const { read, eventKind, getEntity } = opts;
  return function useEntityField(): FieldReadState<T> {
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

/**
 * @emoji 🪝 Same as {@link bindFieldToReact} but wires {@link defineField} so callers share {@link FieldSpec} with tooling/docs.
 * @typeParam E — Concrete {@link Entity} subclass anchor.
 * @typeParam T — Parsed field value.
 */
export function bindDefinedFieldToReact<E extends Entity, T>(
  spec: FieldSpec<T>,
  pathInKit: (self: E) => string,
  opts?: { readonly eventKind?: string },
): (getEntity: () => E | null) => () => FieldReadState<T> {
  const eventKind = opts?.eventKind;
  return (getEntity: () => E | null) =>
    function useDefinedEntityField(): FieldReadState<T> {
    const entity = getEntity();
    const reader = React.useMemo(() => (entity == null ? null : defineField(entity, spec, pathInKit)), [entity, spec, pathInKit]);
    const [value, setValue] = React.useState<T | undefined>(undefined);
    const [loading, setLoading] = React.useState(false);
    const [error, setError] = React.useState<unknown>(undefined);
    const entityRef = React.useRef(entity);
    entityRef.current = entity;

    const refresh = React.useCallback(async () => {
      if (reader == null) {
        setValue(undefined);
        setError(undefined);
        setLoading(false);
        return;
      }
      setLoading(true);
      setError(undefined);
      try {
        setValue(await reader());
      } catch (err) {
        setError(err);
      } finally {
        setLoading(false);
      }
    }, [reader]);

    React.useEffect(() => {
      void refresh();
    }, [refresh]);

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
export type OperationStatus =
  | { readonly kind: "idle" }
  | { readonly kind: "pending" }
  | { readonly kind: "settled"; readonly result: SetResult };

/**
 * @emoji 🗺️ Maps {@link SetErrorKind#NameTooLong} to a fixed max-length message; otherwise returns {@link SetError#message}.
 * @param maxChars — Upper bound communicated to the user (schema limit or UI policy).
 */
export function mapTooLong(err: SetError, maxChars: number): string {
  if (err.kind === "NameTooLong") return `Name must be at most ${maxChars} characters.`;
  return err.message;
}

/**
 * @emoji 🪝 Binds an entity operation to `[run, status]`; `run` reads latest entity via {@code getEntity} ref (no sync external store).
 * @typeParam E — Concrete {@link Entity} subclass anchor.
 * @typeParam Args — Operation arguments after the entity receiver.
 */
export function bindOpToReact<E extends Entity, Args extends unknown[] = []>(
  impl: (entity: E, ...args: Args) => Promise<SetResult>,
): (getEntity: () => E | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
  return function useEntityOp(getEntity: () => E | null): readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
    const getRef = React.useRef(getEntity);
    getRef.current = getEntity;
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

// #region 🧭KitRuntimeContext
export type KitRuntimeValue = Readonly<{
  kit: Kit;
  readPoint: KitReadPoint;
  setReadPoint: (next: KitReadPoint) => void;
}>;

const KitRuntimeContext = React.createContext<KitRuntimeValue | null>(null);

export type KitContextProviderProps = Readonly<{
  kit: Kit;
  initialReadPoint?: KitReadPoint;
  children: ReactNode;
}>;

/** @emoji 🧭 Provides {@link Kit} + materialization {@link KitReadPoint} for descendant hooks. */
export function KitContextProvider(props: KitContextProviderProps): React.ReactElement {
  const [readPoint, setReadPointState] = React.useState<KitReadPoint>(props.initialReadPoint ?? theKitReadPoint);
  React.useEffect(() => {
    props.kit.setReadPoint(readPoint);
  }, [props.kit, readPoint]);
  const setReadPoint = React.useCallback((next: KitReadPoint) => {
    setReadPointState(next);
  }, []);
  const value = React.useMemo<KitRuntimeValue>(
    () => ({ kit: props.kit, readPoint, setReadPoint }),
    [props.kit, readPoint, setReadPoint],
  );
  return React.createElement(KitRuntimeContext.Provider, { value }, props.children);
}

/** @emoji 🧭 Requires {@link KitContextProvider}; throws when missing (fail-fast for app wiring). */
export function useKit(): KitRuntimeValue {
  const v = React.useContext(KitRuntimeContext);
  if (v == null) throw new Error("semio/react: useKit requires <KitContextProvider>.");
  return v;
}

/** @emoji 🧭 Nullable kit runtime for optional UI (Storybook / diagnostics). */
export function useKitOrNull(): KitRuntimeValue | null {
  return React.useContext(KitRuntimeContext);
}
// #endregion 🧭KitRuntimeContext

// #region 🪢SchemaEntityContext
/** @emoji 🪢 Token carried through nested entity providers for form controllers (not authoritative kit state). */
export type SchemaContext = Readonly<{ entityKind: string; entityId: string }>;

/** @emoji 🪢 React context for {@link SchemaContext} (replaces legacy `SchemaScopeContext`). */
export const SchemaEntityContext = React.createContext<SchemaContext | null>(null);

/** @emoji 🪢 Builds a {@link SchemaContext} value for a single entity id. */
export function schemaContextForEntityId(entityKind: string, entityId: string): SchemaContext {
  return { entityKind, entityId };
}
// #endregion 🪢SchemaEntityContext

// #region 🪢ShellContext
export type KitShellContextValue = Readonly<{ id: string }>;

const KitShellContext = React.createContext<KitShellContextValue | null>(null);

/** @emoji 🪟 Pins active kit tab / shell id for hooks that resolve ids without prop drilling. */
export function KitShellContextProvider(props: { id: string; children: ReactNode }): React.ReactElement {
  return React.createElement(KitShellContext.Provider, { value: { id: props.id } }, props.children);
}

export function useKitShellContext(): KitShellContextValue | null {
  return React.useContext(KitShellContext);
}

/** @emoji 🪟 Alias for {@link KitShellContext} naming parity with older `KitScopeContext`. */
export { KitShellContext as KitContextShellContext };
// #endregion 🪢ShellContext

// #region 🪢EntityContexts
export type DesignEntityContextValue = Readonly<{ designId: string }>;
const DesignEntityContext = React.createContext<DesignEntityContextValue | null>(null);
export function DesignEntityContextProvider(props: { designId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(DesignEntityContext.Provider, { value: { designId: props.designId } }, props.children);
}
export function useDesignEntity(): Design | null {
  const { kit } = useKit();
  const ctx = React.useContext(DesignEntityContext);
  return ctx == null ? null : kit.design(ctx.designId);
}

export type PieceEntityContextValue = Readonly<{ designId: string; pieceId: string }>;
const PieceEntityContext = React.createContext<PieceEntityContextValue | null>(null);
export function PieceEntityContextProvider(props: PieceEntityContextValue & { children: ReactNode }): React.ReactElement {
  return React.createElement(PieceEntityContext.Provider, { value: { designId: props.designId, pieceId: props.pieceId } }, props.children);
}
export function usePieceEntity(): Piece | null {
  const { kit } = useKit();
  const ctx = React.useContext(PieceEntityContext);
  return ctx == null ? null : kit.design(ctx.designId).piece(ctx.pieceId);
}

export type TypeEntityContextValue = Readonly<{ typeId: string }>;
const TypeEntityContext = React.createContext<TypeEntityContextValue | null>(null);
export function TypeEntityContextProvider(props: { typeId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(TypeEntityContext.Provider, { value: { typeId: props.typeId } }, props.children);
}
export function useTypeEntity(): Type | null {
  const { kit } = useKit();
  const ctx = React.useContext(TypeEntityContext);
  return ctx == null ? null : kit.type(ctx.typeId);
}

export type ConnectionEntityContextValue = Readonly<{ designId: string; connectionId: string }>;
const ConnectionEntityContext = React.createContext<ConnectionEntityContextValue | null>(null);
export function ConnectionEntityContextProvider(props: ConnectionEntityContextValue & { children: ReactNode }): React.ReactElement {
  return React.createElement(ConnectionEntityContext.Provider, { value: { designId: props.designId, connectionId: props.connectionId } }, props.children);
}
export function useConnectionEntity(): Connection | null {
  const { kit } = useKit();
  const ctx = React.useContext(ConnectionEntityContext);
  return ctx == null ? null : kit.design(ctx.designId).connection(ctx.connectionId);
}

export type PortEntityContextValue = Readonly<{ typeId: string; portId: string }>;
const PortEntityContext = React.createContext<PortEntityContextValue | null>(null);
export function PortEntityContextProvider(props: PortEntityContextValue & { children: ReactNode }): React.ReactElement {
  return React.createElement(PortEntityContext.Provider, { value: { typeId: props.typeId, portId: props.portId } }, props.children);
}
export function usePortEntity(): Port | null {
  const { kit } = useKit();
  const ctx = React.useContext(PortEntityContext);
  return ctx == null ? null : kit.type(ctx.typeId).port(ctx.portId);
}

export type ConnectorEntityContextValue = Readonly<{ typeId: string; connectorId: string }>;
const ConnectorEntityContext = React.createContext<ConnectorEntityContextValue | null>(null);
export function ConnectorEntityContextProvider(props: ConnectorEntityContextValue & { children: ReactNode }): React.ReactElement {
  return React.createElement(ConnectorEntityContext.Provider, { value: { typeId: props.typeId, connectorId: props.connectorId } }, props.children);
}
export function useConnectorEntity(): Connector | null {
  const { kit } = useKit();
  const ctx = React.useContext(ConnectorEntityContext);
  return ctx == null ? null : kit.type(ctx.typeId).connector(ctx.connectorId);
}

export type QualityEntityContextValue = Readonly<{ qualityId: string }>;
const QualityEntityContext = React.createContext<QualityEntityContextValue | null>(null);
export function QualityEntityContextProvider(props: { qualityId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(QualityEntityContext.Provider, { value: { qualityId: props.qualityId } }, props.children);
}
export function useQualityEntity(): Quality | null {
  const { kit } = useKit();
  const ctx = React.useContext(QualityEntityContext);
  return ctx == null ? null : kit.quality(ctx.qualityId);
}

export type TagEntityContextValue = Readonly<{ tagId: string }>;
const TagEntityContext = React.createContext<TagEntityContextValue | null>(null);
export function TagEntityContextProvider(props: { tagId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(TagEntityContext.Provider, { value: { tagId: props.tagId } }, props.children);
}
export function useTagEntity(): Tag | null {
  const { kit } = useKit();
  const ctx = React.useContext(TagEntityContext);
  return ctx == null ? null : kit.tag(ctx.tagId);
}

export type ConceptEntityContextValue = Readonly<{ conceptId: string }>;
const ConceptEntityContext = React.createContext<ConceptEntityContextValue | null>(null);
export function ConceptEntityContextProvider(props: { conceptId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(ConceptEntityContext.Provider, { value: { conceptId: props.conceptId } }, props.children);
}
export function useConceptEntity(): Concept | null {
  const { kit } = useKit();
  const ctx = React.useContext(ConceptEntityContext);
  return ctx == null ? null : kit.concept(ctx.conceptId);
}

export type AuthorEntityContextValue = Readonly<{ authorId: string }>;
const AuthorEntityContext = React.createContext<AuthorEntityContextValue | null>(null);
export function AuthorEntityContextProvider(props: { authorId: string; children: ReactNode }): React.ReactElement {
  return React.createElement(AuthorEntityContext.Provider, { value: { authorId: props.authorId } }, props.children);
}
export function useAuthorEntity(): Author | null {
  const { kit } = useKit();
  const ctx = React.useContext(AuthorEntityContext);
  return ctx == null ? null : kit.author(ctx.authorId);
}
// #endregion 🪢EntityContexts

// #region 🧪PresetFieldHooks
/** @emoji 🧪 Schema-1:1 connection gap read bound for apps that already provide {@link ConnectionEntityContext}. */
export function useConnectionGapField(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readGap() })();
}

/** @emoji 🧪 Schema-1:1 design name read (mount + manual refresh; subscribe via custom `eventKind` when known). */
export function useDesignNameField(): FieldReadState<string> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, string>({ getEntity: () => entity, read: (d) => d.readName() })();
}
// #endregion 🧪PresetFieldHooks

// #region 🧪Vitest
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  describe("semio/react kit binders", () => {
    it("mapTooLong surfaces max length for NameTooLong", () => {
      const msg = mapTooLong({ kind: "NameTooLong", message: "ignored", field: "name" }, 42);
      expect(msg).toContain("42");
    });
  });
}
// #endregion 🧪Vitest
