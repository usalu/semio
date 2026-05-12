// #region ⚛️Header
// Standalone React hooks for semio: thin adapter over stateless {@link Kit} + {@link Entity} reads/writes.
// #endregion ⚛️Header

// #region 🧷JsReexports
// Value/type re-exports follow the local `@semio/js` imports below (single binding per symbol).
// #endregion 🧷JsReexports

// #region ⚛️Imports
import type {
  AttributeWire,
  BenchmarkWire,
  BackboneConfig,
  BackboneStatusDto,
  ChangeId,
  ChangeKitCommand,
  ConflictResolution,
  ConnectionDiff,
  ConnectionIdDto,
  ConnectionPlain,
  ConnectionSideWire,
  CoordinateWire,
  DesignDiff,
  DesignIdDto,
  DesignMetadataDto,
  DesignPlain,
  DesignShallow,
  FieldSpec,
  JsonObject,
  JsonValue,
  KitBinaryStore,
  KitChildEntityKind,
  KitConflict,
  KitDesignReadKind,
  KitEvent,
  KitFileState,
  KitFolderAdapter,
  KitFullDto,
  KitJsonFileAdapter,
  KitJsonTreeDto,
  KitHostStore,
  KitHostStoreSnapshot,
  KitLike,
  KitReadPoint,
  KitShallowListKind,
  KitStoreClient,
  KitStoreReadSnap,
  KitViewCatalogKey,
  KitWriteScope,
  OffsetInputWire,
  PieceBlueprintWire,
  PieceDiff,
  PieceIdDto,
  PiecePlain,
  PiecePlacementRowDto,
  PlanePlain,
  PlaneWire,
  PositionInputWire,
  PositionWire,
  SetError,
  SetResult,
  TypeDiff,
  TypeIdDto,
  TypeMetadataDto,
  TypePlain,
  TypeShallow,
  WriteStatus,
} from "@semio/js";
import {
  Author,
  Concept,
  Connection,
  Connector,
  CommandBuilder,
  createKitStoreWorker,
  defineField,
  defineFields,
  defineOperation,
  defineOperations,
  DESIGN_ARTIFACT_FIELD_SPECS,
  DESIGN_OPERATION_SPECS,
  Design,
  Entity,
  EventBus,
  Family,
  FileEntity,
  FolderEntity,
  GroupEntity,
  KIT_ARTIFACT_FIELD_SPECS,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  KIT_OPERATION_SPECS,
  Kit,
  LayerEntity,
  normalizeKitFullDtoFolderPaths,
  openKit,
  Piece,
  PiecesOperations,
  Port,
  PropEntity,
  Quality,
  Representation,
  StatEntity,
  StoreCommand,
  StoreField,
  Tag,
  Type,
  kitReadPointKey,
  theKitReadPoint,
  WasmGraph,
} from "@semio/js";
import type { ReactNode, SetStateAction } from "react";
import * as React from "react";
// #endregion ⚛️Imports

// #region 🧷JsPublicExports
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
  DESIGN_ARTIFACT_FIELD_SPECS,
  DESIGN_OPERATION_SPECS,
  Design,
  Entity,
  EventBus,
  Family,
  FileEntity,
  FolderEntity,
  GroupEntity,
  KIT_ARTIFACT_FIELD_SPECS,
  KIT_EVENT_STREAM_SUBSCRIPTION,
  KIT_OPERATION_SPECS,
  Kit,
  LayerEntity,
  normalizeKitFullDtoFolderPaths,
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
  kitReadPointKey,
  theKitReadPoint,
};
// #endregion 🧷JsPublicExports

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

export type DefinedFieldBindOptions<E extends Entity, T> = Readonly<{
  spec: FieldSpec<T>;
  pathInKit: (self: E) => string;
  getEntity: () => E | null;
  eventKind?: string;
}>;

/**
 * @emoji 🪝 Same as {@link bindFieldToReact} but wires {@link defineField} so callers share {@link FieldSpec} with tooling/docs.
 * @typeParam E — Concrete {@link Entity} subclass anchor.
 * @typeParam T — Parsed field value.
 */
export function bindDefinedFieldToReact<E extends Entity, T>(opts: DefinedFieldBindOptions<E, T>): () => FieldReadState<T> {
  const { spec, pathInKit, getEntity, eventKind } = opts;
  return function useDefinedEntityField(): FieldReadState<T> {
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

// #region 🪝KitFieldBind
/** @emoji 🪝 Kit-scoped field bind (uses {@link Kit#bus} like {@link bindFieldToReact}). */
export type KitFieldBindOptions<T> = Readonly<{
  read: (kit: Kit) => Promise<T>;
  eventKind?: string;
  getKit: () => Kit | null;
}>;

export function bindKitFieldToReact<T>(opts: KitFieldBindOptions<T>): () => FieldReadState<T> {
  const { read, eventKind, getKit } = opts;
  return function useKitBoundField(): FieldReadState<T> {
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
export function bindKitOpToReact<Args extends unknown[] = []>(
  impl: (kit: Kit, ...args: Args) => Promise<SetResult>,
): (getKit: () => Kit | null) => readonly [(...args: Args) => Promise<SetResult>, OperationStatus] {
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

// #region 🎭Contexts
// #region 🎒KitRuntime
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
// #endregion 🎒KitRuntime

// #region 📐DesignEntity
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
// #endregion 📐DesignEntity

// #endregion 🎭Contexts

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

export type RepresentationEntityContextValue = Readonly<{ typeId: string; representationId: string }>;
const RepresentationEntityContext = React.createContext<RepresentationEntityContextValue | null>(null);
export function RepresentationEntityContextProvider(props: RepresentationEntityContextValue & { children: ReactNode }): React.ReactElement {
  return React.createElement(RepresentationEntityContext.Provider, { value: { typeId: props.typeId, representationId: props.representationId } }, props.children);
}
export function useRepresentationEntity(): Representation | null {
  const { kit } = useKit();
  const ctx = React.useContext(RepresentationEntityContext);
  return ctx == null ? null : kit.type(ctx.typeId).representation(ctx.representationId);
}
// #endregion 🪢EntityContexts

// #region 🪝HooksKit
// #region 📖KitReads
/** @emoji 📖 Live {@link Kit#readName} + {@code kitRenamed}. */
export function useKitNameField(): FieldReadState<string> {
  const { kit } = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readName(), eventKind: "kitRenamed" })();
}

/** @emoji 📖 Live {@link Kit#readDescription} + {@code changedDescription}. */
export function useKitDescriptionField(): FieldReadState<string> {
  const { kit } = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readDescription(), eventKind: "changedDescription" })();
}

/** @emoji 📖 Live {@link Kit#readId}. */
export function useKitIdField(): FieldReadState<string> {
  const { kit } = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readId() })();
}

/** @emoji 📖 Live {@link Kit#readIcon}. */
export function useKitIconField(): FieldReadState<string> {
  const { kit } = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readIcon() })();
}

/** @emoji 📖 Live {@link Kit#readImage}. */
export function useKitImageField(): FieldReadState<string> {
  const { kit } = useKit();
  return bindKitFieldToReact<string>({ getKit: () => kit, read: (k) => k.readImage() })();
}

/** @emoji 📖 Live {@link Kit#readTypeIds}. */
export function useKitTypeIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readTypeIds() })();
}

/** @emoji 📖 Live {@link Kit#readDesignIds}. */
export function useKitDesignIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readDesignIds() })();
}

/** @emoji 📖 Live {@link Kit#readAuthorIds}. */
export function useKitAuthorIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readAuthorIds() })();
}

/** @emoji 📖 Live {@link Kit#readQualityIds}. */
export function useKitQualityIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readQualityIds() })();
}

/** @emoji 📖 Live {@link Kit#readTagIds}. */
export function useKitTagIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readTagIds() })();
}

/** @emoji 📖 Live {@link Kit#readConceptIds}. */
export function useKitConceptIdsField(): FieldReadState<readonly string[]> {
  const { kit } = useKit();
  return bindKitFieldToReact<readonly string[]>({ getKit: () => kit, read: (k) => k.readConceptIds() })();
}

/** @emoji 🧾 Exposes {@link Kit#ensureChangeId} as a stable callback. */
export function useEnsureKitChangeId(): () => Promise<string> {
  const { kit } = useKit();
  return React.useCallback(() => kit.ensureChangeId(), [kit]);
}
// #endregion 📖KitReads

// #region ✍️KitWrites
/** @emoji ✍️ {@link Kit#rename}. */
export function useRenameKit(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, newName) => k.rename(newName))(() => kit);
}

/** @emoji ✍️ {@link Kit#changeDescription}. */
export function useChangeKitDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, d) => k.changeDescription(d))(() => kit);
}

/** @emoji ✍️ {@link Kit#createTag}. */
export function useCreateTag(): readonly [
  (name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const { kit } = useKit();
  return bindKitOpToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) =>
    k.createTag(n, d, i, o),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTag}. */
export function useDeleteTag(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.deleteTag(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTags}. */
export function useDeleteTags(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[readonly string[]]>((k, ids) => k.deleteTags(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createConcept}. */
export function useCreateConcept(): readonly [
  (name: string, description?: string | null, icon?: string | null, order?: number | null) => Promise<SetResult>,
  OperationStatus,
] {
  const { kit } = useKit();
  return bindKitOpToReact<[string, string | null | undefined, string | null | undefined, number | null | undefined]>((k, n, d, i, o) =>
    k.createConcept(n, d, i, o),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcept}. */
export function useDeleteConcept(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.deleteConcept(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteConcepts}. */
export function useDeleteConcepts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[readonly string[]]>((k, ids) => k.deleteConcepts(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createQuality}. */
export function useCreateQuality(): readonly [
  (
    key: string,
    value?: string | null,
    unit?: string | null,
    definition?: string | null,
    description?: string | null,
    icon?: string | null,
  ) => Promise<SetResult>,
  OperationStatus,
] {
  const { kit } = useKit();
  return bindKitOpToReact<
    [string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]
  >((k, key, value, unit, definition, description, icon) => k.createQuality(key, value, unit, definition, description, icon))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQuality}. */
export function useDeleteQuality(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.deleteQuality(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteQualities}. */
export function useDeleteQualities(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[readonly string[]]>((k, ids) => k.deleteQualities(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createType}. */
export function useCreateType(): readonly [
  (name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>,
  OperationStatus,
] {
  const { kit } = useKit();
  return bindKitOpToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) =>
    k.createType(n, d, i, im, u),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteType}. */
export function useDeleteType(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.deleteType(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteTypes}. */
export function useDeleteTypes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[readonly string[]]>((k, ids) => k.deleteTypes(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#createDesign}. */
export function useCreateDesign(): readonly [
  (name: string, description?: string | null, icon?: string | null, image?: string | null, unit?: string | null) => Promise<SetResult>,
  OperationStatus,
] {
  const { kit } = useKit();
  return bindKitOpToReact<[string, string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined]>((k, n, d, i, im, u) =>
    k.createDesign(n, d, i, im, u),
  )(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesign}. */
export function useDeleteDesign(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.deleteDesign(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#deleteDesigns}. */
export function useDeleteDesigns(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[readonly string[]]>((k, ids) => k.deleteDesigns(ids))(() => kit);
}

/** @emoji ✍️ {@link Kit#saveChange}. */
export function useSaveKitChange(): readonly [() => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[]>(async (k) => {
    await k.saveChange();
    return { ok: true };
  })(() => kit);
}

/** @emoji ✍️ {@link Kit#createCheckpoint}. */
export function useCreateCheckpoint(): readonly [(message: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, message) => k.createCheckpoint(message))(() => kit);
}

/** @emoji ✍️ {@link Kit#startAlternative}. */
export function useStartAlternative(): readonly [(name?: string | null) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string | null | undefined]>((k, name) => k.startAlternative(name ?? undefined))(() => kit);
}

/** @emoji ✍️ {@link Kit#integrateAlternative}. */
export function useIntegrateAlternative(): readonly [(alternativeId: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, id) => k.integrateAlternative(id))(() => kit);
}

/** @emoji ✍️ {@link Kit#login}. */
export function useLogin(): readonly [(username: string, passwordHash: string, hubUrl?: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string, string, string | undefined]>((k, u, p, h) => k.login(u, p, h))(() => kit);
}

/** @emoji ✍️ {@link Kit#logout}. */
export function useLogout(): readonly [() => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[]>((k) => k.logout())(() => kit);
}

/** @emoji ✍️ {@link Kit#sessionStart}. */
export function useStartSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[]>((k) => k.sessionStart())(() => kit);
}

/** @emoji ✍️ {@link Kit#sessionEnd}. */
export function useEndSession(): readonly [() => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[]>((k) => k.sessionEnd())(() => kit);
}

/** @emoji ✍️ {@link Kit#hydrateKitStoreBundleJson}. */
export function useHydrateKitStoreBundleJson(): readonly [(json: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  return bindKitOpToReact<[string]>((k, json) => k.hydrateKitStoreBundleJson(json))(() => kit);
}
// #endregion ✍️KitWrites
// #endregion 🪝HooksKit

// #region 🪝HooksDesign
// #region 📖DesignReads
/** @emoji 📖 Live {@link Design#readName}. */
export function useDesignNameField(): FieldReadState<string> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, string>({ getEntity: () => entity, read: (d) => d.readName() })();
}

/** @emoji 📖 Live {@link Design#readDescription} + {@code changedDescription}. */
export function useDesignDescriptionField(): FieldReadState<string> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, string>({ getEntity: () => entity, read: (d) => d.readDescription(), eventKind: "changedDescription" })();
}

/** @emoji 📖 Live {@link Design#readPieceIds}. */
export function useDesignPieceIdsField(): FieldReadState<readonly string[]> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, readonly string[]>({ getEntity: () => entity, read: (d) => d.readPieceIds() })();
}

/** @emoji 📖 Live {@link Design#readConnectionIds}. */
export function useDesignConnectionIdsField(): FieldReadState<readonly string[]> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, readonly string[]>({ getEntity: () => entity, read: (d) => d.readConnectionIds() })();
}

/** @emoji 📖 Live {@link Design#readAttributeIds}. */
export function useDesignAttributeIdsField(): FieldReadState<readonly string[]> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, readonly string[]>({ getEntity: () => entity, read: (d) => d.readAttributeIds() })();
}

/** @emoji 📖 Live {@link Design#readQualitySum}. */
export function useDesignQualitySumField(): FieldReadState<number> {
  const entity = useDesignEntity();
  return bindFieldToReact<Design, number>({ getEntity: () => entity, read: (d) => d.readQualitySum() })();
}
// #endregion 📖DesignReads

// #region ✍️DesignWrites
/** @emoji ✍️ {@link Design#rename}. */
export function useRenameDesign(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string]>((d, n) => d.rename(n))(() => entity);
}

/** @emoji ✍️ {@link Design#changeDescription}. */
export function useChangeDesignDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string]>((d, t) => d.changeDescription(t))(() => entity);
}

/** @emoji ✍️ {@link Design#flatten}. */
export function useFlattenDesign(): readonly [() => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, []>((d) => d.flatten())(() => entity);
}

/** @emoji ✍️ {@link Design#addAttribute}. */
export function useAddDesignAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string, string, string]>((d, k, v, def) => d.addAttribute(k, v, def))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttribute}. */
export function useRemoveDesignAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string]>((d, id) => d.removeAttribute(id))(() => entity);
}

/** @emoji ✍️ {@link Design#removeAttributes}. */
export function useRemoveDesignAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [readonly string[]]>((d, ids) => d.removeAttributes(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#addFixedPiece}. */
export function useAddFixedPiece(): readonly [
  (blueprintId: string, position: PositionInputWire, name?: string | null, description?: string | null) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string, PositionInputWire, string | null | undefined, string | null | undefined]>((d, bp, pos, n, desc) =>
    d.addFixedPiece(bp, pos, n, desc),
  )(() => entity);
}

/** @emoji ✍️ {@link Design#addChildPieceWithParentConnection}. */
export function useAddChildPieceWithParentConnection(): readonly [
  (
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    name?: string | null,
    description?: string | null,
    position?: PositionInputWire | null,
    scale?: number | null,
  ) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesignEntity();
  return bindOpToReact<
    Design,
    [string, string, string, string, string | null | undefined, string | null | undefined, PositionInputWire | null | undefined, number | null | undefined]
  >((d, bp, pp, pc, cc, n, desc, pos, sc) => d.addChildPieceWithParentConnection(bp, pp, pc, cc, n, desc, pos, sc))(() => entity);
}

/** @emoji ✍️ {@link Design#addHangingChildPieceWithParentConnection}. */
export function useAddHangingChildPieceWithParentConnection(): readonly [
  (
    blueprintId: string,
    parentPieceId: string,
    parentConnector: string,
    childConnector: string,
    position: PositionInputWire,
    name?: string | null,
    description?: string | null,
    scale?: number | null,
  ) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesignEntity();
  return bindOpToReact<
    Design,
    [string, string, string, string, PositionInputWire, string | null | undefined, string | null | undefined, number | null | undefined]
  >((d, bp, pp, pc, cc, pos, n, desc, sc) => d.addHangingChildPieceWithParentConnection(bp, pp, pc, cc, pos, n, desc, sc))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiece}. */
export function useDeleteDesignPiece(): readonly [(pieceId: string) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [string]>((d, id) => d.deletePiece(id))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePieces}. */
export function useDeleteDesignPieces(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const entity = useDesignEntity();
  return bindOpToReact<Design, [readonly string[]]>((d, ids) => d.deletePieces(ids))(() => entity);
}

/** @emoji ✍️ {@link Design#deletePiecesAndConnections}. */
export function useDeleteDesignPiecesAndConnections(): readonly [
  (pieceIds: readonly string[], connectionIds: readonly string[]) => Promise<SetResult>,
  OperationStatus,
] {
  const entity = useDesignEntity();
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
const useTypeCreatePortOp = bindOpToReact<
  Type,
  [string | null | undefined, string | null | undefined, string | null | undefined, string | null | undefined, number | null | undefined]
>((t, code, label, description, icon, order) => t.createPort(code ?? null, label ?? null, description ?? null, icon ?? null, order ?? null));
const useTypeDeletePortOp = bindOpToReact<Type, [string]>((t, id) => t.deletePort(id));
const useTypeDeletePortsOp = bindOpToReact<Type, [readonly string[]]>((t, ids) => t.deletePorts(ids));
const useTypeAddConnectorOp = bindOpToReact<Type, [string, string | null | undefined, string | null | undefined, string | null | undefined]>(
  (t, code, description, icon, portId) => t.addConnector(code, description ?? null, icon ?? null, portId ?? null),
);
const useTypeRemoveConnectorOp = bindOpToReact<Type, [string]>((t, id) => t.removeConnector(id));
const useTypeRemoveConnectorsOp = bindOpToReact<Type, [readonly string[]]>((t, ids) => t.removeConnectors(ids));

/** @emoji 🛡️ Contextual {@link Type}. */
export function useType(): Type | null {
  return useTypeEntity();
}

/** @emoji 📖 Live {@link Type#readName}. */
export function useTypeName(): FieldReadState<string> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, string>({ getEntity: () => entity, read: (t) => t.readName() })();
}

/** @emoji 📖 Live {@link Type#readDescription}. */
export function useTypeDescription(): FieldReadState<string> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, string>({ getEntity: () => entity, read: (t) => t.readDescription() })();
}

/** @emoji 📖 Live {@link Type#readIcon}. */
export function useTypeIcon(): FieldReadState<string> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, string>({ getEntity: () => entity, read: (t) => t.readIcon() })();
}

/** @emoji 📖 Live {@link Type#readImage}. */
export function useTypeImage(): FieldReadState<string> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, string>({ getEntity: () => entity, read: (t) => t.readImage() })();
}

/** @emoji 📖 Live {@link Type#readUnit}. */
export function useTypeUnit(): FieldReadState<string> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, string>({ getEntity: () => entity, read: (t) => t.readUnit() })();
}

/** @emoji 📖 Bulky {@link Type#readConnectors}. */
export function useTypeConnectors(): FieldReadState<readonly { readonly id: string; readonly code: string; readonly name: string }[]> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, readonly { readonly id: string; readonly code: string; readonly name: string }[]>({ getEntity: () => entity, read: (t) => t.readConnectors() })();
}

/** @emoji 📖 Bulky {@link Type#readRepresentations}. */
export function useTypeRepresentations(): FieldReadState<readonly { readonly id: string }[]> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, readonly { readonly id: string }[]>({ getEntity: () => entity, read: (t) => t.readRepresentations() })();
}

/** @emoji 📖 Bulky {@link Type#readAttributes}. */
export function useTypeAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useTypeEntity();
  return bindFieldToReact<Type, readonly AttributeWire[]>({ getEntity: () => entity, read: (t) => t.readAttributes() })();
}

/** @emoji ✍️ {@link TypeOperationInput#rename}. */
export function useRenameType(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeRenameOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeDescription}. */
export function useChangeTypeDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#changeIcon}. */
export function useChangeTypeIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeChangeIconOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addAttribute}. */
export function useAddTypeAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttribute}. */
export function useRemoveTypeAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeAttributes}. */
export function useRemoveTypeAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeRemoveAttributesOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#createPort}. */
export function useCreatePort(): readonly [
  (
    code?: string | null,
    label?: string | null,
    description?: string | null,
    icon?: string | null,
    order?: number | null,
  ) => Promise<SetResult>,
  OperationStatus,
] {
  const e = useTypeEntity();
  return useTypeCreatePortOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePort}. */
export function useDeletePort(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeDeletePortOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#deletePorts}. */
export function useDeletePorts(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeDeletePortsOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#addConnector}. */
export function useAddConnector(): readonly [
  (code: string, description?: string | null, icon?: string | null, portId?: string | null) => Promise<SetResult>,
  OperationStatus,
] {
  const e = useTypeEntity();
  return useTypeAddConnectorOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnector}. */
export function useRemoveConnector(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
  return useTypeRemoveConnectorOp(() => e);
}

/** @emoji ✍️ {@link TypeOperationInput#removeConnectors}. */
export function useRemoveConnectors(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTypeEntity();
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

/** @emoji 🛡️ Contextual {@link Port}. */
export function usePort(): Port | null {
  return usePortEntity();
}

/** @emoji 📖 Live {@link Port#readCode}. */
export function usePortCode(): FieldReadState<string> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, string>({ getEntity: () => entity, read: (p) => p.readCode() })();
}

/** @emoji 📖 Live {@link Port#readLabel}. */
export function usePortLabel(): FieldReadState<string> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, string>({ getEntity: () => entity, read: (p) => p.readLabel() })();
}

/** @emoji 📖 Live {@link Port#readOrder}. */
export function usePortOrder(): FieldReadState<number | null> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, number | null>({ getEntity: () => entity, read: (p) => p.readOrder() })();
}

/** @emoji 📖 Live {@link Port#readName}. */
export function usePortName(): FieldReadState<string> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, string>({ getEntity: () => entity, read: (p) => p.readName() })();
}

/** @emoji 📖 Live {@link Port#readDescription}. */
export function usePortDescription(): FieldReadState<string> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, string>({ getEntity: () => entity, read: (p) => p.readDescription() })();
}

/** @emoji 📖 Live {@link Port#readIcon}. */
export function usePortIcon(): FieldReadState<string> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, string>({ getEntity: () => entity, read: (p) => p.readIcon() })();
}

/** @emoji 📖 Bulky {@link Port#readAttributes}. */
export function usePortAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = usePortEntity();
  return bindFieldToReact<Port, readonly AttributeWire[]>({ getEntity: () => entity, read: (p) => p.readAttributes() })();
}

/** @emoji ✍️ {@link PortOperationInput#rename}. */
export function useRenamePort(): readonly [(newCode: string, newLabel?: string | null) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortRenameOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeDescription}. */
export function useChangePortDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#changeIcon}. */
export function useChangePortIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortChangeIconOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#addAttribute}. */
export function useAddPortAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttribute}. */
export function useRemovePortAttribute(): readonly [(id: string) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link PortOperationInput#removeAttributes}. */
export function useRemovePortAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePortEntity();
  return usePortRemoveAttributesOp(() => e);
}
// #endregion 🔘Port

// #region 🔗Connector
const useConnectorRenameOp = bindOpToReact<Connector, [string]>((c, newCode) => c.rename(newCode));
const useConnectorChangeDescriptionOp = bindOpToReact<Connector, [string]>((c, d) => c.changeDescription(d));
const useConnectorChangeIconOp = bindOpToReact<Connector, [string]>((c, i) => c.changeIcon(i));

/** @emoji 🛡️ Contextual {@link Connector}. */
export function useConnector(): Connector | null {
  return useConnectorEntity();
}

/** @emoji 📖 Live {@link Connector#readCode}. */
export function useConnectorCode(): FieldReadState<string> {
  const entity = useConnectorEntity();
  return bindFieldToReact<Connector, string>({ getEntity: () => entity, read: (c) => c.readCode() })();
}

/** @emoji 📖 Live {@link Connector#readDescription}. */
export function useConnectorDescription(): FieldReadState<string> {
  const entity = useConnectorEntity();
  return bindFieldToReact<Connector, string>({ getEntity: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Connector#readIcon}. */
export function useConnectorIcon(): FieldReadState<string> {
  const entity = useConnectorEntity();
  return bindFieldToReact<Connector, string>({ getEntity: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Bulky {@link Connector#readAttributes}. */
export function useConnectorAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useConnectorEntity();
  return bindFieldToReact<Connector, readonly AttributeWire[]>({ getEntity: () => entity, read: (c) => c.readAttributes() })();
}

/** @emoji ✍️ {@link ConnectorOperationInput#rename}. */
export function useRenameConnector(): readonly [(newCode: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnectorEntity();
  return useConnectorRenameOp(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeDescription}. */
export function useChangeConnectorDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnectorEntity();
  return useConnectorChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link ConnectorOperationInput#changeIcon}. */
export function useChangeConnectorIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConnectorEntity();
  return useConnectorChangeIconOp(() => e);
}
// #endregion 🔗Connector

// #region ✍️Author
/** @emoji 🛡️ Alias for {@link useAuthorEntity} (plan name {@code useAuthor}). */
export function useAuthor(): Author | null {
  return useAuthorEntity();
}

/** @emoji 📖 Live {@link Author#readName}. */
export function useAuthorName(): FieldReadState<string> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, string>({ getEntity: () => entity, read: (a) => a.readName() })();
}

/** @emoji 📖 Live {@link Author#readEmail}. */
export function useAuthorEmail(): FieldReadState<string> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, string>({ getEntity: () => entity, read: (a) => a.readEmail() })();
}

/** @emoji 📖 Live {@link Author#readRank}. */
export function useAuthorRank(): FieldReadState<number | null> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, number | null>({ getEntity: () => entity, read: (a) => a.readRank() })();
}

/** @emoji 📖 Live {@link Author#readDescription}. */
export function useAuthorDescription(): FieldReadState<string> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, string>({ getEntity: () => entity, read: (a) => a.readDescription() })();
}

/** @emoji 📖 Live {@link Author#readIcon}. */
export function useAuthorIcon(): FieldReadState<string> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, string>({ getEntity: () => entity, read: (a) => a.readIcon() })();
}

/** @emoji 📖 Live {@link Author#readRole}. */
export function useAuthorRole(): FieldReadState<string> {
  const entity = useAuthorEntity();
  return bindFieldToReact<Author, string>({ getEntity: () => entity, read: (a) => a.readRole() })();
}
// #endregion ✍️Author

// #region 💎Quality
const useQualityRenameOp = bindOpToReact<Quality, [string]>((q, k) => q.rename(k));
const useQualityChangeDescriptionOp = bindOpToReact<Quality, [string]>((q, d) => q.changeDescription(d));
const useQualityChangeIconOp = bindOpToReact<Quality, [string]>((q, i) => q.changeIcon(i));
const useQualityAddAttributeOp = bindOpToReact<Quality, [string, string, string]>((q, key, value, definition) => q.addAttribute(key, value, definition));
const useQualityRemoveAttributeOp = bindOpToReact<Quality, [string]>((q, id) => q.removeAttribute(id));
const useQualityRemoveAttributesOp = bindOpToReact<Quality, [readonly string[]]>((q, ids) => q.removeAttributes(ids));

/** @emoji 🛡️ Alias for {@link useQualityEntity}. */
export function useQuality(): Quality | null {
  return useQualityEntity();
}

/** @emoji 📖 Live {@link Quality#readKey}. */
export function useQualityKey(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readKey() })();
}

/** @emoji 📖 Live {@link Quality#readValue}. */
export function useQualityValue(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readValue() })();
}

/** @emoji 📖 Live {@link Quality#readUnit}. */
export function useQualityUnit(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readUnit() })();
}

/** @emoji 📖 Live {@link Quality#readDefinition}. */
export function useQualityDefinition(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readDefinition() })();
}

/** @emoji 📖 Live {@link Quality#readDescription}. */
export function useQualityDescription(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readDescription() })();
}

/** @emoji 📖 Live {@link Quality#readIcon}. */
export function useQualityIcon(): FieldReadState<string> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, string>({ getEntity: () => entity, read: (q) => q.readIcon() })();
}

/** @emoji 📖 Live {@link Quality#readAttributes}. */
export function useQualityAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, readonly AttributeWire[]>({ getEntity: () => entity, read: (q) => q.readAttributes() })();
}

/** @emoji 📖 Live {@link Quality#readBenchmarks}. */
export function useQualityBenchmarks(): FieldReadState<readonly BenchmarkWire[]> {
  const entity = useQualityEntity();
  return bindFieldToReact<Quality, readonly BenchmarkWire[]>({ getEntity: () => entity, read: (q) => q.readBenchmarks() })();
}

/** @emoji ✍️ {@link Quality#rename}. */
export function useRenameQuality(): readonly [(newKey: string) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
  return useQualityRenameOp(() => e);
}

/** @emoji ✍️ {@link Quality#changeDescription}. */
export function useChangeQualityDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
  return useQualityChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Quality#changeIcon}. */
export function useChangeQualityIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
  return useQualityChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Quality#addAttribute}. */
export function useAddQualityAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
  return useQualityAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttribute}. */
export function useRemoveQualityAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
  return useQualityRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Quality#removeAttributes}. */
export function useRemoveQualityAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useQualityEntity();
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

/** @emoji 🛡️ Alias for {@link useTagEntity}. */
export function useTag(): Tag | null {
  return useTagEntity();
}

/** @emoji 📖 Live {@link Tag#readName}. */
export function useTagName(): FieldReadState<string> {
  const entity = useTagEntity();
  return bindFieldToReact<Tag, string>({ getEntity: () => entity, read: (t) => t.readName() })();
}

/** @emoji 📖 Live {@link Tag#readDescription}. */
export function useTagDescription(): FieldReadState<string> {
  const entity = useTagEntity();
  return bindFieldToReact<Tag, string>({ getEntity: () => entity, read: (t) => t.readDescription() })();
}

/** @emoji 📖 Live {@link Tag#readIcon}. */
export function useTagIcon(): FieldReadState<string> {
  const entity = useTagEntity();
  return bindFieldToReact<Tag, string>({ getEntity: () => entity, read: (t) => t.readIcon() })();
}

/** @emoji 📖 Live {@link Tag#readOrder}. */
export function useTagOrder(): FieldReadState<number | null> {
  const entity = useTagEntity();
  return bindFieldToReact<Tag, number | null>({ getEntity: () => entity, read: (t) => t.readOrder() })();
}

/** @emoji 📖 Live {@link Tag#readAttributes}. */
export function useTagAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useTagEntity();
  return bindFieldToReact<Tag, readonly AttributeWire[]>({ getEntity: () => entity, read: (t) => t.readAttributes() })();
}

/** @emoji ✍️ {@link Tag#rename}. */
export function useRenameTag(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
  return useTagRenameOp(() => e);
}

/** @emoji ✍️ {@link Tag#changeDescription}. */
export function useChangeTagDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
  return useTagChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Tag#changeIcon}. */
export function useChangeTagIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
  return useTagChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Tag#addAttribute}. */
export function useAddTagAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
  return useTagAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttribute}. */
export function useRemoveTagAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
  return useTagRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Tag#removeAttributes}. */
export function useRemoveTagAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useTagEntity();
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

/** @emoji 🛡️ Alias for {@link useConceptEntity}. */
export function useConcept(): Concept | null {
  return useConceptEntity();
}

/** @emoji 📖 Live {@link Concept#readName}. */
export function useConceptName(): FieldReadState<string> {
  const entity = useConceptEntity();
  return bindFieldToReact<Concept, string>({ getEntity: () => entity, read: (c) => c.readName() })();
}

/** @emoji 📖 Live {@link Concept#readDescription}. */
export function useConceptDescription(): FieldReadState<string> {
  const entity = useConceptEntity();
  return bindFieldToReact<Concept, string>({ getEntity: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Concept#readIcon}. */
export function useConceptIcon(): FieldReadState<string> {
  const entity = useConceptEntity();
  return bindFieldToReact<Concept, string>({ getEntity: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Live {@link Concept#readOrder}. */
export function useConceptOrder(): FieldReadState<number | null> {
  const entity = useConceptEntity();
  return bindFieldToReact<Concept, number | null>({ getEntity: () => entity, read: (c) => c.readOrder() })();
}

/** @emoji 📖 Live {@link Concept#readAttributes}. */
export function useConceptAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useConceptEntity();
  return bindFieldToReact<Concept, readonly AttributeWire[]>({ getEntity: () => entity, read: (c) => c.readAttributes() })();
}

/** @emoji ✍️ {@link Concept#rename}. */
export function useRenameConcept(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptRenameOp(() => e);
}

/** @emoji ✍️ {@link Concept#changeDescription}. */
export function useChangeConceptDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Concept#changeIcon}. */
export function useChangeConceptIcon(): readonly [(newIcon: string) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptChangeIconOp(() => e);
}

/** @emoji ✍️ {@link Concept#addAttribute}. */
export function useAddConceptAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttribute}. */
export function useRemoveConceptAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Concept#removeAttributes}. */
export function useRemoveConceptAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = useConceptEntity();
  return useConceptRemoveAttributesOp(() => e);
}
// #endregion 💡Concept

// #region 🎨Representation
/** @emoji 🛡️ Alias for {@link useRepresentationEntity}. */
export function useRepresentation(): Representation | null {
  return useRepresentationEntity();
}

/** @emoji 📖 Live {@link Representation#readUrl}. */
export function useRepresentationUrl(): FieldReadState<string> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, string>({ getEntity: () => entity, read: (r) => r.readUrl() })();
}

/** @emoji 📖 Live {@link Representation#readDescription}. */
export function useRepresentationDescription(): FieldReadState<string> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, string>({ getEntity: () => entity, read: (r) => r.readDescription() })();
}

/** @emoji 📖 Live {@link Representation#readTagIds} (plan “tags”). */
export function useRepresentationTags(): FieldReadState<readonly string[]> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, readonly string[]>({ getEntity: () => entity, read: (r) => r.readTagIds() })();
}

/** @emoji 📖 Live {@link Representation#readQualityIds}; schema has {@code qualities}, not LOD. */
export function useRepresentationLod(): FieldReadState<readonly string[]> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, readonly string[]>({ getEntity: () => entity, read: (r) => r.readQualityIds() })();
}

/** @emoji 📖 Live {@link Representation#readAttributes}. */
export function useRepresentationAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, readonly AttributeWire[]>({ getEntity: () => entity, read: (r) => r.readAttributes() })();
}

/** @emoji 📖 Live {@link Representation#readFileId}. */
export function useRepresentationFileId(): FieldReadState<string> {
  const entity = useRepresentationEntity();
  return bindFieldToReact<Representation, string>({ getEntity: () => entity, read: (r) => r.readFileId() })();
}
// #endregion 🎨Representation

// #region 🧩Piece
/** @emoji 🛡️ Resolves the contextual {@link Piece} from {@link PieceEntityContextProvider}. */
export function usePiece(): Piece | null {
  return usePieceEntity();
}

/** @emoji 📖 Live {@link Piece#readName}. */
export function usePieceName(): FieldReadState<string> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, string>({ getEntity: () => entity, read: (p) => p.readName() })();
}

/** @emoji 📖 Live {@link Piece#readDescription}. */
export function usePieceDescription(): FieldReadState<string> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, string>({ getEntity: () => entity, read: (p) => p.readDescription() })();
}

/** @emoji 📖 Live {@link Piece#readIcon}. */
export function usePieceIcon(): FieldReadState<string> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, string>({ getEntity: () => entity, read: (p) => p.readIcon() })();
}

/** @emoji 📖 Live {@link Piece#readScale}. */
export function usePieceScale(): FieldReadState<number | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, number | null>({ getEntity: () => entity, read: (p) => p.readScale() })();
}

/** @emoji 📖 Live {@link Piece#readPosition}. */
export function usePiecePosition(): FieldReadState<PositionWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, PositionWire | null>({ getEntity: () => entity, read: (p) => p.readPosition() })();
}

/** @emoji 📖 Live {@link Piece#readFlatPosition}. */
export function usePieceFlatPosition(): FieldReadState<PositionWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, PositionWire | null>({ getEntity: () => entity, read: (p) => p.readFlatPosition() })();
}

/** @emoji 📖 Live {@link Piece#readPlane}. */
export function usePiecePlane(): FieldReadState<PlaneWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, PlaneWire | null>({ getEntity: () => entity, read: (p) => p.readPlane() })();
}

/** @emoji 📖 Live {@link Piece#readCenter}. */
export function usePieceCenter(): FieldReadState<CoordinateWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, CoordinateWire | null>({ getEntity: () => entity, read: (p) => p.readCenter() })();
}

/** @emoji 📖 Live {@link Piece#readFlatPlane}. */
export function usePieceFlatPlane(): FieldReadState<PlaneWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, PlaneWire | null>({ getEntity: () => entity, read: (p) => p.readFlatPlane() })();
}

/** @emoji 📖 Live {@link Piece#readFlatCenter}. */
export function usePieceFlatCenter(): FieldReadState<CoordinateWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, CoordinateWire | null>({ getEntity: () => entity, read: (p) => p.readFlatCenter() })();
}

/** @emoji 📖 Live {@link Piece#readBlueprint}. */
export function usePieceBlueprint(): FieldReadState<PieceBlueprintWire | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, PieceBlueprintWire | null>({ getEntity: () => entity, read: (p) => p.readBlueprint() })();
}

/** @emoji 📖 Live {@link Piece#readAttributes}. */
export function usePieceAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, readonly AttributeWire[]>({ getEntity: () => entity, read: (p) => p.readAttributes() })();
}

/** @emoji 📖 Live {@link Piece#readConnectionKind}. */
export function usePieceConnectionKind(): FieldReadState<"FIXED" | "CONNECTED" | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, "FIXED" | "CONNECTED" | null>({ getEntity: () => entity, read: (p) => p.readConnectionKind() })();
}

/** @emoji 📖 Live {@link Piece#readParentPieceId}. */
export function usePieceParentPieceId(): FieldReadState<string | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, string | null>({ getEntity: () => entity, read: (p) => p.readParentPieceId() })();
}

/** @emoji 📖 Live {@link Piece#readParentConnectionId}. */
export function usePieceParentConnectionId(): FieldReadState<string | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, string | null>({ getEntity: () => entity, read: (p) => p.readParentConnectionId() })();
}

/** @emoji 📖 Live {@link Piece#readChildPieceIds}. */
export function usePieceChildPieceIds(): FieldReadState<readonly string[]> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, readonly string[]>({ getEntity: () => entity, read: (p) => p.readChildPieceIds() })();
}

/** @emoji 📖 Live {@link Piece#readChildConnectionIds}. */
export function usePieceChildConnectionIds(): FieldReadState<readonly string[]> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, readonly string[]>({ getEntity: () => entity, read: (p) => p.readChildConnectionIds() })();
}

/** @emoji 📖 Live {@link Piece#readDepth}. */
export function usePieceDepth(): FieldReadState<number | null> {
  const entity = usePieceEntity();
  return bindFieldToReact<Piece, number | null>({ getEntity: () => entity, read: (p) => p.readDepth() })();
}

const usePieceRenameOp = bindOpToReact<Piece, [string]>((p, n) => p.rename(n));
const usePieceChangeDescriptionOp = bindOpToReact<Piece, [string]>((p, d) => p.changeDescription(d));
const usePieceDragOp = bindOpToReact<Piece, [OffsetInputWire]>((p, o) => p.drag(o));
const usePieceMoveOp = bindOpToReact<Piece, [PositionInputWire]>((p, pos) => p.move(pos));
const usePieceFixOp = bindOpToReact<Piece, []>((p) => p.fix());
const usePieceChangeBlueprintOp = bindOpToReact<Piece, [string]>((p, id) => p.changeBlueprint(id));
const usePieceAddAttributeOp = bindOpToReact<Piece, [string, string, string]>((p, key, value, definition) => p.addAttribute(key, value, definition));
const usePieceRemoveAttributeOp = bindOpToReact<Piece, [string]>((p, id) => p.removeAttribute(id));
const usePieceRemoveAttributesOp = bindOpToReact<Piece, [readonly string[]]>((p, ids) => p.removeAttributes(ids));

/** @emoji ✍️ {@link Piece#rename} bound to {@link PieceEntityContext}. */
export function useRenamePiece(): readonly [(newName: string) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceRenameOp(() => e);
}

/** @emoji ✍️ {@link Piece#changeDescription}. */
export function useChangePieceDescription(): readonly [(newDescription: string) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceChangeDescriptionOp(() => e);
}

/** @emoji ✍️ {@link Piece#drag}. */
export function useDragPiece(): readonly [(offset: OffsetInputWire) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceDragOp(() => e);
}

/** @emoji ✍️ {@link Piece#move}. */
export function useMovePiece(): readonly [(position: PositionInputWire) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceMoveOp(() => e);
}

/** @emoji ✍️ {@link Piece#fix}. */
export function useFixPiece(): readonly [() => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceFixOp(() => e);
}

/** @emoji ✍️ {@link Piece#changeBlueprint}. */
export function useChangePieceBlueprint(): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceChangeBlueprintOp(() => e);
}

/** @emoji ✍️ {@link Piece#addAttribute}. */
export function useAddPieceAttribute(): readonly [(key: string, value: string, definition: string) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceAddAttributeOp(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttribute}. */
export function useRemovePieceAttribute(): readonly [(attributeId: string) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceRemoveAttributeOp(() => e);
}

/** @emoji ✍️ {@link Piece#removeAttributes}. */
export function useRemovePieceAttributes(): readonly [(ids: readonly string[]) => Promise<SetResult>, OperationStatus] {
  const e = usePieceEntity();
  return usePieceRemoveAttributesOp(() => e);
}
// #endregion 🧩Piece

// #region 🪢Pieces
/**
 * @emoji 🪝 Binds {@link PiecesOperations} batch mutations (not an {@link Entity} — no cached kit state on the handle).
 * @typeParam Args — forwarded to the underlying {@link PiecesOperations} method after the ops handle.
 */
function bindPiecesOperationsOpToReact<Args extends unknown[]>(
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

const usePiecesDragOp = bindPiecesOperationsOpToReact((ops, o: OffsetInputWire) => ops.drag(o));
const usePiecesMoveOp = bindPiecesOperationsOpToReact((ops, o: OffsetInputWire) => ops.move(o));
const usePiecesFixOp = bindPiecesOperationsOpToReact((ops) => ops.fix());
const usePiecesChangeBlueprintOp = bindPiecesOperationsOpToReact((ops, id: string) => ops.changeBlueprint(id));

/** @emoji ✍️ {@link PiecesOperations#drag} for {@code design.pieces(ids)}. */
export function useDragPieces(
  designId: string,
  pieceIds: readonly string[],
): readonly [(offset: OffsetInputWire) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  const getOps = React.useCallback(
    () => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)),
    [kit, designId, pieceIds],
  );
  return usePiecesDragOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#move}. */
export function useMovePieces(
  designId: string,
  pieceIds: readonly string[],
): readonly [(offset: OffsetInputWire) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  const getOps = React.useCallback(
    () => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)),
    [kit, designId, pieceIds],
  );
  return usePiecesMoveOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#fix}. */
export function useFixPieces(
  designId: string,
  pieceIds: readonly string[],
): readonly [() => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  const getOps = React.useCallback(
    () => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)),
    [kit, designId, pieceIds],
  );
  return usePiecesFixOp(getOps);
}

/** @emoji ✍️ {@link PiecesOperations#changeBlueprint}. */
export function useChangePiecesBlueprint(
  designId: string,
  pieceIds: readonly string[],
): readonly [(blueprintId: string) => Promise<SetResult>, OperationStatus] {
  const { kit } = useKit();
  const getOps = React.useCallback(
    () => (pieceIds.length === 0 ? null : kit.design(designId).pieces(pieceIds)),
    [kit, designId, pieceIds],
  );
  return usePiecesChangeBlueprintOp(getOps);
}
// #endregion 🪢Pieces

// #region ⛓️Connection
/** @emoji 🛡️ Resolves the contextual {@link Connection} from {@link ConnectionEntityContextProvider}. */
export function useConnection(): Connection | null {
  return useConnectionEntity();
}

/** @emoji 📖 Live {@link Connection#readGap}. */
export function useConnectionGap(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readGap() })();
}

/** @emoji 📖 Live {@link Connection#readShift}. */
export function useConnectionShift(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readShift() })();
}

/** @emoji 📖 Live {@link Connection#readRise}. */
export function useConnectionRise(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readRise() })();
}

/** @emoji 📖 Live {@link Connection#readRotation}. */
export function useConnectionRotation(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readRotation() })();
}

/** @emoji 📖 Live {@link Connection#readTurn}. */
export function useConnectionTurn(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readTurn() })();
}

/** @emoji 📖 Live {@link Connection#readTilt}. */
export function useConnectionTilt(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readTilt() })();
}

/** @emoji 📖 Live {@link Connection#readU}. */
export function useConnectionU(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readU() })();
}

/** @emoji 📖 Live {@link Connection#readV}. */
export function useConnectionV(): FieldReadState<number | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, number | null>({ getEntity: () => entity, read: (c) => c.readV() })();
}

/** @emoji 📖 Live {@link Connection#readConnected}. */
export function useConnectionConnected(): FieldReadState<ConnectionSideWire | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, ConnectionSideWire | null>({ getEntity: () => entity, read: (c) => c.readConnected() })();
}

/** @emoji 📖 Live {@link Connection#readConnecting}. */
export function useConnectionConnecting(): FieldReadState<ConnectionSideWire | null> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, ConnectionSideWire | null>({ getEntity: () => entity, read: (c) => c.readConnecting() })();
}

/** @emoji 📖 Live {@link Connection#readName}. */
export function useConnectionName(): FieldReadState<string> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, string>({ getEntity: () => entity, read: (c) => c.readName() })();
}

/** @emoji 📖 Live {@link Connection#readDescription}. */
export function useConnectionDescription(): FieldReadState<string> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, string>({ getEntity: () => entity, read: (c) => c.readDescription() })();
}

/** @emoji 📖 Live {@link Connection#readIcon}. */
export function useConnectionIcon(): FieldReadState<string> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, string>({ getEntity: () => entity, read: (c) => c.readIcon() })();
}

/** @emoji 📖 Live {@link Connection#readAttributes}. */
export function useConnectionAttributes(): FieldReadState<readonly AttributeWire[]> {
  const entity = useConnectionEntity();
  return bindFieldToReact<Connection, readonly AttributeWire[]>({ getEntity: () => entity, read: (c) => c.readAttributes() })();
}
// #endregion ⛓️Connection


// #region ⚛️Embedded tests
const shouldRunReactEmbeddedTests =
  (typeof process !== "undefined" && process.env.SEMIO_REACT_RUN_EMBEDDED_TESTS === "1") || (typeof (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ !== "undefined" && (globalThis as any).__SEMIO_REACT_RUN_EMBEDDED_TESTS__ === true);

if (shouldRunReactEmbeddedTests) {
  const { describe, expect, it } = await import("vitest");
  const { act, cleanup, render, waitFor } = await import("@testing-library/react");
  const { InMemoryKitStore, asKitInstance, kitReadPointKey, theKitReadPoint, StoreField, StoreCommand } = await import("@semio/js");

  const kitJsonFromStore = (store: KitHostStore) => {
    const host = store as KitHostStore & { _kit?: { toJSON: () => unknown } };
    if (((store as any).__semioKitBridge || (store as any).__semioKitClient) && host._kit) return host._kit.toJSON();
    return store.getSnapshot().kit.toJSON();
  };

  const createTestKitClient = (store: KitHostStore): KitStoreClient => {
    const initialName = String((kitJsonFromStore(store) as KitFullDto).name ?? "");
    let pushKitName!: (v: string) => void;
    const kitNameField = new StoreField<string>(initialName, (push) => {
      pushKitName = push;
      push(initialName);
      return () => {};
    });
    const renameKitCmd = new StoreCommand<import("@semio/js").RenameKitCommandArgs>(async (args) => {
      const v = String(args.input?.name ?? "").trim();
      if (v === "") return { ok: false, error: { kind: "InvalidValue", message: "kit name required" } };
      const kitDto: KitFullDto = JSON.parse(JSON.stringify(kitJsonFromStore(store))) as KitFullDto;
      (kitDto as { name: string }).name = v;
      store.replace(asKitInstance(kitDto));
      pushKitName(v);
      return { ok: true };
    });
    return {
      fetchFullKit: async () => kitJsonFromStore(store) as KitFullDto,
      kitReadPoint: theKitReadPoint,
      submitChangeKitCommands: async (commands: readonly ChangeKitCommand[]) => {
        const kit: KitFullDto = JSON.parse(JSON.stringify(kitJsonFromStore(store))) as KitFullDto;
        for (const cmd of commands) {
          const c = cmd as Record<string, unknown>;
          if ("name" in c && c.name && typeof c.name === "object") {
            const nm = String((c.name as { name?: string }).name ?? "");
            if (nm.trim() === "") return { ok: false, error: { kind: "IllegalName", message: "name cannot be empty" } };
            (kit as { name: string }).name = nm;
            pushKitName(nm);
          }
          if ("description" in c && c.description && typeof c.description === "object")
            (kit as { description?: string }).description =
              (c.description as { description?: string | null }).description ?? undefined;
          if ("icon" in c && c.icon && typeof c.icon === "object")
            (kit as { icon?: string }).icon = (c.icon as { icon?: string | null }).icon ?? undefined;
          if ("image" in c && c.image && typeof c.image === "object")
            (kit as { image?: string }).image = (c.image as { image?: string | null }).image ?? undefined;
          if ("version" in c && c.version && typeof c.version === "object")
            (kit as { version?: string }).version = (c.version as { version?: string | null }).version ?? undefined;
          if ("homepage" in c && c.homepage && typeof c.homepage === "object")
            (kit as { homepage?: string }).homepage = (c.homepage as { homepage?: string | null }).homepage ?? undefined;
          if ("license" in c && c.license && typeof c.license === "object")
            (kit as { license?: string }).license = (c.license as { license?: string | null }).license ?? undefined;
        }
        store.replace(asKitInstance(kit));
        return { ok: true };
      },
      readPieceFlatPlane: async () => null,
      readPieceFlatCenter: async () => null,
      readPieceParentConnectionFull: async () => null,
      readDesignIncludedDesigns: async () => [],
      readDesignClusterableGroups: async () => [],
      readDesignQualitySum: async () => 0,
      readTypeBestRepresentation: async () => null,
      readColoredConnectors: async () => [],
      readDesignReplaceableCatalogTypes: async () => [],
      readDesignReplaceableCatalogDesigns: async () => [],
      readDesignIncludedDesignIds: async () => [],
      kitGraphql: () => {
        throw new Error("kitGraphql not available in embedded test client");
      },
      clusterPieces: async () => ({ ok: true }),
      dragPieces: async () => ({ ok: true }),
      movePieces: async () => ({ ok: true }),
      fixPieces: async () => ({ ok: true }),
      flattenDesign: async () => ({ ok: true }),
      expandDesign: async () => ({ ok: true }),
      deleteConnection: async () => ({ ok: true }),
      changePieceType: async () => ({ ok: true }),
      pasteDesignSelection: async () => ({ ok: true }),
      createHangingPieces: async () => ({ ok: true }),
      createConnectedPiece: async () => ({ ok: true }),
      createFixedPiece: async () => ({ ok: true }),
      getPiecesMetadata: async () => new Map(),
      getPieces: async () => [],
      getConnections: async () => [],
      getDesigns: async () => [],
      getTypes: async () => [],
      getAuthors: async () => [],
      getKitMetadata: async () => {
        const k = kitJsonFromStore(store) as KitFullDto;
        return { id: String(k.id ?? ""), name: String(k.name ?? "") };
      },
      undo: async () => ({ ok: true }),
      redo: async () => ({ ok: true }),
      canUndo: async () => false,
      canRedo: async () => false,
      backboneStatus: async () => ({ attached: false, kind: null, backboneTip: null, pendingWipCheckpoints: 0 }),
      attachBackbone: async () => ({ ok: true } as const),
      detachBackbone: async () => ({ ok: true } as const),
      listConflicts: async () => [] as const,
      resolveConflict: async () => ({ ok: true } as const),
      syncNow: async () => ({ ok: true } as const),
      kitName: kitNameField,
      renameKit: renameKitCmd,
      readKitName: async () => String((kitJsonFromStore(store) as KitFullDto).name ?? ""),
      createAlternativeFromTip: async () => "alt-test",
      getKitWriteScope: () => null,
      setKitWriteScope: () => {},
      finalizeKitWriteTransaction: async () => ({ ok: true }),
      abortKitWriteTransaction: async () => ({ ok: true }),
      subscribe: (cb: (ev: any) => void) => store.subscribe(() => cb({ kind: "test" })),
      setKitReadPoint: (_s: import("@semio/js").KitReadPoint) => {},
      dispose: () => {
        kitNameField.dispose();
        renameKitCmd.dispose();
      },
    } as unknown as KitStoreClient;
  };

  describe("pipeline hooks", () => {
    it("useKitName rejects empty required name via kit client", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "N" }],
          },
        ],
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let renameKit: ((v: string) => Promise<SetResult>) | undefined;
      let lastStatus: OperationStatus | undefined;
      let client: KitStoreClient | null = null;

      function Probe() {
        const [run, st] = useRenameKit();
        renameKit = run;
        lastStatus = st;
        client = useKitStoreClient();
        return null;
      }

      render(React.createElement(KitScope, { store, kitClient, children: React.createElement(Probe) }));

      await waitFor(() => {
        expect(renameKit).toBeDefined();
        expect(client).not.toBeNull();
      });
      const r = await renameKit!("");
      expect(r.ok).toBe(false);
      await waitFor(() => expect(lastStatus?.kind === "settled" && lastStatus.kind === "settled" && !lastStatus.result.ok).toBe(true));
    });

    it("embedded kit client stub exposes read promise methods used by live-read hooks", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const c = createTestKitClient(store);
      expect(typeof c.readPieceFlatPlane).toBe("function");
      expect(typeof c.readDesignIncludedDesigns).toBe("function");
      expect(typeof c.readDesignReplaceableCatalogTypes).toBe("function");
    });

    it("kit metadata hooks write through the kit client (segregated read+mutation pattern)", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let setName: ((v: string) => Promise<any>) | undefined;
      let patchKit: ((p: Record<string, unknown>) => Promise<any>) | undefined;
      let client: KitStoreClient | null = null;

      function Probe() {
        setName = useRenameKit()[0];
        patchKit = usePatchKit().run;
        client = useKitStoreClient();
        return null;
      }

      render(React.createElement(KitScope, { store, kitClient, children: React.createElement(Probe) }));
      await waitFor(() => {
        expect(patchKit).toBeDefined();
        expect(client).not.toBeNull();
      });

      expect((await setName!("Renamed Kit")).ok).toBe(true);
      expect((await patchKit!({ release: "1.2.3", description: "Updated description", icon: "spark", image: "kit.png", homepage: "https://semio.example", license: "LGPL-3.0-or-later" })).ok).toBe(true);

      await waitFor(() => {
        const next = store.getSnapshot().kit.toJSON();
        expect(next.name).toBe("Renamed Kit");
        expect(next.version).toBe("1.2.3");
        expect(next.description).toBe("Updated description");
        expect(next.icon).toBe("spark");
        expect(next.image).toBe("kit.png");
        expect(next.homepage).toBe("https://semio.example");
        expect(next.license).toBe("LGPL-3.0-or-later");
      });
    });

    it("usePieceFlatPlane subscribes narrowly: FlattenInvalidated for one piece rerenders only that hook", async () => {
      const kit = asKitInstance({
        id: "k-gran",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const listeners = new Set<(ev: import("@semio/js").KitEvent) => void>();
      const mockKs = {
        piece(d: string, p: string, _scope: unknown) {
          void _scope;
          return {
            readFlatPlane: async () => ({ tag: `${d}:${p}`, origin: [0, 0, 0] as const }),
          };
        },
      };
      const kitClient = createTestKitClient(store) as KitStoreClient & { internalKs?: () => unknown };
      kitClient.internalKs = () => mockKs as unknown as import("@semio/js").KitStore;
      kitClient.subscribe = (cb: (ev: import("@semio/js").KitEvent) => void) => {
        listeners.add(cb);
        return () => {
          listeners.delete(cb);
        };
      };

      const renders = { p1: 0, p2: 0 };

      function Piece1() {
        usePieceFlatPlaneKitHostBinding("d1", "p1");
        renders.p1 += 1;
        return null;
      }
      function Piece2() {
        usePieceFlatPlaneKitHostBinding("d1", "p2");
        renders.p2 += 1;
        return null;
      }

      render(
        React.createElement(
          KitScope,
          { store, kitClient, children: React.createElement(React.Fragment, null, React.createElement(Piece1), React.createElement(Piece2)) },
        ),
      );

      await waitFor(() => {
        expect(renders.p1).toBeGreaterThan(0);
        expect(renders.p2).toBeGreaterThan(0);
      });

      const afterIdle = { p1: renders.p1, p2: renders.p2 };

      await act(async () => {
        const ev = { FlattenInvalidated: { design: "d1", pieces: ["p1"] } } as import("@semio/js").KitEvent;
        for (const l of [...listeners]) l(ev);
      });

      await waitFor(() => {
        expect(renders.p1).toBeGreaterThan(afterIdle.p1);
        expect(renders.p2).toBe(afterIdle.p2);
      });
    });
  });

  describe("KitRegistry + useOptimistic", () => {
    it("registry open/close refcounts and useOptimistic keeps draft until commit", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let reg: ReturnType<typeof useKitRegistry> | null = null;
      function RegProbe() {
        reg = useKitRegistry();
        return null;
      }
      render(React.createElement(KitRegistryProvider, null, React.createElement(RegProbe)));
      await waitFor(() => expect(reg).not.toBeNull());
      await reg!.open("k1", { store, kitClient });
      expect(reg!.get("k1")?.refs).toBe(1);
      await reg!.open("k1", { store });
      expect(reg!.get("k1")?.refs).toBe(2);
      reg!.close("k1");
      expect(reg!.get("k1")?.refs).toBe(1);
      reg!.close("k1");
      expect(reg!.get("k1")).toBeUndefined();

      const triad: KitFieldBinding<string> = ["hello", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      let opt: ReturnType<typeof useOptimistic<string>> | null = null;
      function OptProbe() {
        opt = useOptimistic(triad);
        return null;
      }
      render(React.createElement(OptProbe));
      await waitFor(() => expect(opt).not.toBeNull());
      expect(opt!.dirty).toBe(false);
    });
  });

  describe("getKitRegistryBridge", () => {
    it("is non-null under KitRegistryProvider and null after unmount", async () => {
      const { unmount } = render(React.createElement(KitRegistryProvider, { children: React.createElement("div", null, "x") }));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      expect(typeof b!.list).toBe("function");
      unmount();
      await waitFor(() => expect(getKitRegistryBridge()).toBeNull());
    });
  });

  describe("useOpenKitGuids + useActiveKitGuid", () => {
    it("mirrors registry list() and activeKitId after open", async () => {
      const kit = asKitInstance({
        id: "k-open",
        name: "OpenK",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let openIds: string[] = [];
      let active: string | undefined;
      function Probe() {
        openIds = useOpenKitGuids();
        active = useActiveKitGuid();
        return null;
      }
      render(React.createElement(KitRegistryProvider, null, React.createElement(Probe)));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      await b!.open("k-open", { store, kitClient });
      b!.setActiveKit("k-open");
      await waitFor(() => {
        expect(openIds).toContain("k-open");
        expect(active).toBe("k-open");
      });
    });
  });

  describe("useOpenKitShallows + useRegistryHasKit + useRegistryKitPersistenceKind", () => {
    it("returns empty shallows when no KitRegistryProvider (Home table shell)", () => {
      cleanup();
      let shallows: ReturnType<typeof useOpenKitShallows> = [];
      function Probe() {
        shallows = useOpenKitShallows();
        return null;
      }
      render(React.createElement(Probe));
      expect(shallows).toEqual([]);
    });

    it("reflects registry kit snapshots and persistence kind", async () => {
      const kit = asKitInstance({
        id: "k-shallow",
        name: "ShallowK",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
      });
      const store = new InMemoryKitStore(kit);
      const kitClient = createTestKitClient(store);
      let shallows: ReturnType<typeof useOpenKitShallows> = [];
      let hasKit = false;
      let pkind: KitPersistenceInfo["kind"] | undefined;
      function Probe() {
        shallows = useOpenKitShallows();
        hasKit = useRegistryHasKit("k-shallow");
        pkind = useRegistryKitPersistenceKind("k-shallow");
        return null;
      }
      const { unmount } = render(React.createElement(KitRegistryProvider, null, React.createElement(Probe)));
      const b = getKitRegistryBridge();
      expect(b).not.toBeNull();
      await b!.open("k-shallow", { store, kitClient });
      await waitFor(() => expect(b!.list()).toContain("k-shallow"));
      await waitFor(() => {
        expect(hasKit).toBe(true);
        expect(pkind).toBe("temporary");
        expect(shallows.some((s) => s.id === "k-shallow" && s.name === "ShallowK")).toBe(true);
      });
      unmount();
      await waitFor(() => expect(getKitRegistryBridge()).toBeNull());
    });
  });

  describe("executeSemioKitCommand moveToFolder", () => {
    it("updates quality folder on InMemoryKitStore", async () => {
      const t = new Date().toISOString();
      const kit = asKitInstance({
        id: "drag-kit",
        name: "Drag Kit",
        createdAt: t,
        updatedAt: t,
        folders: [
          { id: "folder-a", name: "Folder A" },
          { id: "folder-b", name: "Folder B" },
          { id: "folder-c", name: "Folder C", parent: { id: "folder-a" } },
        ],
        types: [{ id: "type-a", name: "Type A", folder: "folder-a", createdAt: t, updatedAt: t }],
        designs: [{ id: "design-a", name: "Design A", folder: "folder-a", pieces: [], connections: [], createdAt: t, updatedAt: t }],
        qualities: [{ id: "quality-a", name: "Quality A", key: "quality.a", folder: "folder-a" }],
        files: [{ id: "file-a", name: "mesh.glb", folder: { id: "folder-a" }, createdAt: t, updatedAt: t }],
      });
      const store = new InMemoryKitStore(kit);
      await executeSemioKitCommand(store, "semio.kit.moveToFolder", "test.moveToFolder.quality", "quality-a", "quality", "folder-b");
      const q = store.getSnapshot().kit.qualities?.find((x) => x.id === "quality-a");
      expect(q?.folder).toBe("folder-b");
    });
  });

  describe("KitStoreClient stub RPC hooks", () => {
    it("records kit command request lifecycle events from the store client", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [{ id: "d1", name: "D", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString(), pieces: [{ id: "p1", name: "P" }], connections: [] }],
      });
      const store = new InMemoryKitStore(kit);
      const listeners = new Set<(event: any) => void>();
      const emit = (event: any) => {
        for (const listener of listeners) listener(event);
      };
      const stub = {
        ...createTestKitClient(store),
        subscribe: (cb: (ev: any) => void) => {
          listeners.add(cb);
          return () => listeners.delete(cb);
        },
        clusterPieces: async () => {
          await new Promise((resolve) => setTimeout(resolve, 0));
          emit({ semioKitCommand: { requestId: "r1", commandKind: "clusterPieces", phase: "accepted" } });
          emit({ semioKitCommand: { requestId: "r1", commandKind: "clusterPieces", phase: "failed", error: { kind: "InvalidValue", message: "bad cluster" } } });
          return { ok: false, error: { kind: "InvalidValue", message: "bad cluster" }, requestId: "r1" };
        },
      } as unknown as KitStoreClient;
      let events: SchemaPropertyEvent[] = [];
      let errors: SetError[] = [];
      function Probe() {
        const { run } = useClusterPieces();
        events = useSchemaEvents({ typeName: "KitCommand" });
        errors = useSetErrors();
        const ran = React.useRef(false);
        React.useEffect(() => {
          if (ran.current) return;
          ran.current = true;
          void run("d1", ["p1"], "C");
        }, [run]);
        return null;
      }
      render(React.createElement(KitScope, { store, kitClient: stub, children: React.createElement(Probe) }));
      await waitFor(() => expect(events.some((event) => event.requestId === "r1" && event.phase === "failed")).toBe(true));
      expect(errors.some((error) => error.message === "bad cluster")).toBe(true);
    });

    it("useClusterPieces forwards failures to useSetErrors", async () => {
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        designs: [
          {
            id: "d1",
            name: "D",
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            pieces: [{ id: "p1", name: "P" }],
            connections: [],
          },
        ],
      });
      const store = new InMemoryKitStore(kit);
      const stub: KitStoreClient = {
        fetchFullKit: async () => store.getSnapshot().kit.toJSON() as KitFullDto,
        kitReadPoint: theKitReadPoint,
        getKitWriteScope: () => null,
        setKitWriteScope: () => {},
        finalizeKitWriteTransaction: async () => ({ ok: true }) as const,
        abortKitWriteTransaction: async () => ({ ok: true }) as const,
        submitChangeKitCommands: async () => ({ ok: true }) as const,
        kitGraphql: () => {
          throw new Error("no gql");
        },
        clusterPieces: async () => ({ ok: false, error: { kind: "InvalidValue", message: "stub-cluster" } }),
        dragPieces: async () => ({ ok: true }) as const,
        movePieces: async () => ({ ok: true }) as const,
        fixPieces: async () => ({ ok: true }) as const,
        flattenDesign: async () => ({ ok: true }) as const,
        expandDesign: async () => ({ ok: true }) as const,
        deleteConnection: async () => ({ ok: true }) as const,
        changePieceType: async () => ({ ok: true }) as const,
        pasteDesignSelection: async () => ({ ok: true }) as const,
        createHangingPieces: async () => ({ ok: true }) as const,
        createConnectedPiece: async () => ({ ok: true }) as const,
        createFixedPiece: async () => ({ ok: true }) as const,
        getPiecesMetadata: async () => new Map(),
        getPieces: async () => [],
        getConnections: async () => [],
        getDesigns: async () => [],
        getTypes: async () => [],
        getAuthors: async () => [],
        getKitMetadata: async () => {
          const k = store.getSnapshot().kit.toJSON() as KitFullDto;
          return { id: String(k.id ?? ""), name: String(k.name ?? "") };
        },
        undo: async () => ({ ok: true }) as const,
        redo: async () => ({ ok: true }) as const,
        canUndo: async () => false,
        canRedo: async () => false,
        backboneStatus: async () => ({ attached: false, kind: null, backboneTip: null, pendingWipCheckpoints: 0 }),
        attachBackbone: async () => ({ ok: true } as const),
        detachBackbone: async () => ({ ok: true } as const),
        listConflicts: async () => [],
        resolveConflict: async () => ({ ok: true } as const),
        syncNow: async () => ({ ok: true } as const),
        kitName: new StoreField<string>(""),
        renameKit: new StoreCommand<import("@semio/js").RenameKitCommandArgs>(async () => ({
          ok: false,
          error: { kind: "NotSupported", message: "stub" },
        })),
        readKitName: async () => "",
        readPieceFlatPlane: async () => null,
        readPieceFlatCenter: async () => null,
        readPieceParentConnectionFull: async () => null,
        readDesignIncludedDesigns: async () => [],
        readDesignClusterableGroups: async () => [],
        readDesignQualitySum: async () => 0,
        readTypeBestRepresentation: async () => null,
        readColoredConnectors: async () => [],
        readDesignReplaceableCatalogTypes: async () => [],
        readDesignReplaceableCatalogDesigns: async () => [],
        readDesignIncludedDesignIds: async () => [],
        subscribe: () => () => {},
        setKitReadPoint: () => {},
        dispose: () => {},
      } as unknown as KitStoreClient;
      let seen: SetError[] = [];
      function Probe() {
        const { run } = useClusterPieces();
        seen = useSetErrors();
        const ran = React.useRef(false);
        React.useEffect(() => {
          if (ran.current) return;
          ran.current = true;
          void run("d1", ["p1"], "C");
        }, [run]);
        return null;
      }
      render(React.createElement(KitScope, { store, kitClient: stub, children: React.createElement(Probe) }));
      await waitFor(() => expect(seen.length).toBeGreaterThan(0));
      expect(seen[0]?.message).toContain("stub-cluster");
    });
  });

  describe("kit data scope", () => {
    it("KitScope kitReadPoint prop drives setKitReadPoint and useKitReadPoint (checkpoint line)", async () => {
      const log: string[] = [];
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        types: [],
        designs: [],
      });
      const store = new InMemoryKitStore(kit);
      const base = createTestKitClient(store);
      const client: KitStoreClient = {
        ...base,
        setKitReadPoint: (s) => {
          log.push(kitReadPointKey(s));
        },
      };
      const ck: KitReadPoint = { checkpoint: { checkpointId: "cpx" } };
      let got: KitReadPoint | null = null;
      function Leaf() {
        got = useKitReadPoint();
        return null;
      }
      const tree = React.createElement(KitScope, {
        store,
        kitClient: client,
        kitReadPoint: ck,
        children: React.createElement(Leaf, null),
      });
      const { unmount } = render(tree);
      await waitFor(() => {
        if (!got || !("checkpoint" in got) || (got as { checkpoint: { checkpointId: string } }).checkpoint.checkpointId !== "cpx") {
          throw new Error("not ready");
        }
      });
      const ckKey = kitReadPointKey({ checkpoint: { checkpointId: "cpx" } });
      expect(log).toContain(ckKey);
      unmount();
    });

    it("KitScope without kitReadPoint follows KitAlternativeSelectionProvider alternative id", async () => {
      const log: string[] = [];
      const kit = asKitInstance({
        id: "k1",
        name: "K",
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        types: [],
        designs: [],
      });
      const store = new InMemoryKitStore(kit);
      const base = createTestKitClient(store);
      const client: KitStoreClient = {
        ...base,
        setKitReadPoint: (s) => {
          log.push(kitReadPointKey(s));
        },
      };
      function Probe() {
        const [, setAlt] = useKitAlternativeSelection();
        React.useEffect(() => {
          setAlt("alt-7");
        }, [setAlt]);
        useKitReadPoint();
        return null;
      }
      const tree = React.createElement(KitAlternativeSelectionProvider, {
        children: React.createElement(KitScope, {
          store,
          kitClient: client,
          children: React.createElement(Probe, null),
        }),
      });
      const { unmount } = render(tree);
      await waitFor(() => {
        expect(log).toContain(kitReadPointKey({ alternative: { alternativeId: "alt-7" } }));
      });
      unmount();
    });
  });

  describe("usePendingTriad", () => {
    it("keeps local draft and does not clear it when commit rejects", async () => {
      const triad: KitFieldBinding<string> = [
        "server",
        async (next) => {
          const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
          if (v === "reject") return { ok: false, error: { kind: "InvalidValue", message: "rejected" } } as const;
          return { ok: true } as const;
        },
        { kind: "idle", pending: 0 },
      ];
      let snap: ReturnType<typeof usePendingTriad<string>> | null = null;
      function P() {
        snap = usePendingTriad(triad);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(snap).not.toBeNull());
      await act(async () => {
        snap!.setPending("reject");
      });
      const r = await act(async () => snap!.commit());
      expect(r.ok).toBe(false);
      expect(snap!.value).toBe("reject");
    });

    it("clears draft when commit succeeds", async () => {
      const triad: KitFieldBinding<string> = [
        "server",
        async (next) => {
          const v = typeof next === "function" ? (next as (p: string) => string)("server") : next;
          return { ok: true } as const;
        },
        { kind: "idle", pending: 0 },
      ];
      let snap: ReturnType<typeof usePendingTriad<string>> | null = null;
      function P() {
        snap = usePendingTriad(triad);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(snap).not.toBeNull());
      await act(async () => {
        snap!.setPending("edited");
      });
      expect(snap!.value).toBe("edited");
      const r = await act(async () => snap!.commit());
      expect(r.ok).toBe(true);
      expect(snap!.value).toBe("server");
    });

    it("two usePendingTriad instances do not share pending state", async () => {
      const triadA: KitFieldBinding<string> = ["a", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      const triadB: KitFieldBinding<string> = ["b", async () => ({ ok: true }) as const, { kind: "idle", pending: 0 }];
      let sa: ReturnType<typeof usePendingTriad<string>> | null = null;
      let sb: ReturnType<typeof usePendingTriad<string>> | null = null;
      function P() {
        sa = usePendingTriad(triadA);
        sb = usePendingTriad(triadB);
        return null;
      }
      render(React.createElement(P));
      await waitFor(() => expect(sa && sb).toBeTruthy());
      await act(async () => {
        sa!.setPending("only-a");
        sb!.setPending("only-b");
      });
      expect(sa!.value).toBe("only-a");
      expect(sb!.value).toBe("only-b");
    });
  });
}
// #endregion ⚛️Embedded tests

//#endregion 🪁SketchpadHost

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
