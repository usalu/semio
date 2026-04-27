// #region 🧲Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// GNU LGPL-3.0 or later — semio/js: `KitStore` + opaque wire batches to `semio/rs` WASM (read shapes are not re-exported).
// #endregion 🧲Header

// #region 📥Imports
import { Subject, filter } from "rxjs";
import { z } from "zod";
// #endregion 📥Imports

// #region 🧵InlineWorker

const kitStoreWorkerSource = String.raw`
let handle = null;
function post(out) {
  self.postMessage(JSON.stringify(out));
}
self.onmessage = async (ev) => {
  let msg;
  try {
    msg = JSON.parse(ev.data);
  } catch {
    post({ op: "error", message: "invalid worker message json" });
    return;
  }
  try {
    if (msg.op === "init") {
      const mod = await import("@semio/rs-wasm");
      if (typeof mod.default === "function") await mod.default();
      if (typeof mod.boot === "function") mod.boot();
      handle = mod.KitStoreHandle.create(msg.dto);
      post({ op: "ready" });
      return;
    }
    if (!handle) {
      post({ op: "error", reqId: "op" in msg && msg.op !== "init" ? msg.reqId : undefined, message: "worker not initialized" });
      return;
    }
    if (msg.op === "execute") {
      const json = await handle.execute(msg.body);
      post({ op: "result", reqId: msg.reqId, json: String(json) });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    if (msg.op === "subscribe") {
      await handle.subscribe(msg.body, (eventJson) => {
        post({ op: "event", reqId: msg.reqId, json: String(eventJson) });
      });
      post({ op: "done", reqId: msg.reqId });
      return;
    }
    post({ op: "error", message: "unrecognized op " + (msg.op ?? "") });
  } catch (e) {
    post({ op: "error", reqId: msg?.reqId, message: String(e) });
  }
};
`;

/** @emoji 🧵 Creates the dedicated WASM worker from this file so semio/js has one source entry. */
function createKitStoreWorker(): Worker {
  const blob = new Blob([kitStoreWorkerSource], { type: "text/javascript" });
  const url = URL.createObjectURL(blob);
  const worker = new Worker(url, { type: "module" });
  URL.revokeObjectURL(url);
  return worker;
}

// #endregion 🧵InlineWorker
// #region 🔌JsonGraphQlDtoTypes

/** @emoji 🪪 Correlates kit command lifecycle events on the wire. */
export type KitCommandRequestId = string;

export type SetErrorKind =
  | "IllegalName"
  | "NameTooLong"
  | "InvalidUrl"
  | "InvalidValue"
  | "DuplicateId"
  | "NotFound"
  | "CyclicReference"
  | "PortFamilyMismatch"
  | "Readonly"
  | "Disposed"
  | "Timeout"
  | "LockPoisoned"
  | "Internal"
  | "NotSupported";

/** @emoji 🧾 Normalized set/mutation error from Rust `SetError`. */
export type SetError = { kind: SetErrorKind; message: string; field?: string; entity?: { kind: string; id: string } };

export type SetResult =
  | { ok: true; requestId?: KitCommandRequestId }
  | { ok: false; error: SetError; requestId?: KitCommandRequestId };

export type KitCommandLifecyclePhase = "accepted" | "succeeded" | "failed";

/**
 * @emoji 🧾 `KitChangeKind` on the wire (camelCase from `semio/rs`), plus any `other` label inside `other`.
 */
export type KitChangeKind =
  | "inferred"
  | "setKitMetadata"
  | "addType"
  | "removeType"
  | "modifyType"
  | "addDesign"
  | "removeDesign"
  | "modifyDesign"
  | "addPiece"
  | "removePiece"
  | "connect"
  | "disconnect"
  | "unifyCheckpoints"
  | "markRelease"
  | { readonly other: string }
  | (string & { readonly _semioExt?: 1 });

/** @emoji 🧾 GraphQL `KitChangeSemanticKind` enum (SCREAMING_SNAKE); pair with {@linkcode KitChangeKind} via {@linkcode kitChangeSemanticKindToGraphQl}. */
export type KitChangeSemanticKindGql =
  | "INFERRED"
  | "SET_KIT_METADATA"
  | "ADD_TYPE"
  | "REMOVE_TYPE"
  | "MODIFY_TYPE"
  | "ADD_DESIGN"
  | "REMOVE_DESIGN"
  | "MODIFY_DESIGN"
  | "ADD_PIECE"
  | "REMOVE_PIECE"
  | "CONNECT"
  | "DISCONNECT"
  | "UNIFY_CHECKPOINTS"
  | "MARK_RELEASE"
  | "OTHER";

/** @emoji 🧾 Maps batch `changeKind` + `changeKindOther` into {@linkcode KitChangeKind} (camelCase unit or `{ other }` for extension labels). */
export function kitChangeSemanticKindToGraphQl(
  gql: KitChangeSemanticKindGql | null | undefined,
  other: string | null | undefined,
): KitChangeKind {
  if (gql === "OTHER" || gql == null) {
    if (other != null && other.length > 0) return { other } as const;
    return "inferred";
  }
  const m: { readonly [K in Exclude<KitChangeSemanticKindGql, "OTHER">]: KitChangeKind } = {
    INFERRED: "inferred",
    SET_KIT_METADATA: "setKitMetadata",
    ADD_TYPE: "addType",
    REMOVE_TYPE: "removeType",
    MODIFY_TYPE: "modifyType",
    ADD_DESIGN: "addDesign",
    REMOVE_DESIGN: "removeDesign",
    MODIFY_DESIGN: "modifyDesign",
    ADD_PIECE: "addPiece",
    REMOVE_PIECE: "removePiece",
    CONNECT: "connect",
    DISCONNECT: "disconnect",
    UNIFY_CHECKPOINTS: "unifyCheckpoints",
    MARK_RELEASE: "markRelease",
  };
  return m[gql as Exclude<KitChangeSemanticKindGql, "OTHER">] ?? "inferred";
}

/** @emoji 🪢 Object branch for GraphQL / serde JSON trees (explicit string slots, no `Record` alias). */
export type KitJsonObjectDto = { readonly [slot: string]: KitJsonTreeDto };
/** @emoji 🪢 Recursive JSON tree from GraphQL / serde kit scalars. */
export type KitJsonTreeDto =
  | string
  | number
  | boolean
  | null
  | readonly KitJsonTreeDto[]
  | KitJsonObjectDto;

/** @emoji 🧱 Parsed JSON tree (strict surface; no open index-signature object typing or untyped values). */
export type JsonValue = string | number | boolean | null | readonly JsonValue[] | JsonObject;
/** @emoji 🧱 JSON object node (string-keyed). */
export type JsonObject = { readonly [key: string]: JsonValue };

/** @emoji 🔒 Recursive readonly DTO view for Zod-inferred and GraphQL-shaped values. */
export type ReadonlyDto<T> = T extends ReadonlyArray<infer U>
  ? ReadonlyArray<ReadonlyDto<U>>
  : T extends object
    ? { readonly [K in keyof T]: ReadonlyDto<T[K]> }
    : T;

/** @emoji 🧱 Mutable JSON object for local construction. */
type JsonObjectMutable = { [key: string]: JsonValue };
/** @emoji 🧱 Mutable `variables` map for GraphQL / kit batch (alias of the JSON builder surface). */
type GraphQlObjectMutable = JsonObjectMutable;

/** @emoji 🧾 `JSON.parse` with a {@link JsonValue} root (GraphQL transport, config knobs). */
function parseJsonValue(text: string): JsonValue {
  return JSON.parse(text) as JsonValue;
}

/** @emoji 🧾 GraphQL HTTP/FFI response envelope before unwrapping `data`. */
type KitGraphqlResponseEnvelope<TData> = Readonly<{
  data?: TData | null;
  errors?: readonly { readonly message?: string }[];
}>;

/**
 * @emoji 🧾 Tags accepted on `KitStore` control-plane batch mappers (`kitStore.batch` and {@link WasmKitStoreClient} routing).
 * @public
 */
export const SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS = {
  changeKitCommands: 1,
  changeKitWithInverse: 1,
  undo: 1,
  redo: 1,
  clusterPieces: 1,
  dragPieces: 1,
  movePieces: 1,
  fixPieces: 1,
  flattenDesign: 1,
  expandDesign: 1,
  deleteConnection: 1,
  changePieceType: 1,
  createHangingPieces: 1,
  createConnectedPiece: 1,
  createFixedPiece: 1,
  pasteDesignSelection: 1,
  listConflicts: 1,
  backboneStatus: 1,
  attachBackbone: 1,
  detachBackbone: 1,
  resolveConflict: 1,
  syncNow: 1,
} as const;
export type SemioKitStoreControlCommandName = keyof typeof SEMIO_KIT_STORE_CONTROL_COMMAND_KINDS;

export type KitCommandLifecycleEvent = {
  semioKitCommand: {
    requestId: KitCommandRequestId;
    commandKind: SemioKitStoreControlCommandName | (string & { readonly _semioStoreControlLabel?: 1 });
    phase: KitCommandLifecyclePhase;
    result?: KitJsonTreeDto;
    error?: SetError;
  };
};

/** @emoji 🧭 Backbone / conflict wire shapes (serde-tagged, matches `kit_backbone_wire` in `semio/rs`). */
export type BackboneConfig =
  | { readonly Memory: null }
  | { readonly Dev: { readonly path: string } }
  | { readonly Local: { readonly folder: string } }
  | { readonly Remote: { readonly url: string; readonly sessionId: string } };
export type BackboneStatusDto = {
  readonly attached: boolean;
  readonly kind?: string | null;
  readonly backboneTip?: string | null;
  readonly pendingWipCheckpoints: number;
};
/** @emoji 🧾 GraphQL `ConflictResolutionBatchInput` (matches `semio/graphql/schema.graphql`). */
export type ConflictResolution = "DROP_WIP" | "FORCE_OVERWRITE_BACKBONE";
export type KitCheckpointDto = KitJsonObjectDto;
export type KitConflict = {
  id: string;
  wipCheckpoint: KitCheckpointDto;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
};

/** @emoji 🧾 One read command in a `KitStore.read` batch (matches `semio/rs` read kit wire, serde camelCase). */
export type ReadPieceCommand =
  | { readonly readPieceFlatPlaneCommand: null }
  | { readonly readPieceFlatCenterCommand: null }
  | { readonly readPieceParentConnectionFullCommand: null };

export type ReadDesignCommand =
  | { readonly readDesignPiecesFullCommand: null }
  | { readonly readDesignConnectionsFullCommand: null }
  | { readonly readDesignPieceCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadPieceCommand> } }
  | { readonly readDesignClusterableGroupsCommand: { readonly selection: ReadonlyArray<KitIdDto> } }
  | { readonly readDesignIncludedDesignsCommand: null }
  | { readonly readDesignQualitySumCommand: { readonly qualityId: KitIdDto } }
  | { readonly readDesignReplaceableCatalogCommand: { readonly selection: ReadonlyArray<KitIdDto> } }
  | { readonly readDesignIncludedDesignIdsCommand: null };

export type ReadTypeCommand = { readonly readTypeBestRepresentationCommand: { readonly tagIds: ReadonlyArray<string> } };

export type ReadKitCommand =
  | { readonly readKitFullCommand: null }
  | { readonly readKitShallowCommand: null }
  | { readonly readKitMetadataCommand: null }
  | { readonly readKitTypeIdsCommand: null }
  | { readonly readKitDesignIdsCommand: null }
  | { readonly readKitTypesMetadataCommand: null }
  | { readonly readKitDesignsMetadataCommand: null }
  | { readonly readKitTypesShallowCommand: null }
  | { readonly readKitDesignsShallowCommand: null }
  | { readonly readKitAuthorsShallowCommand: null }
  | { readonly readKitColoredConnectorsCommand: null }
  | { readonly readKitDesignCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadDesignCommand> } }
  | { readonly readKitTypeCommands: { readonly id: KitIdDto; readonly commands: ReadonlyArray<ReadTypeCommand> } };

/**
 * @emoji 🧭 Which materialized kit view read commands run against (matches `semio/rs` `KitReadScope` / GraphQL oneof `KitReadScopeInput`).
 * Use `theKitReadScope` for the main live line.
 */
export type KitReadScope =
  | { readonly theKit: null }
  | { readonly checkpoint: { readonly checkpointId: string } }
  | { readonly alternative: { readonly alternativeId: string } }
  | { readonly draft: { readonly sessionId: string; readonly draftId: string } }
  | { readonly transaction: { readonly sessionId: string; readonly draftId: string; readonly transactionId: string } };

/** @emoji 🧾 GraphQL `KitReadScopeInput` (variables payload for `Query.kit(scope: …)`). */
export type KitReadScopeInputGraphQL =
  | { readonly theKit: { readonly confirm: true } }
  | { readonly checkpoint: { readonly checkpointId: string } }
  | { readonly alternative: { readonly alternativeId: string } }
  | { readonly draft: { readonly sessionId: string; readonly draftId: string } }
  | { readonly transaction: { readonly sessionId: string; readonly draftId: string; readonly transactionId: string } };

/** @emoji 🧭 Main committed kit line (default read scope). */
export const theKitReadScope: KitReadScope = { theKit: null };

/** @emoji 🧭 Read scope used for materialized GraphQL reads on a {@link KitStoreClient}. */
export function getKitClientReadScope(client: { readonly kitReadScope?: KitReadScope }): KitReadScope {
  return client.kitReadScope ?? theKitReadScope;
}

/** @emoji 🧪 Stable string for cache keys (sorted JSON of the GraphQL oneof payload). */
export function kitReadScopeKey(scope: KitReadScope): string {
  return JSON.stringify(kitReadScopeToGraphQLInput(scope));
}

/** @emoji 🧾 `KitReadScopeInput` object for async-graphql (camelCase, `theKit` uses `ConfirmOnlyInput`). */
export function kitReadScopeToGraphQLInput(scope: KitReadScope): KitReadScopeInputGraphQL {
  if ("theKit" in scope) return { theKit: { confirm: true } };
  if ("checkpoint" in scope) return { checkpoint: { checkpointId: scope.checkpoint.checkpointId } };
  if ("alternative" in scope) return { alternative: { alternativeId: scope.alternative.alternativeId } };
  if ("draft" in scope) return { draft: { sessionId: scope.draft.sessionId, draftId: scope.draft.draftId } };
  return {
    transaction: {
      sessionId: scope.transaction.sessionId,
      draftId: scope.transaction.draftId,
      transactionId: scope.transaction.transactionId,
    },
  };
}

function isTheKitReadScope(s: KitReadScope): boolean {
  return "theKit" in s;
}

// #region 🔖KitWriteScope
/** @emoji 🧭 Active VCS session/draft/open-transaction anchor for kit control-plane `kitStore.batch` writes. */
export type KitWriteScope = { readonly sessionId: string; readonly draftId: string; readonly transactionId: string };

function __normKitStoreBatchKind(k: JsonValue | undefined): string {
  const s =
    k === null || k === undefined
      ? ""
      : typeof k === "string"
        ? k
        : typeof k === "number" || typeof k === "boolean" || typeof k === "bigint"
          ? String(k)
          : "UNKNOWN";
  return s
    .replace(/([a-z])([A-Z])/g, "$1_$2")
    .replace(/[\s-]+/g, "_")
    .toUpperCase();
}

function __coerceAxisComponent(v: KitJsonTreeDto | undefined): number {
  if (v === undefined) return Number.NaN;
  if (typeof v === "number" && !Number.isNaN(v)) return v;
  if (typeof v === "string") return Number(v);
  return Number.NaN;
}

function __vec3(obj: KitJsonTreeDto | null | undefined): { x: number; y: number; z: number } | null {
  if (!isJsonObjectNode(obj)) return null;
  const x = __coerceAxisComponent(obj["x"]);
  const y = __coerceAxisComponent(obj["y"]);
  const z = __coerceAxisComponent(obj["z"]);
  if (!Number.isFinite(x) || !Number.isFinite(y) || !Number.isFinite(z)) return null;
  return { x, y, z };
}

/** @emoji 🧾 Maps a loose plane DTO into GraphQL `PlaneInputBatch` (camelCase axes). */
function __kitPlaneToBatchInput(plane: KitJsonTreeDto | null | undefined): { origin: { x: number; y: number; z: number }; xAxis: { x: number; y: number; z: number }; yAxis: { x: number; y: number; z: number } } | null {
  if (!isJsonObjectNode(plane)) return null;
  const p = plane as KitJsonObjectDto;
  const origin = p["origin"] ?? p["Origin"];
  const xa = p["xAxis"] ?? p["x_axis"] ?? p["XAxis"];
  const ya = p["yAxis"] ?? p["y_axis"] ?? p["YAxis"];
  const o = __vec3(origin);
  const xAxis = __vec3(xa);
  const yAxis = __vec3(ya);
  if (!o || !xAxis || !yAxis) return null;
  return { origin: o, xAxis, yAxis };
}

// #endregion 🔖KitWriteScope

// #region 🔖ReadBatchAndKitRead
/** @emoji 🧾 One `design.flattenMap` row (`DesignFlattenMapEntryObject`). */
export type DesignFlattenMapEntryDto = Readonly<{
  readonly pieceId: string;
  readonly plane: PlanePlain;
  readonly center: PointPlain;
}>;

/** @emoji 🧾 One `design.piecePlacement` row (`PiecePlacementRowObject`). */
export type PiecePlacementRowDto = Readonly<{
  readonly pieceId: string;
  readonly plane: PlanePlain;
  readonly center: PointPlain;
  readonly fixedPieceId: string;
  readonly parentPieceId: string | null;
  readonly depth: number;
  readonly path: readonly string[];
}>;

/** @emoji 🧾 One `kit.coloredConnectors` row (`KitColoredConnectorObject`). */
export type KitColoredConnectorRowDto = Readonly<{
  readonly typeId: TypeIdDto;
  readonly connectorId: ConnectorIdDto;
  readonly color: string;
}>;

/** @emoji 🧾 One `design.includedDesigns` entry (`IncludedDesignObject`). */
export type IncludedDesignInfoDto = Readonly<{
  readonly id: string;
  readonly designId: string;
  readonly connectionKind: string;
  readonly center: PointPlain | null;
  readonly plane: PlanePlain | null;
  readonly externalConnections?: readonly ConnectionPlain[];
}>;

/** @emoji 🧾 `KitMetadataObject` root fields from GraphQL. */
export type KitMetadataDto = Readonly<{
  readonly id: string;
  readonly name: string;
  readonly description?: string | null;
  readonly icon?: string | null;
  readonly image?: string | null;
  readonly preview?: string | null;
  readonly remote?: string | null;
  readonly homepage?: string | null;
  readonly license?: string | null;
  readonly uri?: string | null;
  readonly created?: string | null;
  readonly updated?: string | null;
  readonly version?: string | null;
}>;
// #endregion 🔖ReadBatchAndKitRead

/** @emoji 🧾 Batch input for {@link KitStore.read} (per-command, same for all entries in a batch). */
export type ReadBatch = readonly ReadKitCommand[];

/** @emoji 🧾 One entry in a {@link ReadBatch} (alias for consumers that say “read wire item”). */
export type ReadBatchItem = ReadKitCommand;

export type ReadPieceCommandOutput =
  | { readonly readPieceFlatPlaneCommand: { readonly flatPlane: PlanePlain | null } }
  | { readonly readPieceFlatCenterCommand: { readonly flatCenter: CoordinatePlain | null } }
  | { readonly readPieceParentConnectionFullCommand: { readonly connection: ConnectionPlain | null } };

export type ReadTypeCommandOutput = {
  readonly readTypeBestRepresentationCommand: { readonly representation: RepresentationPlain | null };
};

export type ReadDesignCommandOutput =
  | { readonly readDesignPiecesFullCommand: { readonly pieces: readonly PiecePlain[] } }
  | { readonly readDesignConnectionsFullCommand: { readonly connections: readonly ConnectionPlain[] } }
  | { readonly readDesignPieceCommands: { readonly results: readonly ReadPieceCommandOutput[] } }
  | { readonly readDesignClusterableGroupsCommand: { readonly groups: readonly (readonly KitIdDto[])[] } }
  | { readonly readDesignIncludedDesignsCommand: { readonly designs: readonly IncludedDesignInfoDto[] } }
  | { readonly readDesignQualitySumCommand: { readonly sum: number } }
  | { readonly readDesignReplaceableCatalogCommand: { readonly types: readonly KitIdDto[]; readonly designs: readonly KitIdDto[] } }
  | { readonly readDesignIncludedDesignIdsCommand: { readonly designIds: readonly string[] } };

/** @emoji 🧾 One command’s read output object (per-command payload shape from `semio/rs` GraphQL). */
export type ReadKitCommandOutput =
  | { readonly readKitFullCommand: { readonly full: KitFullDto } }
  | { readonly readKitShallowCommand: { readonly types: readonly TypeShallow[]; readonly designs: readonly DesignShallow[] } }
  | { readonly readKitTypeIdsCommand: { readonly typeIds: readonly KitIdDto[] } }
  | { readonly readKitDesignIdsCommand: { readonly designIds: readonly KitIdDto[] } }
  | { readonly readKitTypesMetadataCommand: { readonly types: readonly TypeMetadataDto[] } }
  | { readonly readKitDesignsMetadataCommand: { readonly designs: readonly DesignMetadataDto[] } }
  | { readonly readKitTypesShallowCommand: { readonly types: readonly TypeShallow[] } }
  | { readonly readKitDesignsShallowCommand: { readonly designs: readonly DesignShallow[] } }
  | { readonly readKitAuthorsShallowCommand: { readonly authors: readonly AuthorMetadataDto[] } }
  | { readonly readKitMetadataCommand: { readonly metadata: KitMetadataDto | null } }
  | { readonly readKitColoredConnectorsCommand: { readonly rows: readonly KitColoredConnectorRowDto[] } }
  | { readonly readKitDesignCommands: { readonly results: readonly ReadDesignCommandOutput[] } }
  | { readonly readKitTypeCommands: { readonly results: readonly ReadTypeCommandOutput[] } };

/** @emoji 🧾 Batch output from {@link KitStore.read}. */
export type ReadBatchResult = readonly ReadKitCommandOutput[];

/**
 * @emoji 📣 GraphQL `KitEvent` scalar + synthetic invalidation rows used by {@link WasmKitStoreClient};
 * field-level rows remain {@link KitJsonObjectDto}; classified kit mutations are top-level keys (`renamedDesign`, `changedKit`, …).
 */
export type KitEvent = Readonly<
  | { readonly Changed: null }
  | { readonly ValidationInvalidated: null }
  | KitCommandLifecycleEvent
  | KitClassifiedMutationEvent
  | KitJsonObjectDto
>;

/** @emoji 🧾 Optional filter for {@link KitStore.subscribeFiltered}. */
export type KitEventFilter = (event: KitEvent) => boolean;

/** @emoji 🧾 Unsubscribe handle returned by {@link KitStore.subscribe}. */
export type Unsubscribe = () => void;

export type KitCommandReceipt = { requestId: KitCommandRequestId; commandKind: string; accepted: boolean };

export type KitStoreOpenOptions = {
  wasmSpecifier?: string;
  timeoutMs?: number;
  /** Optional worker factory (tests); defaults to the inline module worker defined in this file. */
  workerFactory?: () => Worker;
};

// #region 🔖ChangeKitCommand
/** @emoji 🧾 `ChangePieceCommand` JSON (externally tagged, camelCase variant keys) for `kitStore.batch` live `changeKitCommands` (or `ChangeKitCommand` GraphQL scalars). */
export type ChangePieceCommand =
  | { readonly name: { readonly name?: string | null } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly plane: { readonly plane?: KitJsonTreeDto | null } }
  | { readonly center: { readonly center?: KitJsonTreeDto | null } }
  | { readonly scale: { readonly scale?: number | null } }
  | { readonly mirrorPlane: { readonly mirrorPlane?: KitJsonTreeDto | null } }
  | { readonly hidden: { readonly hidden?: boolean | null } }
  | { readonly locked: { readonly locked?: boolean | null } }
  | { readonly color: { readonly color?: string | null } }
  | { readonly type: { readonly typeId?: KitIdDto | null } }
  | { readonly addProp: { readonly prop: PropPlain } }
  | { readonly removeProp: { readonly propId: KitIdDto } }
  | { readonly addAttribute: { readonly attribute: AttributePlain } }
  | { readonly removeAttribute: { readonly id: KitIdDto } };

/** @emoji 🧾 `ChangeConnectionCommand` JSON for nested design commands. */
export type ChangeConnectionCommand =
  | { readonly gap: { readonly value?: number | null } }
  | { readonly shift: { readonly value?: number | null } }
  | { readonly rise: { readonly value?: number | null } }
  | { readonly rotation: { readonly value?: number | null } }
  | { readonly turn: { readonly value?: number | null } }
  | { readonly tilt: { readonly value?: number | null } }
  | { readonly x: { readonly value?: number | null } }
  | { readonly y: { readonly value?: number | null } }
  | { readonly description: { readonly value?: string | null } }
  | { readonly addConnectionAttribute: { readonly attribute: AttributePlain } }
  | { readonly removeConnectionAttribute: { readonly id: KitIdDto } };

/** @emoji 🧾 Nested `ChangeDesignCommand` entries. */
export type ChangeDesignCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly addPiece: { readonly piece: PiecePlain } }
  | { readonly removePiece: { readonly pieceId: KitIdDto } }
  | { readonly addConnection: { readonly connection: ConnectionPlain } }
  | { readonly removeConnection: { readonly connectionId: KitIdDto } }
  | { readonly changePieceCommands: { readonly pieceId: KitIdDto; readonly commands: readonly ChangePieceCommand[] } }
  | { readonly changeConnectionCommands: { readonly connectionId: KitIdDto; readonly commands: readonly ChangeConnectionCommand[] } };

/** @emoji 🧾 Nested `ChangeTypeCommand` entries used by stores / React. */
export type ChangeTypeCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly stock: { readonly stock?: number | null } }
  | { readonly typeVirtual: { readonly value?: boolean | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly addRepresentation: { readonly representation: RepresentationPlain } }
  | { readonly removeRepresentation: { readonly id: KitIdDto } }
  | { readonly addConnector: { readonly connector: ConnectorPlain } }
  | { readonly removeConnector: { readonly connectorId: KitIdDto } }
  | { readonly addTypeProp: { readonly prop: PropPlain } }
  | { readonly removeTypeProp: { readonly propId: KitIdDto } };

/** @emoji 🧾 `ChangeFamilyCommand` JSON. */
export type ChangeFamilyCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } };

export type ChangeFileCommand =
  | { readonly url: { readonly url: string } }
  | { readonly mime: { readonly mime?: string | null } }
  | { readonly size: { readonly size?: number | null } }
  | { readonly hash: { readonly hash?: string | null } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly created: { readonly created?: string | null } }
  | { readonly updated: { readonly updated?: string | null } };

export type ChangeFolderCommand =
  | { readonly path: { readonly path: string } }
  | { readonly description: { readonly description?: string | null } };

export type ChangeAuthorCommand =
  | { readonly name: { readonly name: string } }
  | { readonly email: { readonly email: string } }
  | { readonly role: { readonly role?: string | null } }
  | { readonly rank: { readonly rank?: number | null } };

export type ChangeConceptCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly order: { readonly order?: number | null } };

export type ChangeTagCommand =
  | { readonly name: { readonly name: string } }
  | { readonly order: { readonly order?: number | null } };

export type ChangeKitQualityCommand =
  | { readonly key: { readonly key: string } }
  | { readonly value: { readonly value?: string | null } }
  | { readonly unit: { readonly unit?: string | null } }
  | { readonly definition: { readonly definition?: string | null } }
  | { readonly description: { readonly description?: string | null } };

export type ChangePortCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } };

/** @emoji 🧾 Top-level `ChangeKitCommand` JSON for `changeKitCommands` batch variables. */
export type ChangeKitCommand =
  | { readonly name: { readonly name: string } }
  | { readonly description: { readonly description?: string | null } }
  | { readonly icon: { readonly icon?: string | null } }
  | { readonly image: { readonly image?: string | null } }
  | { readonly preview: { readonly preview?: string | null } }
  | { readonly remote: { readonly remote?: string | null } }
  | { readonly homepage: { readonly homepage?: string | null } }
  | { readonly license: { readonly license?: string | null } }
  | { readonly uri: { readonly uri?: string | null } }
  | { readonly created: { readonly created?: string | null } }
  | { readonly updated: { readonly updated?: string | null } }
  | { readonly version: { readonly version?: string | null } }
  | { readonly addType: { readonly type: TypePlain } }
  | { readonly removeType: { readonly typeId: KitIdDto } }
  | { readonly addDesign: { readonly design: DesignPlain } }
  | { readonly removeDesign: { readonly designId: KitIdDto } }
  | { readonly changeDesignCommands: { readonly designId: KitIdDto; readonly commands: readonly ChangeDesignCommand[] } }
  | { readonly changeTypeCommands: { readonly typeId: KitIdDto; readonly commands: readonly ChangeTypeCommand[] } }
  | { readonly changeFamilyCommands: { readonly familyId: KitIdDto; readonly commands: readonly ChangeFamilyCommand[] } }
  | { readonly changeFileCommands: { readonly fileId: KitIdDto; readonly commands: readonly ChangeFileCommand[] } }
  | { readonly changeFolderCommands: { readonly folderId: KitIdDto; readonly commands: readonly ChangeFolderCommand[] } }
  | { readonly changeAuthorCommands: { readonly authorId: KitIdDto; readonly commands: readonly ChangeAuthorCommand[] } }
  | { readonly changeConceptCommands: { readonly conceptId: KitIdDto; readonly commands: readonly ChangeConceptCommand[] } }
  | { readonly changeTagCommands: { readonly tagId: KitIdDto; readonly commands: readonly ChangeTagCommand[] } }
  | { readonly changeKitQualityCommands: { readonly qualityId: KitIdDto; readonly commands: readonly ChangeKitQualityCommand[] } }
  | { readonly changeKitPortCommands: { readonly portId: KitIdDto; readonly commands: readonly ChangePortCommand[] } }
  | { readonly addFamily: { readonly family: FamilyPlain } }
  | { readonly removeFamily: { readonly familyId: KitIdDto } }
  | { readonly addFolder: { readonly folder: FolderPlain } }
  | { readonly removeFolder: { readonly folderId: KitIdDto } }
  | { readonly addAuthor: { readonly author: AuthorPlain } }
  | { readonly removeAuthor: { readonly authorId: KitIdDto } }
  | { readonly addConcept: { readonly concept: ConceptPlain } }
  | { readonly removeConcept: { readonly conceptId: KitIdDto } }
  | { readonly addTag: { readonly tag: TagPlain } }
  | { readonly removeTag: { readonly tagId: KitIdDto } }
  | { readonly addQuality: { readonly quality: QualityPlain } }
  | { readonly removeQuality: { readonly qualityId: KitIdDto } }
  | { readonly addKitProp: { readonly prop: PropPlain } }
  | { readonly removeKitProp: { readonly propId: KitIdDto } }
  | { readonly addKitAttribute: { readonly attribute: AttributePlain } }
  | { readonly removeKitAttribute: { readonly id: KitIdDto } }
  | { readonly addFile: { readonly file: FilePlain } }
  | { readonly removeFile: { readonly fileId: KitIdDto } }
  | { readonly replaceKitFromFullDto: { readonly dto: KitFullDto } }
  | { readonly clusterPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly clusterName: string } }
  | { readonly dragPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly du: number; readonly dv: number } }
  | { readonly movePieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[]; readonly gap: number; readonly shift: number; readonly rise: number } }
  | { readonly fixPieces: { readonly designId: KitIdDto; readonly pieceIds: readonly string[] } }
  | { readonly flattenDesign: { readonly designId: KitIdDto } }
  | { readonly expandNestedDesign: { readonly parentDesignId: KitIdDto; readonly nestedDesignId: KitIdDto } }
  | { readonly deleteConnection: { readonly designId: KitIdDto; readonly connectionId: KitIdDto } }
  | { readonly changePieceKind: { readonly designId: KitIdDto; readonly pieceId: KitIdDto; readonly newTypeId: KitIdDto } };

/**
 * @public GraphQL `variables` map for `kitStore.batch` (kit JSON object trees, camelCase; matches {@link GraphQlObjectMutable} construction in this module).
 */
export type GraphQlVariables = KitJsonObjectDto;

/** @emoji 🧾 True for object-shaped JSON / kit tree nodes (excludes arrays and `null`). */
function isJsonObjectNode(
  v: JsonValue | KitJsonTreeDto | null | undefined,
): v is GraphQlVariables | JsonObject | KitJsonObjectDto {
  return v != null && typeof v === "object" && !Array.isArray(v);
}

/** @emoji 🧾 GraphQL `ConflictBatchRecord` row (matches `semio/graphql/schema.graphql`). */
export type ConflictBatchRecord = Readonly<{
  id: string;
  backboneTip?: string | null;
  reason: string;
  createdAt: string;
}>;

/** @emoji 🧾 One row from GraphQL `KitStoreBatchResult` (camelCase wire). */
export type KitStoreBatchResultRow = Readonly<{
  kind: string;
  ok?: boolean | null;
  sessionId?: string | null;
  draftId?: string | null;
  transactionId?: string | null;
  changeKind?: KitChangeSemanticKindGql | null;
  changeKindOther?: string | null;
  inverse?: readonly ChangeKitCommand[] | null;
  conflicts?: readonly ConflictBatchRecord[] | null;
  backbone?: { attached: boolean; kind?: string | null; tip?: string | null } | null;
}>;

/** @emoji 🧾 Forward + inverse command atoms on the subscription bus (`KitChange` from `semio/rs`). */
export type KitChange = Readonly<{
  readonly forward: readonly ChangeKitCommand[];
  readonly inverse: readonly ChangeKitCommand[];
  readonly kind?: KitChangeKind;
  readonly author?: string | null;
  readonly time?: string | null;
}>;

/** @emoji 🧾 Payload for {@link KitEvent} `renamedDesign` (camelCase from `semio/rs`). */
export type RenamedDesignKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `renamedType`. */
export type RenamedTypeKitEvent = Readonly<{ readonly typeId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `draggedFlatCenterPiece`. */
export type DraggedFlatCenterPieceKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `movedPiecesFlatCenter`. */
export type MovedPiecesFlatCenterKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `clusteredPieces`. */
export type ClusteredPiecesKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `fixedPiecesFlatCenter`. */
export type FixedPiecesFlatCenterKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceIds: readonly string[];
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `flattenedDesign`. */
export type FlattenedDesignKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `expandedNestedDesign`. */
export type ExpandedNestedDesignKitEvent = Readonly<{
  readonly parentDesignId: string;
  readonly nestedDesignId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `deletedConnection`. */
export type DeletedConnectionKitEvent = Readonly<{
  readonly designId: string;
  readonly connectionId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedPieceKind`. */
export type ChangedPieceKindKitEvent = Readonly<{
  readonly designId: string;
  readonly pieceId: string;
  readonly change: KitChange;
}>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedDesignCommands`. */
export type ChangedDesignCommandsKitEvent = Readonly<{ readonly designId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedTypeCommands`. */
export type ChangedTypeCommandsKitEvent = Readonly<{ readonly typeId: string; readonly change: KitChange }>;
/** @emoji 🧾 Payload for {@link KitEvent} `changedKit` (fallback classified change). */
export type ChangedKitEvent = Readonly<{ readonly change: KitChange }>;

/** @emoji 🧾 Classified kit mutation row: exactly one variant key plus payload (same wire as `semio/rs` `KitEvent`). */
export type KitClassifiedMutationEvent = Readonly<
  | { readonly renamedDesign: RenamedDesignKitEvent }
  | { readonly renamedType: RenamedTypeKitEvent }
  | { readonly draggedFlatCenterPiece: DraggedFlatCenterPieceKitEvent }
  | { readonly movedPiecesFlatCenter: MovedPiecesFlatCenterKitEvent }
  | { readonly clusteredPieces: ClusteredPiecesKitEvent }
  | { readonly fixedPiecesFlatCenter: FixedPiecesFlatCenterKitEvent }
  | { readonly flattenedDesign: FlattenedDesignKitEvent }
  | { readonly expandedNestedDesign: ExpandedNestedDesignKitEvent }
  | { readonly deletedConnection: DeletedConnectionKitEvent }
  | { readonly changedPieceKind: ChangedPieceKindKitEvent }
  | { readonly changedDesignCommands: ChangedDesignCommandsKitEvent }
  | { readonly changedTypeCommands: ChangedTypeCommandsKitEvent }
  | { readonly changedKit: ChangedKitEvent }
>;

const __KIT_CLASSIFIED_MUTATION_KEYS = [
  "renamedDesign",
  "renamedType",
  "draggedFlatCenterPiece",
  "movedPiecesFlatCenter",
  "clusteredPieces",
  "fixedPiecesFlatCenter",
  "flattenedDesign",
  "expandedNestedDesign",
  "deletedConnection",
  "changedPieceKind",
  "changedDesignCommands",
  "changedTypeCommands",
  "changedKit",
] as const;

/** @emoji 🧾 True when {@link KitEvent} is a {@link KitClassifiedMutationEvent}. */
export function isKitClassifiedMutationEvent(ev: KitEvent): ev is KitClassifiedMutationEvent {
  if (typeof ev !== "object" || ev === null) return false;
  for (const k of __KIT_CLASSIFIED_MUTATION_KEYS) {
    if (k in ev) return true;
  }
  return false;
}

/** @emoji 🧾 Wrap nested piece commands under one design id. */
export function kitChangeDesignPiece(
  designId: string,
  pieceId: string,
  commands: readonly ChangePieceCommand[],
): ChangeKitCommand {
  return {
    changeDesignCommands: {
      designId: { id: designId },
      commands: [{ changePieceCommands: { pieceId: { id: pieceId }, commands: [...commands] } }],
    },
  };
}

/** @emoji 🧾 Wrap nested connection commands under one design id. */
export function kitChangeDesignConnection(
  designId: string,
  connectionId: string,
  commands: readonly ChangeConnectionCommand[],
): ChangeKitCommand {
  return {
    changeDesignCommands: {
      designId: { id: designId },
      commands: [{ changeConnectionCommands: { connectionId: { id: connectionId }, commands: [...commands] } }],
    },
  };
}

const __kid = (x: string): { readonly id: string } => ({ id: x });

/** @emoji 🧾 Maps schema/UI data keys onto connection wire keys (`u`→`x`, `v`→`y`). */
export function connectionDiffKeyForDataKey(dataKey: string): string {
  if (dataKey === "u") return "x";
  if (dataKey === "v") return "y";
  return dataKey;
}

/** @emoji 🧾 UI/schema partial for piece field writes (maps to {@link ChangePieceCommand}). */
export type PieceFieldPatchInput = Readonly<{
  name?: string | null;
  description?: string | null;
  plane?: KitJsonTreeDto;
  center?: KitJsonTreeDto;
  scale?: number | string | null;
  mirrorPlane?: KitJsonTreeDto;
  hidden?: boolean;
  isHidden?: boolean;
  locked?: boolean;
  isLocked?: boolean;
  color?: string | null;
  type?: string | KitJsonObjectDto | null;
}>;

/** @emoji 🧾 UI/schema partial for connection field writes. */
export type ConnectionFieldPatchInput = Readonly<{
  gap?: number | string | null;
  shift?: number | string | null;
  rise?: number | string | null;
  rotation?: number | string | null;
  turn?: number | string | null;
  tilt?: number | string | null;
  x?: number | string | null;
  y?: number | string | null;
  u?: number | string | null;
  v?: number | string | null;
  description?: string | null;
}>;

/** @emoji 🧾 Value bucket for `buildSchemaEntityChangeCommands` (schema hooks). */
export type SchemaEntityFieldValue = KitJsonTreeDto | string | number | boolean | null | object;

/** @emoji 🧾 Converts a piece field patch into nested `changePieceCommands` wire entries. */
export function piecePatchToChangeCommands(patch: PieceFieldPatchInput): ChangePieceCommand[] {
  const out: ChangePieceCommand[] = [];
  if ("name" in patch) out.push({ name: { name: patch.name == null ? null : String(patch.name) } });
  if ("description" in patch) out.push({ description: { description: patch.description == null ? null : String(patch.description) } });
  if ("plane" in patch) out.push({ plane: { plane: patch.plane } });
  if ("center" in patch) out.push({ center: { center: patch.center } });
  if ("scale" in patch) out.push({ scale: { scale: typeof patch.scale === "number" ? patch.scale : Number(patch.scale) } });
  if ("mirrorPlane" in patch) out.push({ mirrorPlane: { mirrorPlane: patch.mirrorPlane } });
  if ("hidden" in patch) out.push({ hidden: { hidden: Boolean(patch.hidden) } });
  if ("isHidden" in patch) out.push({ hidden: { hidden: Boolean(patch.isHidden) } });
  if ("locked" in patch) out.push({ locked: { locked: Boolean(patch.locked) } });
  if ("isLocked" in patch) out.push({ locked: { locked: Boolean(patch.isLocked) } });
  if ("color" in patch) out.push({ color: { color: patch.color == null ? null : String(patch.color) } });
  if ("type" in patch) {
    const t = patch.type;
    const tid = t && typeof t === "object" && t !== null && "id" in t ? String((t as { id: string }).id) : String(t);
    out.push({ type: { typeId: { id: tid } } });
  }
  return out;
}

/** @emoji 🧾 Converts a connection field patch into nested `changeConnectionCommands` wire entries. */
export function connectionPatchToChangeCommands(patch: ConnectionFieldPatchInput): ChangeConnectionCommand[] {
  const out: ChangeConnectionCommand[] = [];
  const num = (v: string | number | null | undefined) => (typeof v === "number" && !Number.isNaN(v) ? v : Number(v));
  const opt = (v: string | number | null | undefined): number | null => (v == null ? null : num(v));
  if ("gap" in patch) out.push({ gap: { value: opt(patch.gap) } });
  if ("shift" in patch) out.push({ shift: { value: opt(patch.shift) } });
  if ("rise" in patch) out.push({ rise: { value: opt(patch.rise) } });
  if ("rotation" in patch) out.push({ rotation: { value: opt(patch.rotation) } });
  if ("turn" in patch) out.push({ turn: { value: opt(patch.turn) } });
  if ("tilt" in patch) out.push({ tilt: { value: opt(patch.tilt) } });
  if ("x" in patch) out.push({ x: { value: opt(patch.x) } });
  if ("y" in patch) out.push({ y: { value: opt(patch.y) } });
  if ("u" in patch) out.push({ x: { value: opt(patch.u) } });
  if ("v" in patch) out.push({ y: { value: opt(patch.v) } });
  if ("description" in patch) out.push({ description: { value: patch.description == null ? null : String(patch.description) } });
  return out;
}

/**
 * @emoji 🧾 Maps a schema entity + field to `changeKitCommands` for `submitChangeKitCommands` (React + kit store).
 * `designId` is required for Piece/Connection; otherwise pass `null`.
 */
export function buildSchemaEntityChangeCommands(
  kind: string,
  id: string,
  field: string,
  value: unknown,
  designId: string | null,
): readonly ChangeKitCommand[] {
  const valueCast = value as SchemaEntityFieldValue;
  void valueCast;
  switch (kind) {
    case "Kit": {
      if (field === "name") return [{ name: { name: String(value) } } as const];
      if (field === "description")
        return [{ description: { description: value == null ? null : String(value) } } as const];
      if (field === "icon") return [{ icon: { icon: (value as string) ?? null } } as const];
      if (field === "image") return [{ image: { image: (value as string) ?? null } } as const];
      if (field === "homepage") return [{ homepage: { homepage: (value as string) ?? null } } as const];
      if (field === "license") return [{ license: { license: (value as string) ?? null } } as const];
      if (field === "version" || field === "release") return [{ version: { version: (value as string) ?? null } } as const];
      if (field === "preview") return [{ preview: { preview: (value as string) ?? null } } as const];
      if (field === "remote") return [{ remote: { remote: (value as string) ?? null } } as const];
      if (field === "uri") return [{ uri: { uri: (value as string) ?? null } } as const];
      if (field === "created" || field === "createdAt") return [{ created: { created: (value as string) ?? null } } as const];
      if (field === "updated" || field === "updatedAt") return [{ updated: { updated: (value as string) ?? null } } as const];
      return [];
    }
    case "Type": {
      const inner = oneChangeTypeCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeTypeCommands: { typeId: __kid(id), commands: [inner] } } as const];
    }
    case "Design": {
      const inner = oneChangeDesignCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeDesignCommands: { designId: __kid(id), commands: [inner] } } as const];
    }
    case "Author": {
      const inner = oneChangeAuthorCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeAuthorCommands: { authorId: __kid(id), commands: [inner] } } as const];
    }
    case "Tag": {
      const inner = oneChangeTagCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeTagCommands: { tagId: __kid(id), commands: [inner] } } as const];
    }
    case "File": {
      const inner = oneChangeFileCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFileCommands: { fileId: __kid(id), commands: [inner] } } as const];
    }
    case "Folder": {
      const inner = oneChangeFolderCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFolderCommands: { folderId: __kid(id), commands: [inner] } } as const];
    }
    case "Quality": {
      const inner = oneChangeQualityCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeKitQualityCommands: { qualityId: __kid(id), commands: [inner] } } as const];
    }
    case "Port": {
      const inner = oneChangePortCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeKitPortCommands: { portId: __kid(id), commands: [inner] } } as const];
    }
    case "Concept": {
      const inner = oneChangeConceptCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeConceptCommands: { conceptId: __kid(id), commands: [inner] } } as const];
    }
    case "Family": {
      const inner = oneChangeFamilyCommandForField(field, valueCast);
      if (!inner) return [];
      return [{ changeFamilyCommands: { familyId: __kid(id), commands: [inner] } } as const];
    }
    case "Piece": {
      if (!designId) return [];
      if (field === "name") return [kitChangeDesignPiece(designId, id, [{ name: { name: String(value) } }])];
      if (field === "description")
        return [kitChangeDesignPiece(designId, id, [{ description: { description: value == null ? null : String(value) } }])];
      if (field === "plane") return [kitChangeDesignPiece(designId, id, [{ plane: { plane: value as KitJsonTreeDto } }])];
      if (field === "center") return [kitChangeDesignPiece(designId, id, [{ center: { center: value as KitJsonTreeDto } }])];
      if (field === "scale") return [kitChangeDesignPiece(designId, id, [{ scale: { scale: Number(value) } }])];
      if (field === "mirrorPlane")
        return [kitChangeDesignPiece(designId, id, [{ mirrorPlane: { mirrorPlane: value as KitJsonTreeDto } }])];
      if (field === "isHidden" || field === "hidden") return [kitChangeDesignPiece(designId, id, [{ hidden: { hidden: Boolean(value) } }])];
      if (field === "isLocked" || field === "locked") return [kitChangeDesignPiece(designId, id, [{ locked: { locked: Boolean(value) } }])];
      if (field === "color") return [kitChangeDesignPiece(designId, id, [{ color: { color: value == null ? null : String(value) } }])];
      if (field === "type" || field === "typeId") {
        const t = value;
        const tid = t && typeof t === "object" && t !== null && "id" in t ? String((t as { id: string }).id) : String(t);
        return [kitChangeDesignPiece(designId, id, [{ type: { typeId: { id: tid } } }])];
      }
      return [];
    }
    case "Connection": {
      if (!designId) return [];
      const dk = connectionDiffKeyForDataKey(field);
      if (dk === "gap") return [kitChangeDesignConnection(designId, id, [{ gap: { value: Number(value) } }])];
      if (dk === "shift") return [kitChangeDesignConnection(designId, id, [{ shift: { value: Number(value) } }])];
      if (dk === "rise") return [kitChangeDesignConnection(designId, id, [{ rise: { value: Number(value) } }])];
      if (dk === "rotation") return [kitChangeDesignConnection(designId, id, [{ rotation: { value: Number(value) } }])];
      if (dk === "turn") return [kitChangeDesignConnection(designId, id, [{ turn: { value: Number(value) } }])];
      if (dk === "tilt") return [kitChangeDesignConnection(designId, id, [{ tilt: { value: Number(value) } }])];
      if (dk === "x") return [kitChangeDesignConnection(designId, id, [{ x: { value: Number(value) } }])];
      if (dk === "y") return [kitChangeDesignConnection(designId, id, [{ y: { value: Number(value) } }])];
      if (field === "description")
        return [kitChangeDesignConnection(designId, id, [{ description: { value: value == null ? null : String(value) } }])];
      return [];
    }
    default:
      return [];
  }
}

function oneChangeTypeCommandForField(field: string, value: SchemaEntityFieldValue): ChangeTypeCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  if (field === "image") return { image: { image: (value as string) ?? null } } as const;
  if (field === "stock") return { stock: { stock: (value as number) ?? null } } as const;
  if (field === "typeVirtual" || field === "virtual" || field === "isAbstract")
    return { typeVirtual: { value: (value as boolean) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  return null;
}
function oneChangeDesignCommandForField(field: string, value: SchemaEntityFieldValue): ChangeDesignCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  if (field === "image") return { image: { image: (value as string) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  return null;
}
function oneChangeAuthorCommandForField(field: string, value: SchemaEntityFieldValue): ChangeAuthorCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "email") return { email: { email: String(value ?? "") } } as const;
  if (field === "role") return { role: { role: (value as string) ?? null } } as const;
  if (field === "rank") return { rank: { rank: (value as number) ?? null } } as const;
  return null;
}
function oneChangeTagCommandForField(field: string, value: SchemaEntityFieldValue): ChangeTagCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "order" || field === "orderIndex") return { order: { order: (value as number) ?? null } } as const;
  return null;
}
function oneChangeFileCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFileCommand | null {
  if (field === "url") return { url: { url: String(value ?? "") } } as const;
  if (field === "mime") return { mime: { mime: (value as string) ?? null } } as const;
  if (field === "size") return { size: { size: (value as number) ?? null } } as const;
  if (field === "hash") return { hash: { hash: (value as string) ?? null } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "created" || field === "createdAt") return { created: { created: (value as string) ?? null } } as const;
  if (field === "updated" || field === "updatedAt") return { updated: { updated: (value as string) ?? null } } as const;
  return null;
}
function oneChangeFolderCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFolderCommand | null {
  if (field === "path") return { path: { path: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  return null;
}
function oneChangeQualityCommandForField(field: string, value: SchemaEntityFieldValue): ChangeKitQualityCommand | null {
  if (field === "key") return { key: { key: String(value ?? "") } } as const;
  if (field === "value") return { value: { value: (value as string) ?? null } } as const;
  if (field === "unit") return { unit: { unit: (value as string) ?? null } } as const;
  if (field === "definition") return { definition: { definition: (value as string) ?? null } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  return null;
}
function oneChangePortCommandForField(field: string, value: SchemaEntityFieldValue): ChangePortCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  return null;
}
function oneChangeConceptCommandForField(field: string, value: SchemaEntityFieldValue): ChangeConceptCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "order" || field === "orderIndex") return { order: { order: (value as number) ?? null } } as const;
  return null;
}
function oneChangeFamilyCommandForField(field: string, value: SchemaEntityFieldValue): ChangeFamilyCommand | null {
  if (field === "name") return { name: { name: String(value ?? "") } } as const;
  if (field === "description") return { description: { description: (value as string) ?? null } } as const;
  if (field === "icon") return { icon: { icon: (value as string) ?? null } } as const;
  return null;
}

/** @emoji 🧾 Shorthand for `client.submitChangeKitCommands` (React / tests). */
export async function submitKitChangeCommands(
  client: KitStoreClient,
  commands: readonly ChangeKitCommand[],
): Promise<SetResult> {
  return client.submitChangeKitCommands(commands);
}

/** @emoji 🧾 Locates the parent design id for a piece or connection via the materialized kit snapshot. */
export async function resolveDesignIdForPieceOrConnection(
  client: KitStoreClient,
  entityKind: string,
  entityId: string,
): Promise<string | null> {
  const snap = await client.fetchFullKit();
  if (entityKind === "Piece") return __findDesignIdForPieceInKitDto(snap, entityId);
  if (entityKind === "Connection") return __findDesignIdForConnectionInKitDto(snap, entityId);
  return null;
}

/** @emoji 🧾 Applies a piece field patch under one design (wire construction stays in JS). */
export async function kitStoreClientUpdatePiece(
  client: KitStoreClient,
  designId: string,
  pieceId: string,
  patch: unknown,
): Promise<SetResult> {
  const pcmds = piecePatchToChangeCommands(patch as PieceFieldPatchInput);
  if (!pcmds.length) return { ok: true };
  return client.submitChangeKitCommands([kitChangeDesignPiece(designId, pieceId, pcmds)]);
}

/** @emoji 🧾 Applies a connection field patch under one design (wire construction stays in JS). */
export async function kitStoreClientUpdateConnection(
  client: KitStoreClient,
  designId: string,
  connectionId: string,
  patch: unknown,
): Promise<SetResult> {
  const ccmds = connectionPatchToChangeCommands(patch as ConnectionFieldPatchInput);
  if (!ccmds.length) return { ok: true };
  return client.submitChangeKitCommands([kitChangeDesignConnection(designId, connectionId, ccmds)]);
}

function __findDesignIdForPieceInKitDto(kit: KitFullDto, pieceId: string): string | null {
  for (const d of kit.designs ?? []) {
    if (d.pieces?.some((p) => p.id === pieceId)) return d.id;
  }
  return null;
}
function __findDesignIdForConnectionInKitDto(kit: KitFullDto, connectionId: string): string | null {
  for (const d of kit.designs ?? []) {
    if (d.connections?.some((c) => c.id === connectionId)) return d.id;
  }
  return null;
}

/**
 * @emoji 🧾 Writes a single field on a top-level or nested entity via `changeKitCommands` (React / kit store).
 * `key` is the DTO / schema data key (e.g. `name`, `icon`).
 */
export async function writeKitStoreClientSchemaField(
  client: KitStoreClient,
  typeName: string,
  key: string,
  value: unknown,
  entityId: string,
): Promise<SetResult> {
  const valueCast = value as SchemaEntityFieldValue;
  const root = await client.fetchFullKit();
  let designId: string | null = null;
  if (typeName === "Piece") designId = __findDesignIdForPieceInKitDto(root, entityId);
  if (typeName === "Connection") designId = __findDesignIdForConnectionInKitDto(root, entityId);
  const cmds = buildSchemaEntityChangeCommands(
    typeName,
    entityId,
    key,
    valueCast,
    typeName === "Piece" || typeName === "Connection" ? designId : null,
  );
  if (!cmds.length) return { ok: false, error: { kind: "NotSupported", message: `${typeName}.${key}` } };
  return client.submitChangeKitCommands(cmds);
}

// #endregion 🔖ChangeKitCommand

// #endregion 🔌JsonGraphQlDtoTypes

// #region 🪢InternalReadBatch
// Read command outputs are public {@link ReadDesignCommandOutput} / {@link ReadPieceCommandOutput} / {@link ReadTypeCommandOutput} above.
// #endregion 🪢InternalReadBatch

// #region 🧰GraphqlUtil

function normalizeRustSetError(raw: JsonValue): SetError {
  if (raw == null || typeof raw !== "object" || Array.isArray(raw)) return { kind: "Internal", message: "invalid error payload" };
  if (!isJsonObjectNode(raw)) return { kind: "Internal", message: "invalid error payload" };
  const o = raw as JsonObject;
  const kind = typeof o["kind"] === "string" ? (o["kind"] as SetErrorKind) : "Internal";
  const message = typeof o["message"] === "string" ? o["message"] : JSON.stringify(raw);
  return { kind, message };
}

function normalizeWasmThrownKitError(err: { toString(): string }): SetError {
  const message = String(err).replace(/^Error:\s*/, "").trim();
  const lower = message.toLowerCase();
  if (lower.includes("illegal name") || lower.includes("cannot be empty")) return { kind: "IllegalName", message };
  if (lower.includes("name too long") || (lower.includes("exceeds") && lower.includes("char"))) return { kind: "NameTooLong", message };
  return { kind: "Internal", message };
}

function withTimeout<T>(p: Promise<T>, ms: number, label: string): Promise<T> {
  if (!ms || ms <= 0) return p;
  return new Promise((resolve, reject) => {
    const t = setTimeout(() => reject(new Error(label)), ms);
    p.then(
      (v) => {
        clearTimeout(t);
        resolve(v);
      },
      (e) => {
        clearTimeout(t);
        reject(e);
      },
    );
  });
}

function kitGraphqlData<TData>(response: KitGraphqlResponseEnvelope<TData>): TData {
  if (response == null || typeof response !== "object") throw new Error("kitGraphql: response is not an object");
  const r = response;
  if (Array.isArray(r.errors) && r.errors.length > 0) throw new Error(r.errors[0]?.message ?? "GraphQL error");
  const d = r.data;
  if (d != null && typeof d === "object") return d;
  throw new Error("kitGraphql: no data in response");
}

/** @internal Root field for scoped kit reads (`Query.kit`). */
function gqlDataKitRoot<T>(d: { kit?: T } | null | undefined): T | undefined {
  if (d == null) return undefined;
  return d.kit;
}

function kitGraphqlJsonToReadonlyArray(
  v: JsonValue | KitJsonTreeDto | null | undefined,
): readonly KitJsonTreeDto[] {
  if (Array.isArray(v)) return v as readonly KitJsonTreeDto[];
  if (v == null) return [];
  if (typeof v === "string") {
    try {
      const p: JsonValue = parseJsonValue(v);
      return Array.isArray(p) ? (p as readonly KitJsonTreeDto[]) : [];
    } catch {
      return [];
    }
  }
  return [];
}

function __mutableJsonObjectCopy(row: JsonObject | KitJsonObjectDto): { [k: string]: JsonValue } {
  const o: { [k: string]: JsonValue } = {};
  for (const [k, v] of Object.entries(row)) {
    o[k] = v as JsonValue;
  }
  return o;
}

/** @emoji 🧾 Maps GraphQL `TypeMetadataObject` / `DesignMetadataObject` field names to {@link TypeSchema} / {@link DesignSchema} wire keys (`createdAt`, `virtual`, …). */
function __normalizeTypeOrDesignMetadataRow(row: JsonObject | KitJsonObjectDto): JsonObject {
  const out = __mutableJsonObjectCopy(row);
  if (out["createdAt"] === undefined && typeof out["created"] === "string") out["createdAt"] = out["created"] as string;
  delete out["created"];
  if (out["updatedAt"] === undefined && typeof out["updated"] === "string") out["updatedAt"] = out["updated"] as string;
  delete out["updated"];
  if (out["virtual"] === undefined && typeof out["typeVirtual"] === "boolean") out["virtual"] = out["typeVirtual"] as boolean;
  delete out["typeVirtual"];
  if (out["parent"] === undefined && out["kit"] != null && typeof out["kit"] === "object" && !Array.isArray(out["kit"])) {
    const k = out["kit"] as JsonObject;
    if (typeof k["id"] === "string") out["parent"] = { id: k["id"] as string };
  }
  delete out["kit"];
  return out;
}

//#region 🔌KitGraphqlReadSelections
const KIT_GQL_TYPE_SHALLOW_FIELDS =
  "id name description icon image stock isAbstract unit location { id } created updated";
const KIT_GQL_DESIGN_SHALLOW_FIELDS =
  "id name description icon image location { id } unit created updated kit { id }";
const KIT_GQL_TYPE_METADATA_FIELDS =
  "id name description icon image stock isAbstract unit location { id } created updated";
const KIT_GQL_DESIGN_METADATA_FIELDS =
  "id name description icon image location { id } unit created updated kit { id }";
//#endregion 🔌KitGraphqlReadSelections

function kitGraphqlKitShallowRoot(row: JsonObject): JsonObject {
  const s = row["shallow"];
  if (s != null && typeof s === "object" && !Array.isArray(s)) return s as JsonObject;
  return {} as JsonObject;
}

function kitGraphqlShallowPacketsFromArray(arr: JsonValue | undefined): readonly JsonObject[] {
  if (!Array.isArray(arr)) return [];
  const out: JsonObject[] = [];
  for (const el of arr) {
    if (el != null && typeof el === "object" && !Array.isArray(el)) {
      const sh = (el as JsonObject)["shallow"];
      if (sh != null && typeof sh === "object" && !Array.isArray(sh)) out.push(sh as JsonObject);
    }
  }
  return out;
}

function kitGraphqlExtractNestedMetadata(arr: JsonValue | undefined): readonly JsonObject[] {
  if (!Array.isArray(arr)) return [];
  const out: JsonObject[] = [];
  for (const el of arr) {
    if (el != null && typeof el === "object" && !Array.isArray(el)) {
      const m = (el as JsonObject)["metadata"];
      if (m != null && typeof m === "object" && !Array.isArray(m)) out.push(m as JsonObject);
    }
  }
  return out;
}

/** @emoji 🧾 GraphQL JSON uses `null` for absent scalars; Zod `.optional()` expects omission — drop top-level `null` entries before parse. */
function __stripTopLevelJsonNulls(row: JsonObject | KitJsonObjectDto): JsonObject {
  const out = __mutableJsonObjectCopy(row);
  for (const k of Object.keys(out)) {
    if (out[k] === null) delete out[k];
  }
  return out;
}

/** @emoji 🧾 JSON, `KitJsonTreeDto`, or typed bus {@link KitEvent} in subscription and lifecycle helpers. */

/** @emoji 🧾 Narrows subscription payloads to semio kit command lifecycle rows. */
export function isKitCommandLifecycleEvent(event: unknown): event is KitCommandLifecycleEvent {
  if (event == null || typeof event !== "object" || Array.isArray(event)) return false;
  const c = (event as KitJsonObjectDto)["semioKitCommand"];
  if (c == null || typeof c !== "object" || Array.isArray(c)) return false;
  const v = c as JsonObject;
  return typeof v["requestId"] === "string" && typeof v["commandKind"] === "string" && typeof v["phase"] === "string";
}

function __normalizeTopLevelKitEventJson(raw: unknown): KitJsonTreeDto | null {
  if (raw === "Changed") return { Changed: null } as KitJsonTreeDto;
  if (raw === "ValidationInvalidated") return { ValidationInvalidated: null } as KitJsonTreeDto;
  if (raw == null) return null;
  return raw as KitJsonTreeDto;
}

export function normalizeKitEventFromSubscription(raw: unknown): KitEvent | undefined {
  const raw0 = __normalizeTopLevelKitEventJson(raw);
  if (raw0 == null) return undefined;
  if (typeof raw0 === "string") return undefined;
  if (typeof raw0 !== "object" || Array.isArray(raw0)) return undefined;
  const top = raw0 as JsonObject;
  const lifecycleWrapper: KitJsonTreeDto =
    top["semioKitCommand"] !== undefined
      ? raw0
      : top["SemioKitCommand"] !== undefined
        ? { semioKitCommand: top["SemioKitCommand"] as KitJsonTreeDto }
        : raw0;
  const w = isJsonObjectNode(lifecycleWrapper) ? lifecycleWrapper : null;
  const command = w?.["semioKitCommand"];
  if (isKitCommandLifecycleEvent({ semioKitCommand: command } as KitEvent)) {
    if (command == null || typeof command !== "object" || Array.isArray(command)) return undefined;
    const value = command as JsonObject;
    const requestIdRaw = value["requestId"];
    if (typeof requestIdRaw !== "string" || typeof value["commandKind"] !== "string" || typeof value["phase"] !== "string") return undefined;
    const errRaw = value["error"];
    const error =
      errRaw != null && typeof errRaw === "object" && !Array.isArray(errRaw) ? normalizeRustSetError(errRaw as JsonValue) : undefined;
    return {
      semioKitCommand: {
        requestId: requestIdRaw,
        commandKind: value["commandKind"] as string,
        phase: value["phase"] as KitCommandLifecyclePhase,
        result: (value["result"] as KitJsonTreeDto | undefined) ?? undefined,
        error,
      },
    };
  }
  return raw0 as KitEvent;
}

type KitGraphqlHandle = { execute(requestJson: string): Promise<string> };

async function kitGraphqlRun(
  handle: KitGraphqlHandle,
  body: { query: string; variables?: GraphQlVariables; operationName?: string },
  timeoutMs?: number,
): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
  const json = await withTimeout(handle.execute(JSON.stringify(body)), timeoutMs ?? 0, "graphql");
  return parseJsonValue(json) as KitGraphqlResponseEnvelope<JsonValue>;
}

/**
 * @emoji 🌐 Typed GraphQL execute: same transport as {@link kitGraphqlRun} with a parametric `data` root for call sites.
 * @typeParam TData GraphQL `data` object shape (e.g. {@link KitGraphqlDataKitScopedFullDto} for {@link KIT_SCOPED_FULL_DTO_QUERY}).
 */
export async function kitGraphqlRunTyped<TData extends JsonValue>(
  handle: { execute(requestJson: string): Promise<string> },
  body: { query: string; variables?: GraphQlVariables; operationName?: string },
  timeoutMs?: number,
): Promise<KitGraphqlResponseEnvelope<TData>> {
  return (await kitGraphqlRun(handle, body, timeoutMs)) as KitGraphqlResponseEnvelope<TData>;
}

// #endregion 🧰GraphqlUtil

// #region 🪜Transport

type WasmExecuteFn = (requestJson: string) => Promise<string>;
type WasmSubscribeFn = (requestJson: string, onEvent: (eventJson: string) => void) => Promise<void>;

/** @internal Used only when `globalThis.Worker` is missing (e.g. Node vitest); browser builds always use {@link WorkerStringTransport}. */
class InlineWasmTransport {
  constructor(
    private readonly handle: {
      execute: WasmExecuteFn;
      subscribe: WasmSubscribeFn;
      free?: () => void;
    },
  ) {}
  /** Returns the **complete JSON** GraphQL response document (one `{ "data": ..., "errors": ... }`). */
  async execute(requestJson: string): Promise<string> {
    return await this.handle.execute(requestJson);
  }
  /** Streams subscription events as **complete JSON** documents (one full GraphQL response per event). */
  async subscribe(requestJson: string, onEvent: (eventJson: string) => void): Promise<void> {
    await this.handle.subscribe(requestJson, onEvent);
  }
  dispose(): void {
    if (typeof this.handle.free === "function") {
      try {
        this.handle.free();
      } catch {
        /* ignore */
      }
    }
  }
}

class WorkerStringTransport {
  private worker: Worker;
  private nextSerial = 0;

  constructor(worker: Worker) {
    this.worker = worker;
  }

  init(dto: KitFullDto): Promise<void> {
    return new Promise((resolve, reject) => {
      const t = setTimeout(() => reject(new Error("worker init timeout")), 30_000);
      const onReady = (ev: MessageEvent<string>) => {
        try {
          const m = JSON.parse(ev.data) as { op: string };
          if (m.op === "ready") {
            clearTimeout(t);
            this.worker.removeEventListener("message", onReady);
            resolve();
          }
        } catch {
          /* ignore */
        }
      };
      this.worker.addEventListener("message", onReady);
      this.worker.postMessage(JSON.stringify({ op: "init", dto }));
    });
  }

  /** Returns the **complete JSON** GraphQL response document (one `{ "data": ..., "errors": ... }`). */
  async execute(requestJson: string): Promise<string> {
    const reqId = `r-${++this.nextSerial}-${Date.now().toString(36)}`;
    return await new Promise<string>((resolve, reject) => {
      let result: string | null = null;
      const w = (ev: MessageEvent<string>) => {
        let m: { op: string; reqId?: string; json?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.reqId !== reqId) return;
        if (m.op === "result" && typeof m.json === "string") {
          result = m.json;
        }
        if (m.op === "done") {
          this.worker.removeEventListener("message", w);
          if (result == null) reject(new Error("graphql: worker completed without result"));
          else resolve(result);
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", w);
          reject(new Error(m.message ?? "worker error"));
        }
      };
      this.worker.addEventListener("message", w);
      this.worker.postMessage(JSON.stringify({ op: "execute", reqId, body: requestJson }));
    });
  }

  /** Streams subscription events as **complete JSON** documents (one full GraphQL response per event). */
  async subscribe(requestJson: string, onEvent: (eventJson: string) => void): Promise<void> {
    const reqId = `s-${++this.nextSerial}-${Date.now().toString(36)}`;
    await new Promise<void>((resolve, reject) => {
      const w = (ev: MessageEvent<string>) => {
        let m: { op: string; reqId?: string; json?: string; message?: string };
        try {
          m = JSON.parse(ev.data) as typeof m;
        } catch {
          return;
        }
        if (m.reqId !== reqId) return;
        if (m.op === "event" && typeof m.json === "string") onEvent(m.json);
        if (m.op === "done") {
          this.worker.removeEventListener("message", w);
          resolve();
        }
        if (m.op === "error") {
          this.worker.removeEventListener("message", w);
          reject(new Error(m.message ?? "worker error"));
        }
      };
      this.worker.addEventListener("message", w);
      this.worker.postMessage(JSON.stringify({ op: "subscribe", reqId, body: requestJson }));
    });
  }

  dispose(): void {
    this.worker.terminate();
  }
}

// #endregion 🪜Transport

// #region 📦KitStore

/** @internal Resolves `semio_bg.wasm` for Node / Vitest when `import.meta.url` is not adjacent to `semio/rs/pkg` (e.g. bundled `semio/react` tests). */
async function __readSemioWasmBytesFromMonorepoCandidates(): Promise<Uint8Array | undefined> {
  try {
    const fs = await import("node:fs/promises");
    const path = await import("node:path");
    const envPath =
      typeof process !== "undefined" && process.env
        ? process.env["SEMIO_WASM_BG_PATH"] ?? process.env["SEMIO_RS_WASM_PATH"]
        : undefined;
    const candidates: string[] = [];
    if (typeof envPath === "string" && envPath.trim().length) candidates.push(envPath.trim());
    try {
      const { fileURLToPath } = await import("node:url");
      candidates.push(fileURLToPath(new URL("../rs/pkg/semio_bg.wasm", import.meta.url)));
    } catch {
      /* Vitest may bundle this module with a synthetic `import.meta.url` that is not beside `semio/rs/pkg`. */
    }
    if (typeof process !== "undefined" && typeof process.cwd === "function") {
      let dir = process.cwd();
      for (let i = 0; i < 16; i++) {
        candidates.push(path.join(dir, "semio", "rs", "pkg", "semio_bg.wasm"));
        candidates.push(path.join(dir, "rs", "pkg", "semio_bg.wasm"));
        const parent = path.dirname(dir);
        if (parent === dir) break;
        dir = parent;
      }
    }
    for (const wasmPath of candidates) {
      try {
        return await fs.readFile(wasmPath);
      } catch {
        /* try next candidate */
      }
    }
  } catch {
    /* fs / path unavailable */
  }
  return undefined;
}

/** @emoji 🔗 `Mutation.kitStore.batch` selection; must stay aligned with `semio/graphql/schema.graphql`. */
export const KIT_STORE_BATCH_MUTATION = `mutation($input: KitStoreBatchInput!) { kitStore { batch(input: $input) { clientMutationId results { kind ok count changeKind changeKindOther inverse sessionId draftId transactionId checkpointId alternativeId backbone { attached kind tip } conflicts { id backboneTip reason createdAt } } } } } }`;

/** @emoji 🔗 Root subscription document for `Subscription.eventStream` (kit scalar stream). */
export const KIT_EVENT_STREAM_SUBSCRIPTION = `subscription { eventStream }`;

/**
 * @emoji 🔗 Scoped full kit DTO via `Query.kit` — must stay aligned with `semio/graphql/schema.graphql` (`KitStore.fullDto`).
 */
export const KIT_SCOPED_FULL_DTO_QUERY = `query KitScopedFullDto($scope: KitReadScopeInput!) { kit(scope: $scope) { fullDto } }` as const;

/** @emoji 🧾 Typed `data` shape for {@link KIT_SCOPED_FULL_DTO_QUERY} (after {@link kitGraphqlData}). */
export type KitGraphqlDataKitScopedFullDto = Readonly<{
  readonly kit: Readonly<{ readonly fullDto: KitJsonTreeDto }> | null;
}>;

/**
 * @emoji 🌐 Single kit control plane: GraphQL strings over one dedicated `Worker` running `semio/rs` WASM (`KitStoreHandle`).
 */
export class KitStore {
  private readonly timeoutMs: number;
  private transport!: WorkerStringTransport | InlineWasmTransport;
  private readonly fanout = new Subject<KitEvent>();
  private gqlLoopRunning = false;
  private disposed = false;
  /** @emoji 🧭 VCS anchors for {@link KitStore.runScopedTransactionBatch}; auto-filled from batch results. */
  private kitWriteSessionId: string | null = null;
  private kitWriteDraftId: string | null = null;
  private kitWriteTransactionId: string | null = null;

  private constructor(timeoutMs: number) {
    this.timeoutMs = timeoutMs;
  }

  static async open(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
    const timeoutMs = opts?.timeoutMs ?? 60_000;
    const wasmSpecifier = opts?.wasmSpecifier ?? (globalThis as { __SEMIO_WASM_SPECIFIER__?: string }).__SEMIO_WASM_SPECIFIER__ ?? "@semio/rs-wasm";
    const dto = JSON.parse(JSON.stringify(initialKit)) as KitFullDto;
    /** Vitest may expose `Worker` (e.g. jsdom); blob worker still `fetch`es `.wasm` — prefer inline init when Vitest is active. */
    const preferInlineWasmInVitest = (() => {
      try {
        const env = (import.meta as { env?: JsonObject }).env;
        if (env && Boolean(env["VITEST"])) return true;
      } catch {
        /* ignore */
      }
      return typeof process !== "undefined" && !!process.env && "VITEST" in process.env;
    })();

    const wasmBytesPre = await __readSemioWasmBytesFromMonorepoCandidates();
    const useDedicatedWorker =
      typeof Worker !== "undefined" && !preferInlineWasmInVitest && wasmBytesPre == null;

    if (useDedicatedWorker) {
      const worker = opts?.workerFactory?.() ?? createKitStoreWorker();
      const wt = new WorkerStringTransport(worker);
      await wt.init(dto);
      const ks = new KitStore(timeoutMs);
      ks.transport = wt;
      await withTimeout(ks.warmGraphqlRead(), timeoutMs, "graphql");
      void ks.startSubscriptionLoop();
      return ks;
    }

    const mod = wasmSpecifier === "@semio/rs-wasm" ? await import("@semio/rs-wasm") : await import(/* @vite-ignore */ wasmSpecifier);
    if (typeof mod.default === "function") {
      if (wasmBytesPre) await mod.default({ module_or_path: wasmBytesPre });
      else await mod.default();
    } else await mod.default();
    if (typeof mod.boot === "function") mod.boot();
    const handle = mod.KitStoreHandle.create(dto as object);
    const t = new InlineWasmTransport(handle);
    const ks = new KitStore(timeoutMs);
    ks.transport = t;
    await withTimeout(ks.warmGraphqlRead(), timeoutMs, "graphql");
    void ks.startSubscriptionLoop();
    return ks;
  }

  private graphqlHandle(): KitGraphqlHandle {
    return { execute: (requestJson: string) => this.transport.execute(requestJson) };
  }

  private ensureAlive(): void {
    if (this.disposed) throw new Error("KitStore disposed");
  }

  private readScopeVars(scope: KitReadScope, extra: GraphQlVariables = {} as GraphQlVariables): GraphQlVariables {
    return { scope: kitReadScopeToGraphQLInput(scope) as JsonValue, ...extra } as GraphQlVariables;
  }

  private async gqlRun(body: { query: string; variables?: GraphQlVariables; operationName?: string }): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    this.ensureAlive();
    return kitGraphqlRun(this.graphqlHandle(), body, this.timeoutMs);
  }

  private async gqlRunWithReadScope(
    scope: KitReadScope,
    body: { query: string; variables?: GraphQlVariables | undefined; operationName?: string },
  ): Promise<KitGraphqlResponseEnvelope<JsonValue>> {
    return this.gqlRun({ ...body, variables: this.readScopeVars(scope, body.variables ?? ({} as GraphQlVariables)) });
  }

  /** @emoji 🧾 Single `Query.kit(scope: …) { … }` selection using only `$scope` (dedupes scoped read strings). */
  private async gqlKitReadOnlyScope(scope: KitReadScope, innerSelection: string): Promise<JsonObject> {
    const data = kitGraphqlData(
      await this.gqlRunWithReadScope(scope, {
        query: `query($scope: KitReadScopeInput!) { kit(scope: $scope) { ${innerSelection} } }`,
      }),
    ) as { kit?: JsonObject | null };
    const k = gqlDataKitRoot(data);
    return k != null && typeof k === "object" && !Array.isArray(k) ? (k as JsonObject) : ({} as JsonObject);
  }

  /** @emoji 📣 Subscribe to kit GraphQL subscription events (RxJS-free public surface). */
  subscribe(handler: (event: KitEvent) => void): Unsubscribe {
    const sub = this.fanout.subscribe({ next: handler });
    return () => {
      sub.unsubscribe();
    };
  }

  /** @emoji 📣 Subscribe only when {@link KitEventFilter} returns true. */
  subscribeFiltered(filterFn: KitEventFilter, handler: (event: KitEvent) => void): Unsubscribe {
    const sub = this.fanout.pipe(filter(filterFn)).subscribe({ next: handler });
    return () => {
      sub.unsubscribe();
    };
  }

  /** @emoji 📣 Fires only after coalescing wire `Changed` / synthetic `{ Changed: null }` rows. */
  subscribeRootInvalidation(handler: () => void): Unsubscribe {
    return this.subscribeFiltered(
      (ev) => typeof ev === "object" && ev !== null && "Changed" in ev && (ev as { Changed?: null }).Changed === null,
      () => handler(),
    );
  }

  /** @emoji 📣 Kit command lifecycle scalar rows (`semioKitCommand` / `SemioKitCommand`). */
  subscribeSemioKitCommandLifecycle(handler: (row: KitCommandLifecycleEvent["semioKitCommand"]) => void): Unsubscribe {
    return this.subscribeFiltered(
      (ev) => isKitCommandLifecycleEvent(ev),
      (ev) => handler((ev as KitCommandLifecycleEvent).semioKitCommand),
    );
  }

  private startSubscriptionLoop(): void {
    if (this.gqlLoopRunning) return;
    this.gqlLoopRunning = true;
    void this.transport
      .subscribe(JSON.stringify({ query: KIT_EVENT_STREAM_SUBSCRIPTION }), (eventJson: string) => {
        try {
          const msg = parseJsonValue(eventJson) as KitGraphqlResponseEnvelope<{ eventStream?: JsonValue | null }>;
          if (msg.errors && Array.isArray(msg.errors) && msg.errors.length) return;
          const ev: JsonValue | null | undefined = msg.data?.eventStream;
          if (ev === undefined) return;
          const n = normalizeKitEventFromSubscription(ev);
          if (n) this.fanout.next(n);
          else this.fanout.next(ev as KitEvent);
        } catch {
          /* ignore */
        }
      })
      .catch(() => {
        this.gqlLoopRunning = false;
      });
  }

  async dispose(): Promise<void> {
    if (this.disposed) return;
    this.disposed = true;
    this.fanout.complete();
    this.transport.dispose();
  }

  /** @internal One `Query.kit { fullDto }` read to verify GraphQL after WASM init (no local kit cache). */
  private async warmGraphqlRead(): Promise<void> {
    await this.materializedLiveJsonForReadScope(theKitReadScope);
  }

  /** @emoji 🧾 Full DTO for a {@link KitReadScope} via scoped `Query.kit` (`fullDto`); sole full-kit read path (no WASM `snapshot` fallback). */
  async materializedLiveJsonForReadScope(scope: KitReadScope): Promise<KitFullDto> {
    this.ensureAlive();
    const data = kitGraphqlData(
      await kitGraphqlRunTyped<KitGraphqlDataKitScopedFullDto>(this.graphqlHandle(), {
        query: KIT_SCOPED_FULL_DTO_QUERY,
        variables: this.readScopeVars(scope),
      },
      this.timeoutMs,
      ),
    );
    const j = data.kit?.fullDto;
    if (j == null || typeof j !== "object" || Array.isArray(j)) {
      throw new Error("kitGraphql: fullDto missing or not an object for the given read scope");
    }
    return semioCoerceKitFullDtoFromJson(j as KitJsonTreeDto);
  }

  /** @emoji 🧾 Main-line live kit DTO from `Query.kit` / `fullDto`. */
  async theKit(): Promise<KitFullDto> {
    return this.materializedLiveJsonForReadScope(theKitReadScope);
  }

  async materializeAt(checkpointId: string): Promise<KitFullDto> {
    const idArg = checkpointId.trim() === "" ? null : checkpointId;
    const data = kitGraphqlData(
      await this.gqlRunWithReadScope(theKitReadScope, {
        query: `query($scope: KitReadScopeInput!, $id: String) { kit(scope: $scope) { materializeAt(checkpointId: $id) } }`,
        variables: { id: idArg },
      }),
    );
    const j = (data as { kit?: { materializeAt?: JsonValue } }).kit?.materializeAt;
    return j as KitFullDto;
  }

  async vcsState(): Promise<JsonObject> {
    const data = kitGraphqlData(
      await this.gqlRunWithReadScope(theKitReadScope, {
        query: `query($scope: KitReadScopeInput!) { kit(scope: $scope) { vcsState { theKitHead theKitLine root { id name } checkpoints { id parent message time authors hash isRelease changeCount } alternatives { id name root checkpoints } sessions { id drafts { id parentCheckpoint targetAlternative finalizedTransactionCount redoTransactionCount openTransactionId canUndo canRedo } } } } }`,
      }),
    );
    return ((data as { kit?: { vcsState?: JsonObject } }).kit?.vcsState ?? {}) as JsonObject;
  }

  /** @emoji 🧾 True when any session draft reports `canUndo` (VCS draft/transaction stack). */
  async canUndo(): Promise<boolean> {
    const v = await this.vcsState();
    const sessions = v["sessions"];
    if (!Array.isArray(sessions)) return false;
    for (const s of sessions) {
      const drafts = (s as { drafts?: JsonValue }).drafts;
      if (!Array.isArray(drafts)) continue;
      for (const d of drafts) {
        if ((d as { canUndo?: boolean }).canUndo) return true;
      }
    }
    return false;
  }

  /** @emoji 🧾 True when any session draft reports `canRedo`. */
  async canRedo(): Promise<boolean> {
    const v = await this.vcsState();
    const sessions = v["sessions"];
    if (!Array.isArray(sessions)) return false;
    for (const s of sessions) {
      const drafts = (s as { drafts?: JsonValue }).drafts;
      if (!Array.isArray(drafts)) continue;
      for (const d of drafts) {
        if ((d as { canRedo?: boolean }).canRedo) return true;
      }
    }
    return false;
  }


  /** @emoji 🧾 Maps control-plane canvas `commandKind` + `variables` to one `TransactionBatchCommandInput` entry (draft/transaction scoped). */
  private scopedTransactionInnerFromControl(commandKind: string, variables: JsonObject): JsonObject | null {
    const v = variables;
    const designCommand = (sub: JsonObject, designId: string) => ({
      design: { designId, commands: [sub] },
    });
    switch (commandKind) {
      case "changeKitCommands":
        return { changeKitCommands: { commands: (v["commands"] as readonly ChangeKitCommand[] | undefined) ?? [] } } as GraphQlVariables;
      case "clusterPieces":
        return designCommand(
          { clusterPieces: { pieceIds: v.pieceIds, clusterName: v.clusterName } },
          String(v.designId),
        ) as JsonObject;
      case "dragPieces":
        return designCommand(
          { dragPieces: { pieceIds: v.pieceIds, du: v.du, dv: v.dv } },
          String(v.designId),
        ) as JsonObject;
      case "movePieces":
        return designCommand(
          { movePieces: { pieceIds: v.pieceIds, gap: v.gap, shift: v.shift, rise: v.rise } },
          String(v.designId),
        ) as JsonObject;
      case "fixPieces":
        return designCommand({ fixPieces: { pieceIds: v.pieceIds } }, String(v.designId)) as JsonObject;
      case "flattenDesign":
        return designCommand({ flattenDesign: { confirm: true } }, String(v.designId)) as JsonObject;
      case "expandDesign":
        return designCommand({ expandDesign: { nestedDesignId: String(v.nestedDesignId) } }, String(v.parentDesignId)) as JsonObject;
      case "deleteConnection":
        return designCommand(
          { deleteConnection: { connectionId: String(v.connectionId) } },
          String(v.designId),
        ) as JsonObject;
      case "changePieceType":
        return designCommand(
          { changePieceType: { pieceId: String(v.pieceId), newTypeId: String(v.newTypeId) } },
          String(v.designId),
        ) as JsonObject;
      case "createHangingPieces": {
        const pl = __kitPlaneToBatchInput(v.plane);
        if (!pl) return null;
        return designCommand({ createHangingPieces: { typeIds: v.typeIds, plane: pl } }, String(v.designId)) as JsonObject;
      }
      case "createConnectedPiece":
        return designCommand(
          {
            createConnectedPiece: {
              parentPiece: String(v.parentPiece),
              parentPort: String(v.parentPort),
              childType: String(v.childType),
              childPort: String(v.childPort),
            },
          },
          String(v.designId),
        ) as JsonObject;
      case "createFixedPiece": {
        const pl = __kitPlaneToBatchInput(v.plane);
        if (!pl) return null;
        return designCommand({ createFixedPiece: { typeId: String(v.typeId), plane: pl } }, String(v.designId)) as JsonObject;
      }
      default:
        return null;
    }
  }

  /** @emoji 🧾 Updates {@link KitStore.kitWriteSessionId} / draft / transaction ids from `kitStore.batch` rows. */
  private absorbWriteAnchorsFromBatchResults(results: readonly JsonObject[]): void {
    for (const raw of results) {
      const k = __normKitStoreBatchKind(raw.kind);
      if (k === "NEW_SESSION" && typeof raw.sessionId === "string") this.kitWriteSessionId = raw.sessionId;
      if (k === "NEW_DRAFT" && typeof raw.draftId === "string") this.kitWriteDraftId = raw.draftId;
      if (k === "START_TRANSACTION" && typeof raw.transactionId === "string") this.kitWriteTransactionId = raw.transactionId;
      if (k === "FINALIZE_TRANSACTION" || k === "ABORT_TRANSACTION") this.kitWriteTransactionId = null;
      if (k === "FINALIZE_DRAFT" || k === "ABORT_DRAFT") {
        this.kitWriteDraftId = null;
        this.kitWriteTransactionId = null;
      }
      if (k === "END_SESSION") {
        this.kitWriteSessionId = null;
        this.kitWriteDraftId = null;
        this.kitWriteTransactionId = null;
      }
    }
  }

  /** @emoji 🧾 Wraps transaction commands in `session → draft → transaction` batch shape (auto-starts missing levels). */
  private buildSessionScopedTopLevelCommand(innerTxCommands: readonly JsonObject[]): JsonObject {
    const sid = this.kitWriteSessionId;
    const did = this.kitWriteDraftId;
    const tid = this.kitWriteTransactionId;

    const transactionPayload: JsonObjectMutable = { commands: [...innerTxCommands] as unknown as JsonValue[] };
    if (tid) transactionPayload.transactionId = tid as JsonValue;

    const draftCommands: unknown[] = [];
    if (!tid) draftCommands.push({ startTransaction: { confirm: true } });
    draftCommands.push({ transaction: transactionPayload });

    const draftPayload: JsonObjectMutable = { commands: draftCommands as JsonValue[] };
    if (did) draftPayload.draftId = did as JsonValue;

    const sessionCommands: unknown[] = [];
    if (!sid) sessionCommands.push({ createSession: { confirm: true } });
    if (!did) sessionCommands.push({ createDraft: {} });
    sessionCommands.push({ draft: draftPayload });

    const sessionPayload: JsonObjectMutable = { commands: sessionCommands as JsonValue[] };
    if (sid) sessionPayload.sessionId = sid as JsonValue;
    return { session: sessionPayload } as JsonObject;
  }

  /** @emoji 🧾 Runs `kitStore.batch` with one session subtree that applies `innerTxCommands` inside an open transaction. */
  private async runScopedTransactionBatch(innerTxCommands: readonly JsonObject[]): Promise<readonly JsonObject[]> {
    const top = this.buildSessionScopedTopLevelCommand(innerTxCommands);
    const batch = await this.runKitStoreBatch([top as JsonObject]);
    this.absorbWriteAnchorsFromBatchResults(batch.results as JsonObject[]);
    return batch.results as JsonObject[];
  }

  /** @emoji 🧭 Exposes the last session/draft/transaction anchors inferred from batch results (or {@link KitStore.setKitWriteScope}). */
  getKitWriteScope(): KitWriteScope | null {
    const s = this.kitWriteSessionId;
    const d = this.kitWriteDraftId;
    const t = this.kitWriteTransactionId;
    if (s && d && t) return { sessionId: s, draftId: d, transactionId: t };
    return null;
  }

  /** @emoji 🧭 Pins the VCS write anchor (pass `null` to clear). */
  setKitWriteScope(scope: KitWriteScope | null): void {
    if (scope == null) {
      this.kitWriteSessionId = null;
      this.kitWriteDraftId = null;
      this.kitWriteTransactionId = null;
      return;
    }
    this.kitWriteSessionId = scope.sessionId;
    this.kitWriteDraftId = scope.draftId;
    this.kitWriteTransactionId = scope.transactionId;
  }

  /** @emoji 🧾 Finalizes the active kit write transaction (commits atoms into the draft). */
  async finalizeKitWriteTransaction(): Promise<SetResult> {
    const t = this.kitWriteTransactionId;
    const d = this.kitWriteDraftId;
    const s = this.kitWriteSessionId;
    if (!s || !d || !t) return { ok: false, error: { kind: "Internal", message: "finalizeKitWriteTransaction: no open transaction" } };
    try {
      const batch = await this.runKitStoreBatch([
        {
          session: {
            sessionId: s,
            commands: [
              {
                draft: {
                  draftId: d,
                  commands: [{ transaction: { transactionId: t, commands: [{ finalizeTransaction: { confirm: true } }] } }],
                },
              },
            ],
          },
        },
      ]);
      this.absorbWriteAnchorsFromBatchResults(batch.results as JsonObject[]);
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji 🧾 Aborts the active kit write transaction (drops uncommitted atoms). */
  async abortKitWriteTransaction(): Promise<SetResult> {
    const t = this.kitWriteTransactionId;
    const d = this.kitWriteDraftId;
    const s = this.kitWriteSessionId;
    if (!s || !d || !t) return { ok: false, error: { kind: "Internal", message: "abortKitWriteTransaction: no open transaction" } };
    try {
      const batch = await this.runKitStoreBatch([
        {
          session: {
            sessionId: s,
            commands: [
              {
                draft: {
                  draftId: d,
                  commands: [{ transaction: { transactionId: t, commands: [{ abortTransaction: { confirm: true } }] } }],
                },
              },
            ],
          },
        },
      ]);
      this.absorbWriteAnchorsFromBatchResults(batch.results as JsonObject[]);
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  private backboneBatchCommandsFromJson(kind: string, batchVariables: Readonly<JsonObject>): Readonly<JsonObject>[] | null {
    if (kind === "attachBackbone") {
      const c = (batchVariables.config as BackboneConfig | null | undefined) ?? { Memory: null };
      const bcmd: Readonly<JsonObject> =
        "Memory" in c
          ? { memory: { confirm: true } }
          : "Dev" in c
            ? { dev: { path: c.Dev.path } }
            : "Local" in c
              ? { local: { folder: c.Local.folder } }
              : { remote: { url: c.Remote.url, sessionId: c.Remote.sessionId } };
      return [{ backbone: { commands: [{ attachBackbone: bcmd }] } }];
    }
    if (kind === "detachBackbone") return [{ backbone: { commands: [{ detachBackbone: { confirm: true } }] } }];
    if (kind === "listConflicts") return [{ backbone: { commands: [{ listConflicts: { confirm: true } }] } }];
    if (kind === "backboneStatus") return [{ backbone: { commands: [{ backboneStatus: { confirm: true } }] } }];
    if (kind === "syncNow") return [{ backbone: { commands: [{ syncNow: { confirm: true } }] } }];
    if (kind === "resolveConflict")
      return [
        {
          backbone: {
            commands: [
              {
                resolveConflict: {
                  conflictId: String(batchVariables.id),
                  strategy: batchVariables.strategy,
                },
              },
            ],
          },
        },
      ];
    return null;
  }

  /** @emoji 🧾 `kitStore.batch` control-plane entry (session, checkpoint, alternative, backbone). */
  private async runKitStoreBatch(commands: readonly JsonObject[], clientMutationId?: string): Promise<{
    clientMutationId?: string | null;
    results: readonly {
      kind?: string;
      ok?: boolean | null;
      count?: number | null;
      changeKind?: KitChangeSemanticKindGql | null;
      changeKindOther?: string | null;
      inverse?: readonly ChangeKitCommand[] | null;
      conflicts?: readonly JsonObject[];
      backbone?: JsonObject | null;
    }[];
  }> {
    this.ensureAlive();
    const data = kitGraphqlData(
      await this.gqlRun({
        query: KIT_STORE_BATCH_MUTATION,
        variables: { input: { clientMutationId: clientMutationId ?? null, commands: [...commands] } },
      }),
    ) as { kitStore: { batch: { clientMutationId?: string | null; results: readonly JsonObject[] } } };
    return data.kitStore.batch as {
      clientMutationId?: string | null;
      results: readonly {
        kind?: string;
        ok?: boolean | null;
        count?: number | null;
        changeKind?: KitChangeSemanticKindGql | null;
        changeKindOther?: string | null;
        inverse?: readonly ChangeKitCommand[] | null;
        conflicts?: readonly JsonObject[];
        backbone?: JsonObject | null;
      }[];
    };
  }

  /** @emoji 🧾 Top-level `backbone` batch that returns {@link SetResult} (attach, detach, resolve, sync). */
  private async runBackboneBatchSetResult(kind: string, variables: Readonly<JsonObject>): Promise<SetResult> {
    this.ensureAlive();
    const back = this.backboneBatchCommandsFromJson(kind, variables);
    if (!back) return { ok: false, error: { kind: "Internal", message: `backbone batch: unsupported ${kind}` } };
    try {
      const b = await this.runKitStoreBatch(back);
      const r0 = b.results[0] as { ok?: boolean } | undefined;
      if (r0?.ok === false) return { ok: false, error: { kind: "Internal", message: `${kind} rejected` } };
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji 🧾 Backbone batch rows that surface conflicts or status DTOs. */
  private async runBackboneBatchTypedRead<T>(kind: "listConflicts" | "backboneStatus"): Promise<T> {
    this.ensureAlive();
    const back = this.backboneBatchCommandsFromJson(kind, {});
    if (!back) throw new Error(`backbone batch read: unsupported ${kind}`);
    const b = await this.runKitStoreBatch(back);
    const r0 = b.results[0] as
      | { conflicts?: readonly ConflictBatchRecord[] | null; backbone?: JsonValue }
      | undefined;
    if (kind === "listConflicts") return ((r0?.conflicts as readonly ConflictBatchRecord[] | undefined) ?? []) as T;
    const br = (r0?.backbone as Partial<BackboneStatusDto> | null | undefined) ?? {};
    return {
      attached: br.attached ?? false,
      kind: br.kind,
      backboneTip: br.backboneTip,
      pendingWipCheckpoints: br.pendingWipCheckpoints ?? 0,
    } as T;
  }

  /** @emoji 🧾 VCS undo/redo via `kitStore.batch` (open transaction or draft stack). */
  private async runVcsUndoRedo(kind: "undo" | "redo"): Promise<SetResult> {
    this.ensureAlive();
    const t = this.kitWriteTransactionId;
    const d = this.kitWriteDraftId;
    const s = this.kitWriteSessionId;
    try {
      if (t && s && d) {
        const inner = kind === "undo" ? ({ undoTransaction: { count: 1 } } as const) : ({ redoTransaction: { count: 1 } } as const);
        const b = await this.runKitStoreBatch([
          {
            session: {
              sessionId: s,
              commands: [
                {
                  draft: {
                    draftId: d,
                    commands: [{ transaction: { transactionId: t, commands: [inner] } }],
                  },
                },
              ],
            },
          } as unknown as JsonObject,
        ]);
        const r0 = b.results[0] as { ok?: boolean } | undefined;
        if (r0?.ok === false) return { ok: false, error: { kind: "Internal", message: "kit store batch: undo/redo rejected" } };
        return { ok: true };
      }
      if (s && d) {
        const inner = kind === "undo" ? ({ undoDraft: { count: 1 } } as const) : ({ redoDraft: { count: 1 } } as const);
        const b = await this.runKitStoreBatch(
          [
            {
              session: {
                sessionId: s,
                commands: [
                  {
                    draft: {
                      draftId: d,
                      commands: [inner],
                    },
                  },
                ],
              },
            } as unknown as JsonObject,
          ],
        );
        const r0 = b.results[0] as { ok?: boolean } | undefined;
        if (r0?.ok === false) return { ok: false, error: { kind: "Internal", message: "kit store batch: undo/redo rejected" } };
        return { ok: true };
      }
      return {
        ok: false,
        error: { kind: "Internal", message: "undo/redo requires an active draft or open transaction (set anchors via scoped writes)" },
      };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  /** @emoji 🧾 Design/canvas ops mapped into scoped `session → draft → transaction` batch. */
  private async runScopedCanvasControl(kind: string, variables: JsonObject): Promise<SetResult> {
    this.ensureAlive();
    const inner = this.scopedTransactionInnerFromControl(kind, variables);
    if (inner == null) return { ok: false, error: { kind: "NotSupported", message: `no batch mapping for ${kind}` } };
    try {
      const rows = await this.runScopedTransactionBatch([inner]);
      for (const row of rows) {
        if (row.ok === false) return { ok: false, error: { kind: "Internal", message: `batch ${String(row.kind)}` } };
      }
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  async changeKitWithInverse(commands: unknown): Promise<{ kind: KitChangeKind; inverse: readonly ChangeKitCommand[] }> {
    this.ensureAlive();
    const rows = await this.runScopedTransactionBatch([
      { changeKitWithInverse: { commands: (commands as readonly unknown[]) ?? [] } } as JsonObject,
    ]);
    const hit = [...rows].reverse().find((r) => __normKitStoreBatchKind(r.kind) === "CHANGE_KIT_WITH_INVERSE") as
      | { changeKind?: KitChangeSemanticKindGql; changeKindOther?: string | null; inverse?: readonly ChangeKitCommand[] }
      | undefined;
    if (hit != null && hit.changeKind != null) {
      const inv = Array.isArray(hit.inverse) ? (hit.inverse as readonly ChangeKitCommand[]) : [];
      return { kind: kitChangeSemanticKindToGraphQl(hit.changeKind, hit.changeKindOther ?? null), inverse: inv };
    }
    throw new Error("changeKitWithInverse: missing batch row");
  }

  async clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult> {
    return this.runScopedCanvasControl("clusterPieces", { designId, pieceIds: [...pieceIds], clusterName });
  }

  async dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.runScopedCanvasControl("dragPieces", { designId, pieceIds: [...pieceIds], du, dv });
  }

  async movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.runScopedCanvasControl("movePieces", { designId, pieceIds: [...pieceIds], gap, shift, rise });
  }

  async fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult> {
    return this.runScopedCanvasControl("fixPieces", { designId, pieceIds: [...pieceIds] });
  }

  async flattenDesign(designId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("flattenDesign", { designId });
  }

  async expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("expandDesign", { parentDesignId, nestedDesignId });
  }

  async deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("deleteConnection", { designId, connectionId });
  }

  async changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.runScopedCanvasControl("changePieceType", { designId, pieceId, newTypeId });
  }

  async pasteDesignSelection(_designId: string, _selection: KitJsonTreeDto, _plane: PlanePlain | null): Promise<SetResult> {
    return {
      ok: false,
      error: { kind: "NotSupported", message: "pasteDesignSelection: not yet implemented in Rust store" },
    };
  }

  async createHangingPieces(designId: string, typeIds: readonly string[], plane: PlanePlain): Promise<SetResult> {
    return this.runScopedCanvasControl("createHangingPieces", { designId, typeIds: [...typeIds], plane });
  }

  async createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult> {
    return this.runScopedCanvasControl("createConnectedPiece", { designId, parentPiece, parentPort, childType, childPort });
  }

  async createFixedPiece(designId: string, typeId: string, plane: PlanePlain): Promise<SetResult> {
    return this.runScopedCanvasControl("createFixedPiece", { designId, typeId, plane });
  }

  async undo(): Promise<SetResult> {
    return this.runVcsUndoRedo("undo");
  }

  async redo(): Promise<SetResult> {
    return this.runVcsUndoRedo("redo");
  }

  async attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
    return this.runBackboneBatchSetResult("attachBackbone", { config: cfg });
  }

  async detachBackbone(): Promise<SetResult> {
    return this.runBackboneBatchSetResult("detachBackbone", {});
  }

  async backboneStatus(): Promise<BackboneStatusDto> {
    return this.runBackboneBatchTypedRead<BackboneStatusDto>("backboneStatus");
  }

  async listConflicts(): Promise<KitConflict[]> {
    const raw = await this.runBackboneBatchTypedRead<readonly ConflictBatchRecord[]>("listConflicts");
    if (Array.isArray(raw)) return raw as KitConflict[];
    return [];
  }

  async resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
    return this.runBackboneBatchSetResult("resolveConflict", { id, strategy });
  }

  async syncNow(): Promise<SetResult> {
    return this.runBackboneBatchSetResult("syncNow", {});
  }

  /**
   * @emoji 🧭 Read-only flatten map rows for one design (`semio/rs` `flatten_map`), for algorithm / MCP tooling.
   */
  async readDesignFlattenMap(scope: KitReadScope, designId: string): Promise<readonly DesignFlattenMapEntryDto[]> {
    const data = kitGraphqlData(
      await this.gqlRunWithReadScope(scope, {
        query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { flattenMap } } }`,
        variables: { id: designId },
      }),
    ) as { kit?: { designByDtoId?: { flattenMap?: JsonValue } | null } | null };
    const raw = gqlDataKitRoot(data)?.designByDtoId?.flattenMap;
    const FlatRow = z.object({ pieceId: z.string(), plane: PlaneSchema, center: PointSchema });
    if (Array.isArray(raw)) {
      const out: DesignFlattenMapEntryDto[] = [];
      for (const row of raw) {
        const pr = FlatRow.safeParse(row);
        if (pr.success) out.push(pr.data);
      }
      return out;
    }
    if (typeof raw === "string") {
      try {
        const p = parseJsonValue(raw) as JsonValue;
        if (!Array.isArray(p)) return [];
        const out: DesignFlattenMapEntryDto[] = [];
        for (const row of p) {
          const pr = FlatRow.safeParse(row);
          if (pr.success) out.push(pr.data);
        }
        return out;
      } catch {
        return [];
      }
    }
    return [];
  }

  async read(scope: KitReadScope, batch: ReadBatch): Promise<ReadBatchResult> {
    this.ensureAlive();
    const out: ReadKitCommandOutput[] = [];
    for (const c of batch) out.push(await this.mapReadCommand(scope, c));
    return out;
  }

  /** @emoji 🧾 Apply typed `ChangeKitCommand` batch inside the active session draft transaction (`kitStore.batch`). */
  async submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult> {
    try {
      const rows = await this.runScopedTransactionBatch([{ changeKitCommands: { commands: [...commands] } } as JsonObject]);
      for (const row of rows) {
        if (row.ok === false) return { ok: false, error: { kind: "Internal", message: `batch ${String(row.kind)}` } };
      }
      return { ok: true };
    } catch (e) {
      return { ok: false, error: { kind: "Internal", message: String(e) } };
    }
  }

  private async mapReadCommand(scope: KitReadScope, c: ReadKitCommand): Promise<ReadKitCommandOutput> {
    if ("readKitFullCommand" in c && c.readKitFullCommand === null) {
      const d = await this.materializedLiveJsonForReadScope(scope);
      return { readKitFullCommand: { full: d } };
    }
    if ("readKitShallowCommand" in c && c.readKitShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "typesShallow designsShallow");
      return {
        readKitShallowCommand: {
          types: semioParseTypeShallowArrayJson(row.typesShallow as KitJsonTreeDto | string),
          designs: semioParseDesignShallowArrayJson(row.designsShallow as KitJsonTreeDto | string),
        },
      };
    }
    if ("readKitTypeIdsCommand" in c && c.readKitTypeIdsCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "typeIds");
      return { readKitTypeIdsCommand: { typeIds: semioParseKitIdDtoArray(row.typeIds as KitJsonTreeDto | string) } };
    }
    if ("readKitDesignIdsCommand" in c && c.readKitDesignIdsCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "designIds");
      return { readKitDesignIdsCommand: { designIds: semioParseKitIdDtoArray(row.designIds as KitJsonTreeDto | string) } };
    }
    if ("readKitTypesMetadataCommand" in c && c.readKitTypesMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(
        scope,
        "typesMetadata { id name description icon image stock typeVirtual unit location { id } created updated }",
      );
      return { readKitTypesMetadataCommand: { types: semioParseTypeMetadataArrayJson(row.typesMetadata as KitJsonTreeDto | string) } };
    }
    if ("readKitDesignsMetadataCommand" in c && c.readKitDesignsMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(
        scope,
        "designsMetadata { id name description icon image location { id } unit created updated kit { id } }",
      );
      return { readKitDesignsMetadataCommand: { designs: semioParseDesignMetadataArrayJson(row.designsMetadata as KitJsonTreeDto | string) } };
    }
    if ("readKitTypesShallowCommand" in c && c.readKitTypesShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "typesShallow");
      return { readKitTypesShallowCommand: { types: semioParseTypeShallowArrayJson(row.typesShallow as KitJsonTreeDto | string) } };
    }
    if ("readKitDesignsShallowCommand" in c && c.readKitDesignsShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "designsShallow");
      return { readKitDesignsShallowCommand: { designs: semioParseDesignShallowArrayJson(row.designsShallow as KitJsonTreeDto | string) } };
    }
    if ("readKitAuthorsShallowCommand" in c && c.readKitAuthorsShallowCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "authorsShallow");
      return { readKitAuthorsShallowCommand: { authors: semioParseAuthorMetadataArrayJson(row.authorsShallow as KitJsonTreeDto | string) } };
    }
    if ("readKitMetadataCommand" in c && c.readKitMetadataCommand === null) {
      const row = await this.gqlKitReadOnlyScope(
        scope,
        "kitMetadata { id name description icon image preview remote homepage license uri created updated version }",
      );
      return { readKitMetadataCommand: { metadata: semioParseKitMetadataJson(row.kitMetadata as KitJsonTreeDto) } };
    }
    if ("readKitColoredConnectorsCommand" in c && c.readKitColoredConnectorsCommand === null) {
      const row = await this.gqlKitReadOnlyScope(scope, "coloredConnectors { typeId { id } connectorId { id } color }");
      return {
        readKitColoredConnectorsCommand: {
          rows: semioParseColoredConnectorRowsJson(row.coloredConnectors as KitJsonTreeDto | readonly KitJsonTreeDto[]),
        },
      };
    }
    if ("readKitDesignCommands" in c && c.readKitDesignCommands) {
      const { id, commands } = c.readKitDesignCommands;
      const results: ReadDesignCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapDesignRead(scope, id.id, sub));
      return { readKitDesignCommands: { results } };
    }
    if ("readKitTypeCommands" in c && c.readKitTypeCommands) {
      const { id, commands } = c.readKitTypeCommands;
      const results: ReadTypeCommandOutput[] = [];
      for (const sub of commands) results.push(await this.mapTypeRead(scope, id.id, sub));
      return { readKitTypeCommands: { results } };
    }
    throw new Error(`read: unsupported ${Object.keys(c).join(",")}`);
  }

  private async mapDesignRead(scope: KitReadScope, designId: string, cmd: ReadDesignCommand): Promise<ReadDesignCommandOutput> {
    if ("readDesignPiecesFullCommand" in cmd && cmd.readDesignPiecesFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { piecesFull } } }`,
          variables: { id: designId },
        }),
      ) as { kit?: { designByDtoId?: { piecesFull?: JsonValue } | null } | null };
      return { readDesignPiecesFullCommand: { pieces: semioParsePiecePlainArrayJson(gqlDataKitRoot(d)?.designByDtoId?.piecesFull as KitJsonTreeDto | string) } };
    }
    if ("readDesignConnectionsFullCommand" in cmd && cmd.readDesignConnectionsFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { connectionsFull } } }`,
          variables: { id: designId },
        }),
      ) as { kit?: { designByDtoId?: { connectionsFull?: JsonValue } | null } | null };
      return {
        readDesignConnectionsFullCommand: {
          connections: semioParseConnectionPlainArrayJson(gqlDataKitRoot(d)?.designByDtoId?.connectionsFull as KitJsonTreeDto | string),
        },
      };
    }
    if ("readDesignPieceCommands" in cmd && cmd.readDesignPieceCommands) {
      const { id, commands } = cmd.readDesignPieceCommands;
      const results: ReadPieceCommandOutput[] = [];
      for (const pc of commands) results.push(await this.mapPieceRead(scope, designId, id.id, pc));
      return { readDesignPieceCommands: { results } };
    }
    if ("readDesignClusterableGroupsCommand" in cmd && cmd.readDesignClusterableGroupsCommand) {
      const sel = cmd.readDesignClusterableGroupsCommand.selection.map((s) => s.id);
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!, $sel: [String!]!) { kit(scope: $scope) { designByDtoId(id: $id) { clusterableGroups(selection: $sel) } } }`,
          variables: { id: designId, sel },
        }),
      ) as { kit?: { designByDtoId?: { clusterableGroups?: JsonValue } | null } | null };
      const raw = gqlDataKitRoot(d)?.designByDtoId?.clusterableGroups;
      const groups: readonly (readonly KitIdDto[])[] = Array.isArray(raw)
        ? raw.map((row: unknown) => (Array.isArray(row) ? row.map((pid: unknown): KitIdDto => ({ id: String(pid) })) : []))
        : [];
      return { readDesignClusterableGroupsCommand: { groups } };
    }
    if ("readDesignIncludedDesignsCommand" in cmd && cmd.readDesignIncludedDesignsCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { includedDesigns } } }`,
          variables: { id: designId },
        }),
      ) as { kit?: { designByDtoId?: { includedDesigns?: JsonValue } | null } | null };
      return {
        readDesignIncludedDesignsCommand: {
          designs: semioParseDesignIncludedDesignArrayJson(gqlDataKitRoot(d)?.designByDtoId?.includedDesigns as KitJsonTreeDto | readonly KitJsonTreeDto[]),
        },
      };
    }
    if ("readDesignQualitySumCommand" in cmd && cmd.readDesignQualitySumCommand) {
      const qid = cmd.readDesignQualitySumCommand.qualityId.id;
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!, $qid: String!) { kit(scope: $scope) { designByDtoId(id: $id) { qualitySum(qualityId: $qid) } } }`,
          variables: { id: designId, qid },
        }),
      ) as { kit?: { designByDtoId?: { qualitySum?: number } | null } | null };
      return { readDesignQualitySumCommand: { sum: gqlDataKitRoot(d)?.designByDtoId?.qualitySum ?? 0 } };
    }
    if ("readDesignReplaceableCatalogCommand" in cmd && cmd.readDesignReplaceableCatalogCommand) {
      const sel = cmd.readDesignReplaceableCatalogCommand.selection.map((s) => s.id);
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!, $sel: [String!]!) { kit(scope: $scope) { designByDtoId(id: $id) { replaceableCatalog(selection: $sel) { typeIds designIds } } } }`,
          variables: { id: designId, sel },
        }),
      ) as { kit?: { designByDtoId?: { replaceableCatalog?: { typeIds?: string[]; designIds?: string[] } } | null } | null };
      const rc = gqlDataKitRoot(d)?.designByDtoId?.replaceableCatalog;
      return {
        readDesignReplaceableCatalogCommand: {
          types: (rc?.typeIds ?? []).map((id: string) => ({ id: String(id) })),
          designs: (rc?.designIds ?? []).map((id: string) => ({ id: String(id) })),
        },
      };
    }
    if ("readDesignIncludedDesignIdsCommand" in cmd && cmd.readDesignIncludedDesignIdsCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { includedDesignIds } } }`,
          variables: { id: designId },
        }),
      ) as { kit?: { designByDtoId?: { includedDesignIds?: string[] } | null } | null };
      return { readDesignIncludedDesignIdsCommand: { designIds: gqlDataKitRoot(d)?.designByDtoId?.includedDesignIds ?? [] } };
    }
    throw new Error(`readDesign: ${Object.keys(cmd).join(",")}`);
  }

  private async mapPieceRead(scope: KitReadScope, designId: string, pieceId: string, cmd: ReadPieceCommand): Promise<ReadPieceCommandOutput> {
    if ("readPieceFlatPlaneCommand" in cmd && cmd.readPieceFlatPlaneCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $d: String!, $p: String!) { kit(scope: $scope) { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatPlane } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kit?: { designByDtoId?: { pieceByDtoId?: { flatPlane?: JsonValue } | null } | null } | null };
      return {
        readPieceFlatPlaneCommand: { flatPlane: semioParsePlaneNullableJson(gqlDataKitRoot(d)?.designByDtoId?.pieceByDtoId?.flatPlane as KitJsonTreeDto) },
      };
    }
    if ("readPieceFlatCenterCommand" in cmd && cmd.readPieceFlatCenterCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $d: String!, $p: String!) { kit(scope: $scope) { designByDtoId(id: $d) { pieceByDtoId(id: $p) { flatCenter } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kit?: { designByDtoId?: { pieceByDtoId?: { flatCenter?: JsonValue } | null } | null } | null };
      return {
        readPieceFlatCenterCommand: {
          flatCenter: semioParseCoordinateNullableJson(gqlDataKitRoot(d)?.designByDtoId?.pieceByDtoId?.flatCenter as KitJsonTreeDto),
        },
      };
    }
    if ("readPieceParentConnectionFullCommand" in cmd && cmd.readPieceParentConnectionFullCommand === null) {
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $d: String!, $p: String!) { kit(scope: $scope) { designByDtoId(id: $d) { pieceByDtoId(id: $p) { parentConnection { id gap shift rise rotation turn tilt u v description } } } } }`,
          variables: { d: designId, p: pieceId },
        }),
      ) as { kit?: { designByDtoId?: { pieceByDtoId?: { parentConnection?: JsonValue } | null } | null } | null };
      return {
        readPieceParentConnectionFullCommand: {
          connection: semioParseConnectionNullableJson(gqlDataKitRoot(d)?.designByDtoId?.pieceByDtoId?.parentConnection as KitJsonTreeDto),
        },
      };
    }
    throw new Error(`readPiece: ${Object.keys(cmd).join(",")}`);
  }

  private async mapTypeRead(scope: KitReadScope, typeId: string, cmd: ReadTypeCommand): Promise<ReadTypeCommandOutput> {
    if ("readTypeBestRepresentationCommand" in cmd && cmd.readTypeBestRepresentationCommand) {
      const tags = cmd.readTypeBestRepresentationCommand.tagIds;
      const d = kitGraphqlData(
        await this.gqlRunWithReadScope(scope, {
          query: `query($scope: KitReadScopeInput!, $id: String!, $tags: [String!]!) { kit(scope: $scope) { typeByDtoId(id: $id) { bestRepresentation(tagIds: $tags) } } }`,
          variables: { id: typeId, tags: [...tags] },
        }),
      ) as { kit?: { typeByDtoId?: { bestRepresentation?: JsonValue } | null } | null };
      return {
        readTypeBestRepresentationCommand: {
          representation: semioParseRepresentationNullableJson(gqlDataKitRoot(d)?.typeByDtoId?.bestRepresentation as KitJsonTreeDto),
        },
      };
    }
    throw new Error(`readType: ${Object.keys(cmd).join(",")}`);
  }

  async getPiecesMetadata(scope: KitReadScope, designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    const d = kitGraphqlData(
      await this.gqlRunWithReadScope(scope, {
        query: `query($scope: KitReadScopeInput!, $id: String!) { kit(scope: $scope) { designByDtoId(id: $id) { piecePlacement { pieceId fixedPieceId parentPieceId depth path plane { origin { x y z } xAxis { x y z } yAxis { x y z } } center { x y z } } } } } }`,
        variables: { id: designId },
      }),
    ) as { kit?: { designByDtoId?: { piecePlacement?: readonly JsonValue[] } | null } | null };
    const rows = gqlDataKitRoot(d)?.designByDtoId?.piecePlacement;
    return semioParsePiecePlacementMapJson(rows);
  }

  async getPieces(scope: KitReadScope, designId: string): Promise<readonly PiecePlain[]> {
    const out = await this.read(scope, [{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignPiecesFullCommand: null }] } }]);
    const row = out[0];
    if (row && "readKitDesignCommands" in row) {
      const sub = row.readKitDesignCommands.results[0];
      if (sub && "readDesignPiecesFullCommand" in sub) return sub.readDesignPiecesFullCommand.pieces;
    }
    return [];
  }

  async getConnections(scope: KitReadScope, designId: string): Promise<readonly ConnectionPlain[]> {
    const out = await this.read(scope, [{ readKitDesignCommands: { id: { id: designId }, commands: [{ readDesignConnectionsFullCommand: null }] } }]);
    const row = out[0];
    if (row && "readKitDesignCommands" in row) {
      const sub = row.readKitDesignCommands.results[0];
      if (sub && "readDesignConnectionsFullCommand" in sub) return sub.readDesignConnectionsFullCommand.connections;
    }
    return [];
  }

  async getDesigns(scope: KitReadScope): Promise<readonly DesignShallow[]> {
    const out = await this.read(scope, [{ readKitDesignsShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitDesignsShallowCommand" in row) return row.readKitDesignsShallowCommand.designs;
    return [];
  }

  async getTypes(scope: KitReadScope): Promise<readonly TypeShallow[]> {
    const out = await this.read(scope, [{ readKitTypesShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitTypesShallowCommand" in row) return row.readKitTypesShallowCommand.types;
    return [];
  }

  async getAuthors(scope: KitReadScope): Promise<readonly AuthorMetadataDto[]> {
    const out = await this.read(scope, [{ readKitAuthorsShallowCommand: null }]);
    const row = out[0];
    if (row && "readKitAuthorsShallowCommand" in row) return row.readKitAuthorsShallowCommand.authors;
    return [];
  }

  async getKitMetadata(scope: KitReadScope): Promise<KitMetadataDto | null> {
    const out = await this.read(scope, [{ readKitMetadataCommand: null }]);
    const row = out[0];
    if (row && "readKitMetadataCommand" in row) return row.readKitMetadataCommand.metadata;
    return null;
  }

  // #region KitStoreEntityFactories
  /** @emoji 🧭 Sync handle for kit-scoped design reads and mutations (no I/O). */
  design(id: string, readScope: KitReadScope = theKitReadScope): DesignStore {
    return new DesignStore(this, id, readScope);
  }
  /** @emoji 🧭 Sync handle for kit-scoped kind reads and mutations (no I/O). */
  type(id: string, readScope: KitReadScope = theKitReadScope): TypeStore {
    return new TypeStore(this, id, readScope);
  }
  /** @emoji 🧭 Sync handle for a piece within a design (no I/O). */
  piece(designId: string, id: string, readScope: KitReadScope = theKitReadScope): PieceStore {
    return new PieceStore(this, designId, id, readScope);
  }
  /** @emoji 🧭 Sync handle for a connection within a design (no I/O). */
  connection(designId: string, id: string, readScope: KitReadScope = theKitReadScope): ConnectionStore {
    return new ConnectionStore(this, designId, id, readScope);
  }
  family(id: string, readScope: KitReadScope = theKitReadScope): FamilyStore {
    return new FamilyStore(this, id, readScope);
  }
  file(id: string, readScope: KitReadScope = theKitReadScope): FileStore {
    return new FileStore(this, id, readScope);
  }
  folder(id: string, readScope: KitReadScope = theKitReadScope): FolderStore {
    return new FolderStore(this, id, readScope);
  }

  /** @emoji 🧭 All design ids in the live kit as {@link DesignStore} handles. */
  async designs(scope: KitReadScope = theKitReadScope): Promise<readonly DesignStore[]> {
    const out = await this.read(scope, [{ readKitDesignIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitDesignIdsCommand?: { designIds?: JsonValue } }).readKitDesignIdsCommand?.designIds,
    );
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => this.design(toId(row), scope)).filter((s) => s.id !== "");
  }

  /** @emoji 🧭 All kind ids in the live kit as {@link TypeStore} handles. */
  async types(scope: KitReadScope = theKitReadScope): Promise<readonly TypeStore[]> {
    const out = await this.read(scope, [{ readKitTypeIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitTypeIdsCommand?: { typeIds?: JsonValue } }).readKitTypeIdsCommand?.typeIds,
    );
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => this.type(toId(row), scope)).filter((s) => s.id !== "");
  }

  /** @emoji 🧾 Design row id strings from `readKitDesignIdsCommand` (no {@link DesignStore} allocation). */
  async designRowIds(scope: KitReadScope = theKitReadScope): Promise<readonly string[]> {
    const out = await this.read(scope, [{ readKitDesignIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitDesignIdsCommand?: { designIds?: JsonValue } }).readKitDesignIdsCommand?.designIds,
    );
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => toId(row)).filter((s) => s !== "");
  }

  /** @emoji 🧾 Kind row id strings from `readKitTypeIdsCommand` (no {@link TypeStore} allocation). */
  async kindRowIds(scope: KitReadScope = theKitReadScope): Promise<readonly string[]> {
    const out = await this.read(scope, [{ readKitTypeIdsCommand: null }]);
    const ids = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitTypeIdsCommand?: { typeIds?: JsonValue } }).readKitTypeIdsCommand?.typeIds,
    );
    const toId = (row: unknown): string => {
      if (typeof row === "string") return row;
      if (row && typeof row === "object" && "id" in row && typeof (row as { id: unknown }).id === "string") return (row as { id: string }).id;
      return "";
    };
    return ids.map((row) => toId(row)).filter((s) => s !== "");
  }

  /** @emoji 🧾 Per-kind metadata rows (`readKitTypesMetadataCommand`). */
  async kindMetadataRows(scope: KitReadScope = theKitReadScope): Promise<readonly unknown[]> {
    const out = await this.read(scope, [{ readKitTypesMetadataCommand: null }]);
    return kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitTypesMetadataCommand?: { types?: JsonValue } }).readKitTypesMetadataCommand?.types,
    );
  }

  /** @emoji 🧾 Per-design metadata rows (`readKitDesignsMetadataCommand`). */
  async designMetadataRows(scope: KitReadScope = theKitReadScope): Promise<readonly unknown[]> {
    const out = await this.read(scope, [{ readKitDesignsMetadataCommand: null }]);
    return kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitDesignsMetadataCommand?: { designs?: JsonValue } }).readKitDesignsMetadataCommand?.designs,
    );
  }
  // #endregion KitStoreEntityFactories
}

// #endregion 📦KitStore

// #region 🧰OpenKit

/**
 * @emoji 🧰 Convenience alias for {@link KitStore.open}.
 */
export async function openKit(initialKit: KitFullDto, opts?: KitStoreOpenOptions): Promise<KitStore> {
  return KitStore.open(initialKit, opts);
}

// #endregion 🧰OpenKit

// #region KitEventEntityFilter
/** @emoji 🧾 Whether a {@link KitChange} references a design id in forward or inverse commands. */
function kitChangeTouchesDesignId(change: KitChange, designId: string): boolean {
  for (const cmd of [...change.forward, ...change.inverse]) {
    if (jsonSubtreeHasIdKey(cmd, "designId", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "design_id", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "parentDesignId", designId)) return true;
    if (jsonSubtreeHasIdKey(cmd, "nestedDesignId", designId)) return true;
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given design id. */
function kitClassifiedMutationTouchesDesign(ev: KitClassifiedMutationEvent, designId: string): boolean {
  if ("renamedDesign" in ev && ev.renamedDesign.designId === designId) return true;
  if ("draggedFlatCenterPiece" in ev && ev.draggedFlatCenterPiece.designId === designId) return true;
  if ("movedPiecesFlatCenter" in ev && ev.movedPiecesFlatCenter.designId === designId) return true;
  if ("clusteredPieces" in ev && ev.clusteredPieces.designId === designId) return true;
  if ("fixedPiecesFlatCenter" in ev && ev.fixedPiecesFlatCenter.designId === designId) return true;
  if ("flattenedDesign" in ev && ev.flattenedDesign.designId === designId) return true;
  if (
    "expandedNestedDesign" in ev &&
    (ev.expandedNestedDesign.parentDesignId === designId || ev.expandedNestedDesign.nestedDesignId === designId)
  )
    return true;
  if ("deletedConnection" in ev && ev.deletedConnection.designId === designId) return true;
  if ("changedPieceKind" in ev && ev.changedPieceKind.designId === designId) return true;
  if ("changedDesignCommands" in ev && ev.changedDesignCommands.designId === designId) return true;
  if ("changedKit" in ev && kitChangeTouchesDesignId(ev.changedKit.change, designId)) return true;
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given kind id. */
function kitClassifiedMutationTouchesType(ev: KitClassifiedMutationEvent, typeId: string): boolean {
  if ("renamedType" in ev && ev.renamedType.typeId === typeId) return true;
  if ("changedTypeCommands" in ev && ev.changedTypeCommands.typeId === typeId) return true;
  if ("changedPieceKind" in ev && jsonSubtreeHasIdKey(ev.changedPieceKind.change, "newTypeId", typeId)) return true;
  if ("changedKit" in ev) {
    const ch = ev.changedKit.change;
    for (const cmd of [...ch.forward, ...ch.inverse]) {
      if (jsonSubtreeHasIdKey(cmd, "typeId", typeId)) return true;
      if (jsonSubtreeHasIdKey(cmd, "type_id", typeId)) return true;
    }
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given piece in a design. */
function kitClassifiedMutationTouchesPiece(ev: KitClassifiedMutationEvent, designId: string, pieceId: string): boolean {
  if ("changedPieceKind" in ev && ev.changedPieceKind.designId === designId && ev.changedPieceKind.pieceId === pieceId) return true;
  if ("draggedFlatCenterPiece" in ev && ev.draggedFlatCenterPiece.designId === designId && ev.draggedFlatCenterPiece.pieceIds.includes(pieceId))
    return true;
  if ("movedPiecesFlatCenter" in ev && ev.movedPiecesFlatCenter.designId === designId && ev.movedPiecesFlatCenter.pieceIds.includes(pieceId))
    return true;
  if ("clusteredPieces" in ev && ev.clusteredPieces.designId === designId && ev.clusteredPieces.pieceIds.includes(pieceId)) return true;
  if ("fixedPiecesFlatCenter" in ev && ev.fixedPiecesFlatCenter.designId === designId && ev.fixedPiecesFlatCenter.pieceIds.includes(pieceId))
    return true;
  if ("changedDesignCommands" in ev && ev.changedDesignCommands.designId === designId) {
    return jsonSubtreeHasIdKey(ev.changedDesignCommands.change, "pieceId", pieceId);
  }
  if ("changedKit" in ev && kitChangeTouchesDesignId(ev.changedKit.change, designId)) {
    return jsonSubtreeHasIdKey(ev.changedKit.change, "pieceId", pieceId);
  }
  return false;
}

/** @emoji 🧾 Whether a {@link KitClassifiedMutationEvent} concerns the given connection in a design. */
function kitClassifiedMutationTouchesConnection(ev: KitClassifiedMutationEvent, designId: string, connectionId: string): boolean {
  if ("deletedConnection" in ev && ev.deletedConnection.designId === designId && ev.deletedConnection.connectionId === connectionId) return true;
  if (kitClassifiedMutationTouchesDesign(ev, designId) && "changedKit" in ev) {
    return jsonSubtreeHasIdKey(ev.changedKit.change, "connectionId", connectionId);
  }
  return false;
}

/** @emoji 🧪 True when JSON-like subtree (including kit command atoms) contains a string field `key` equal to `id`. */
function jsonSubtreeHasIdKey(raw: unknown, key: string, id: string): boolean {
  if (raw == null) return false;
  if (typeof raw === "string") return false;
  if (typeof raw === "number" || typeof raw === "boolean") return false;
  if (Array.isArray(raw)) {
    for (const x of raw) if (jsonSubtreeHasIdKey(x, key, id)) return true;
    return false;
  }
  if (typeof raw === "object") {
    const o = raw as { readonly [k: string]: unknown };
    const v = o[key];
    if (typeof v === "string" && v === id) return true;
    for (const k of Object.keys(o)) if (jsonSubtreeHasIdKey(o[k], key, id)) return true;
  }
  return false;
}

/** @emoji 🧭 Design-scoped kit events (excludes bare `Changed` and `FlattenInvalidated`, which are handled separately per subscriber). */
export function kitEventTouchesDesignStrict(ev: KitEvent, designId: string): boolean {
  if (designId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesDesign(ev, designId)) return true;
  const d = (ev as { Design?: { design_id?: string; event?: JsonValue } }).Design;
  if (d && typeof d.design_id === "string" && d.design_id === designId) return true;
  if (jsonSubtreeHasIdKey(ev, "design_id", designId)) return true;
  const ca = (ev as { ChildAdded?: { parent?: { id?: string }; child?: { id?: string } } }).ChildAdded;
  if (ca && ca.parent?.id === designId) return true;
  const cr = (ev as { ChildRemoved?: { parent?: { id?: string }; child?: { id?: string } } }).ChildRemoved;
  if (cr && cr.parent?.id === designId) return true;
  return false;
}

/** @emoji 🧭 Whether a subscription {@link KitEvent} likely concerns the given design (includes kit-wide invalidations). */
export function kitEventTouchesDesign(ev: KitEvent, designId: string): boolean {
  if (designId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  if ("ValidationInvalidated" in ev && (ev as { ValidationInvalidated?: null }).ValidationInvalidated === null) return true;
  const fi = (ev as { FlattenInvalidated?: { design?: string; pieces?: readonly string[] } }).FlattenInvalidated;
  if (fi && typeof fi.design === "string" && fi.design === designId) return true;
  return kitEventTouchesDesignStrict(ev, designId);
}

/** @emoji 🧭 Type-scoped events (no bare `Changed`). */
export function kitEventTouchesTypeStrict(ev: KitEvent, typeId: string): boolean {
  if (typeId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesType(ev, typeId)) return true;
  const t = (ev as { Type?: { type_id?: string } }).Type;
  if (t && typeof t.type_id === "string" && t.type_id === typeId) return true;
  if (jsonSubtreeHasIdKey(ev, "type_id", typeId)) return true;
  const ca = (ev as { ChildAdded?: { parent?: { id?: string; kind?: string }; child?: { id?: string } } }).ChildAdded;
  if (ca?.parent?.kind === "Type" && ca.parent.id === typeId) return true;
  const cr = (ev as { ChildRemoved?: { parent?: { id?: string; kind?: string }; child?: { id?: string } } }).ChildRemoved;
  if (cr?.parent?.kind === "Type" && cr.parent.id === typeId) return true;
  return false;
}

/** @emoji 🧭 Whether a subscription {@link KitEvent} likely concerns the given kind id. */
export function kitEventTouchesType(ev: KitEvent, typeId: string): boolean {
  if (typeId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  if ("ValidationInvalidated" in ev && (ev as { ValidationInvalidated?: null }).ValidationInvalidated === null) return true;
  return kitEventTouchesTypeStrict(ev, typeId);
}

/** @emoji 🧭 Piece-scoped events (design-scoped strict + piece id + flatten rows). */
export function kitEventTouchesPiece(ev: KitEvent, designId: string, pieceId: string): boolean {
  if (pieceId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesPiece(ev, designId, pieceId)) return true;
  if (kitEventTouchesDesignStrict(ev, designId)) return true;
  const p = (ev as { Piece?: { piece_id?: string } }).Piece;
  if (p && typeof p.piece_id === "string" && p.piece_id === pieceId) return true;
  if (jsonSubtreeHasIdKey(ev, "piece_id", pieceId)) return true;
  const fi = (ev as { FlattenInvalidated?: { design?: string; pieces?: string[] } }).FlattenInvalidated;
  if (fi && fi.design === designId) {
    const rows = fi.pieces;
    if (!Array.isArray(rows) || rows.length === 0) return true;
    return rows.includes(pieceId);
  }
  return false;
}

/** @emoji 🧭 Connection-scoped events (design-scoped strict + connection id). */
export function kitEventTouchesConnection(ev: KitEvent, designId: string, connectionId: string): boolean {
  if (connectionId === "") return false;
  if (isKitClassifiedMutationEvent(ev) && kitClassifiedMutationTouchesConnection(ev, designId, connectionId)) return true;
  if (kitEventTouchesDesignStrict(ev, designId)) return true;
  const c = (ev as { Connection?: { connection_id?: string } }).Connection;
  if (c && typeof c.connection_id === "string" && c.connection_id === connectionId) return true;
  if (jsonSubtreeHasIdKey(ev, "connection_id", connectionId)) return true;
  return false;
}

/** @emoji 🧭 Family / file / folder entity filters (ChildAdded paths + id fields). */
export function kitEventTouchesFamily(ev: KitEvent, familyId: string): boolean {
  if (familyId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { Family?: { family_id?: string } }).Family;
  if (f && typeof f.family_id === "string" && f.family_id === familyId) return true;
  if (jsonSubtreeHasIdKey(ev, "family_id", familyId)) return true;
  return false;
}

export function kitEventTouchesFile(ev: KitEvent, fileId: string): boolean {
  if (fileId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { File?: { file_id?: string } }).File;
  if (f && typeof f.file_id === "string" && f.file_id === fileId) return true;
  if (jsonSubtreeHasIdKey(ev, "file_id", fileId)) return true;
  return false;
}

export function kitEventTouchesFolder(ev: KitEvent, folderId: string): boolean {
  if (folderId === "") return false;
  if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) return true;
  const f = (ev as { Folder?: { folder_id?: string } }).Folder;
  if (f && typeof f.folder_id === "string" && f.folder_id === folderId) return true;
  if (jsonSubtreeHasIdKey(ev, "folder_id", folderId)) return true;
  return false;
}
// #endregion KitEventEntityFilter

// #region 🧩KitWasmBridgeMerged
// #region 🔌KitStoreClientTypes

export type KitStoreExecuteResult = { ok: true; result: JsonValue } | { ok: false; error: SetError };

export type WriteStatus =
  | { kind: "readonly"; pending: 0; lastError?: SetError }
  | { kind: "idle"; pending: 0; lastError?: SetError }
  | { kind: "pending"; pending: number }
  | { kind: "error"; pending: 0; lastError?: SetError };

/** @emoji 🧾 Sketchpad string-command context/result (opaque JSON). */
export type KitCommandContext = JsonObject;
export type KitCommandResult = JsonObject;

/** @emoji 🧾 Typed kit mutation envelope for React facades (`kitStore.batch` transaction `changeKitCommands`). */
type KitTypedChangeKitCommandsBatch = { readonly kind: "changeKitCommands"; readonly commands: readonly ChangeKitCommand[] };

/** @emoji 🧾 Typed `changeKitCommands` batch facade for React (opaque to string command routers). */
export type SemioKitCommandFacade = { runMutation(cmd: KitTypedChangeKitCommandsBatch): Promise<SetResult> };

export type KitStoreReadSnap = { readonly version: number; readonly data: unknown; readonly pending: number };

export type KitDesignReadKind = "metadata" | "pieces" | "connections";
export type KitShallowListKind = "designs" | "types" | "authors";
export type KitViewCatalogKey = "typeIds" | "typesMetadata" | "designIds" | "designsMetadata";

/** @emoji 🧾 Minimal async bridge for pulling authoritative kit JSON into a {@link KitHostStore}. */
export type SemioKitBridge = { fetchFullKit(): Promise<KitFullDto> };

/** @emoji 🧾 Browser / test kit RPC surface used by React hooks (wraps {@link KitStore}). */
export type KitStoreClient = SemioKitBridge & {
  /** @emoji 🧾 Materialized read scope (see {@link WasmKitStoreClient#kitReadScope} / {@link getKitClientReadScope}). */
  readonly kitReadScope: KitReadScope;
  getKitWriteScope(): KitWriteScope | null;
  setKitWriteScope(scope: KitWriteScope | null): void;
  finalizeKitWriteTransaction(): Promise<SetResult>;
  abortKitWriteTransaction(): Promise<SetResult>;
  clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult>;
  dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult>;
  movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult>;
  fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult>;
  flattenDesign(designId: string): Promise<SetResult>;
  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult>;
  deleteConnection(designId: string, connectionId: string): Promise<SetResult>;
  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult>;
  pasteDesignSelection(designId: string, selection: KitJsonTreeDto, plane: PlanePlain | null): Promise<SetResult>;
  createHangingPieces(designId: string, typeIds: readonly string[], plane: PlanePlain): Promise<SetResult>;
  createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult>;
  createFixedPiece(designId: string, typeId: string, plane: PlanePlain): Promise<SetResult>;
  submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult>;
  undo(): Promise<SetResult>;
  redo(): Promise<SetResult>;
  canUndo(): Promise<boolean>;
  canRedo(): Promise<boolean>;
  getPiecesMetadata(designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>>;
  getPieces(designId: string): Promise<readonly PiecePlain[]>;
  getConnections(designId: string): Promise<readonly ConnectionPlain[]>;
  getDesigns(): Promise<readonly DesignShallow[]>;
  getTypes(): Promise<readonly TypeShallow[]>;
  getAuthors(): Promise<readonly AuthorMetadataDto[]>;
  getKitMetadata(): Promise<KitMetadataDto | null>;
  backboneStatus(): Promise<BackboneStatusDto>;
  attachBackbone(cfg: BackboneConfig): Promise<SetResult>;
  detachBackbone(): Promise<SetResult>;
  listConflicts(): Promise<KitConflict[]>;
  resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult>;
  syncNow(): Promise<SetResult>;
  kitGraphql(): LiveKitRoot;
  subscribe(cb: (ev: KitEvent) => void): () => void;
  readPieceFlatPlane(designId: string, pieceId: string): Promise<PlanePlain | null>;
  readPieceFlatCenter(designId: string, pieceId: string): Promise<CoordinatePlain | null>;
  readPieceParentConnectionFull(designId: string, pieceId: string): Promise<ConnectionPlain | null>;
  readDesignIncludedDesigns(designId: string): Promise<readonly IncludedDesignInfoDto[]>;
  readDesignClusterableGroups(designId: string, selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]>;
  readDesignQualitySum(designId: string, qualityId: string): Promise<number>;
  readTypeBestRepresentation(typeId: string, tagIds: readonly string[]): Promise<RepresentationPlain | null>;
  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]>;
  readDesignReplaceableCatalogTypes(designId: string, selection: readonly string[]): Promise<readonly string[]>;
  readDesignReplaceableCatalogDesigns(designId: string, selection: readonly string[]): Promise<readonly string[]>;
  readDesignIncludedDesignIds(designId: string): Promise<readonly string[]>;
  /** @emoji 🧭 Switch materialized read DTO / GraphQL root (matches {@link WasmKitStoreClient.setKitReadScope}; no-op in fallback). */
  setKitReadScope(scope: KitReadScope): void;
  dispose(): void;
};

// #endregion 🔌KitStoreClientTypes

// #region 🧰ReadHelpers

function firstDesignPieceResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as JsonObject | undefined;
  if (!r0) return undefined;
  const inner = r0.readDesignPieceCommands as { results?: readonly unknown[] } | undefined;
  const p0 = inner?.results?.[0] as JsonObject | undefined;
  if (!p0) return undefined;
  const block = p0[cmdKey] as JsonObject | undefined;
  return block;
}

function firstDesignResult(out: readonly unknown[], cmdKey: string): unknown {
  const row = out[0] as { readKitDesignCommands?: { results?: readonly unknown[] } };
  const r0 = row.readKitDesignCommands?.results?.[0] as JsonObject | undefined;
  if (!r0) return undefined;
  return r0[cmdKey];
}

// #endregion 🧰ReadHelpers

// #region 📦LiveKitRoot

/** @emoji 🧭 Graph-shaped reads routed through {@link KitStore.read} (no legacy JS kit graph). */
export class LiveKitRoot {
  constructor(
    private readonly ks: KitStore,
    private readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  piece(designId: string, pieceId: string): LivePiece {
    return new LivePiece(this.ks, this.readScope, designId, pieceId);
  }

  design(designId: string): LiveDesign {
    return new LiveDesign(this.ks, this.readScope, designId);
  }

  type(typeId: string): LiveType {
    return new LiveType(this.ks, this.readScope, typeId);
  }

  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]> {
    return this.ks.read(this.readScope, [{ readKitColoredConnectorsCommand: null }]).then((out) => {
      const row = out[0];
      if (row && "readKitColoredConnectorsCommand" in row) return row.readKitColoredConnectorsCommand.rows;
      return [];
    });
  }
}

class LivePiece {
  constructor(
    private readonly ks: KitStore,
    private readonly readScope: KitReadScope,
    private readonly designId: string,
    private readonly pieceId: string,
  ) {}

  private run(cmd: ReadPieceCommand): Promise<ReadBatchResult> {
    const batch: ReadBatch = [
      {
        readKitDesignCommands: {
          id: { id: this.designId },
          commands: [{ readDesignPieceCommands: { id: { id: this.pieceId }, commands: [cmd] } }],
        },
      },
    ];
    return this.ks.read(this.readScope, batch);
  }

  readFlatPlane(): Promise<PlanePlain | null> {
    return this.run({ readPieceFlatPlaneCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceFlatPlaneCommand") as { flatPlane?: PlanePlain | null } | undefined;
      return blk?.flatPlane ?? null;
    });
  }

  readFlatCenter(): Promise<CoordinatePlain | null> {
    return this.run({ readPieceFlatCenterCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceFlatCenterCommand") as { flatCenter?: CoordinatePlain | null } | undefined;
      return blk?.flatCenter ?? null;
    });
  }

  readParentConnectionFull(): Promise<ConnectionPlain | null> {
    return this.run({ readPieceParentConnectionFullCommand: null }).then((out) => {
      const blk = firstDesignPieceResult(out, "readPieceParentConnectionFullCommand") as { connection?: ConnectionPlain | null } | undefined;
      return blk?.connection ?? null;
    });
  }
}

class LiveDesign {
  constructor(
    private readonly ks: KitStore,
    private readonly readScope: KitReadScope,
    private readonly designId: string,
  ) {}

  private run(cmd: ReadDesignCommand): Promise<ReadBatchResult> {
    return this.ks.read(this.readScope, [{ readKitDesignCommands: { id: { id: this.designId }, commands: [cmd] } }]);
  }

  readIncludedDesigns(): Promise<readonly IncludedDesignInfoDto[]> {
    return this.run({ readDesignIncludedDesignsCommand: null }).then((out) => {
      const blk = firstDesignResult(out, "readDesignIncludedDesignsCommand") as { designs?: readonly IncludedDesignInfoDto[] } | undefined;
      return blk?.designs ?? [];
    });
  }

  readClusterableGroups(selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    const cmd: ReadDesignCommand = {
      readDesignClusterableGroupsCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => {
      const blk = firstDesignResult(out, "readDesignClusterableGroupsCommand") as { groups?: readonly (readonly KitIdDto[])[] } | undefined;
      return blk?.groups ?? [];
    });
  }

  readQualitySum(qualityId: string): Promise<number> {
    const cmd: ReadDesignCommand = { readDesignQualitySumCommand: { qualityId: { id: qualityId } } };
    return this.run(cmd).then((out) => {
      const s = (firstDesignResult(out, "readDesignQualitySumCommand") as { sum?: number } | undefined)?.sum;
      return typeof s === "number" && !Number.isNaN(s) ? s : 0;
    });
  }

  readReplaceableCatalog(selection: readonly string[]): Promise<{ types: string[]; designs: string[] }> {
    const cmd: ReadDesignCommand = {
      readDesignReplaceableCatalogCommand: { selection: selection.map((id) => ({ id })) },
    };
    return this.run(cmd).then((out) => {
      const blk = firstDesignResult(out, "readDesignReplaceableCatalogCommand") as
        | { types?: readonly unknown[]; designs?: readonly unknown[] }
        | undefined;
      const toIds = (xs: readonly unknown[] | undefined) =>
        (xs ?? []).map((x) => (typeof x === "string" ? x : (x as { id?: string })?.id)).filter((x): x is string => typeof x === "string");
      return { types: toIds(blk?.types), designs: toIds(blk?.designs) };
    });
  }

  readIncludedDesignIds(): Promise<string[]> {
    return this.run({ readDesignIncludedDesignIdsCommand: null }).then((out) => {
      const ids = (firstDesignResult(out, "readDesignIncludedDesignIdsCommand") as { designIds?: readonly unknown[] } | undefined)?.designIds;
      return (ids ?? []).map((x) => (typeof x === "string" ? x : (x as { id?: string }).id)).filter((x): x is string => typeof x === "string");
    });
  }
}

class LiveType {
  constructor(
    private readonly ks: KitStore,
    private readonly readScope: KitReadScope,
    private readonly typeId: string,
  ) {}

  readBestRepresentation(tagIds: readonly string[]): Promise<RepresentationPlain | null> {
    return this.ks
      .read(this.readScope, [
        {
          readKitTypeCommands: {
            id: { id: this.typeId },
            commands: [{ readTypeBestRepresentationCommand: { tagIds: [...tagIds] } }],
          },
        },
      ])
      .then((out) => {
        const row = out[0];
        if (row && "readKitTypeCommands" in row) {
          const r0 = row.readKitTypeCommands.results[0];
          if (r0 && "readTypeBestRepresentationCommand" in r0) return r0.readTypeBestRepresentationCommand.representation;
        }
        return null;
      });
  }
}

// #endregion 📦LiveKitRoot

// #region 🪜LiveReadHub
/** @emoji 🧾 Live-read external stores (`useSyncExternalStore`) are implemented in `@semio/react` so `semio/js` holds no derived read caches. */
// #endregion 🪜LiveReadHub

// #region 🧰EventFilters

/** @emoji 🧭 Any kit graph mutation may flip undo/redo eligibility. */
export function kitEventAffectsCanUndoRedo(ev: KitEvent): boolean {
  void ev;
  return true;
}

/** @emoji 🧭 Live piece reads invalidate when the piece or its design changes. */
export function kitEventAffectsPieceLiveRead(ev: KitEvent, designId?: string, pieceId?: string): boolean {
  if (!designId || !pieceId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesPiece(ev as KitEvent, designId, pieceId);
}

/** @emoji 🧭 Replaceable catalog reads are design-scoped. */
export function kitEventAffectsReplaceableCatalogRead(ev: KitEvent, designId?: string, _selection?: ReadonlySet<string>): boolean {
  void _selection;
  if (!designId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesDesign(ev as KitEvent, designId);
}

/** @emoji 🧭 Design quality sum reads follow design-scoped invalidation. */
export function kitEventAffectsDesignQualitySumRead(ev: KitEvent, designId?: string, _qualityId?: string): boolean {
  void _qualityId;
  if (!designId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesDesign(ev as KitEvent, designId);
}

/** @emoji 🧭 Type-scoped reads follow kind-scoped invalidation. */
export function kitEventAffectsTypeScopedRead(ev: KitEvent, typeId?: string): boolean {
  if (!typeId) return true;
  if (ev == null || typeof ev !== "object") return true;
  return kitEventTouchesType(ev as KitEvent, typeId);
}

/** @emoji 🧭 Colored connector rows are kit-wide; invalidate on broad graph changes. */
export function kitEventAffectsKitColoredConnectorsRead(ev: KitEvent): boolean {
  if (ev == null || typeof ev !== "object") return true;
  const e = ev as KitEvent;
  if ("Changed" in e && (e as { Changed?: null }).Changed === null) return true;
  if ("ValidationInvalidated" in e && (e as { ValidationInvalidated?: null }).ValidationInvalidated === null) return true;
  return true;
}

// #endregion 🧰EventFilters

// #region 📦WasmKitStoreClient

export class WasmKitStoreClient implements KitStoreClient {
  private readonly listeners = new Set<(ev: KitEvent) => void>();
  private readonly offKit: () => void;
  /** @emoji 🧭 Active read scope for {@link getPieces} and {@link fetchFullKit}. */
  kitReadScope: KitReadScope = theKitReadScope;

  constructor(
    private readonly ks: KitStore,
    readScope: KitReadScope = theKitReadScope,
  ) {
    this.kitReadScope = readScope;
    this.offKit = this.ks.subscribe((ev: KitEvent) => {
      for (const l of this.listeners) l(ev);
    });
  }

  setKitReadScope(scope: KitReadScope): void {
    this.kitReadScope = scope;
    const ev = { Changed: null } as KitEvent;
    for (const l of this.listeners) l(ev);
  }

  /** @internal For read-store adapters. */
  internalKs(): KitStore {
    return this.ks;
  }

  /** @emoji 🧾 Authoritative full kit from `semio/rs` via GraphQL (no local DTO cache). */
  fetchFullKit(): Promise<KitFullDto> {
    return this.ks.materializedLiveJsonForReadScope(this.kitReadScope);
  }

  subscribe(cb: (ev: KitEvent) => void): () => void {
    this.listeners.add(cb);
    return () => {
      this.listeners.delete(cb);
    };
  }

  dispose(): void {
    this.offKit();
    this.listeners.clear();
    void this.ks.dispose();
  }

  kitGraphql(): LiveKitRoot {
    return new LiveKitRoot(this.ks, this.kitReadScope);
  }

  getKitWriteScope(): KitWriteScope | null {
    return this.ks.getKitWriteScope();
  }

  setKitWriteScope(scope: KitWriteScope | null): void {
    this.ks.setKitWriteScope(scope);
  }

  finalizeKitWriteTransaction(): Promise<SetResult> {
    return this.ks.finalizeKitWriteTransaction();
  }

  abortKitWriteTransaction(): Promise<SetResult> {
    return this.ks.abortKitWriteTransaction();
  }

  readPieceFlatPlane(designId: string, pieceId: string): Promise<PlanePlain | null> {
    return this.ks.piece(designId, pieceId, this.kitReadScope).readFlatPlane();
  }

  readPieceFlatCenter(designId: string, pieceId: string): Promise<CoordinatePlain | null> {
    return this.ks.piece(designId, pieceId, this.kitReadScope).readFlatCenter();
  }

  readPieceParentConnectionFull(designId: string, pieceId: string): Promise<ConnectionPlain | null> {
    return this.ks.piece(designId, pieceId, this.kitReadScope).readParentConnectionFull();
  }

  readDesignIncludedDesigns(designId: string): Promise<readonly IncludedDesignInfoDto[]> {
    return this.ks.design(designId, this.kitReadScope).readIncludedDesigns();
  }

  readDesignClusterableGroups(designId: string, selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    return this.ks.design(designId, this.kitReadScope).readClusterableGroups(selection);
  }

  readDesignQualitySum(designId: string, qualityId: string): Promise<number> {
    return this.ks.design(designId, this.kitReadScope).readQualitySum(qualityId);
  }

  readTypeBestRepresentation(typeId: string, tagIds: readonly string[]): Promise<RepresentationPlain | null> {
    return this.ks.type(typeId, this.kitReadScope).readBestRepresentation(tagIds);
  }

  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]> {
    return new LiveKitRoot(this.ks, this.kitReadScope).readColoredConnectors();
  }

  readDesignReplaceableCatalogTypes(designId: string, selection: readonly string[]): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadScope).readReplaceableCatalogTypes(selection);
  }

  readDesignReplaceableCatalogDesigns(designId: string, selection: readonly string[]): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadScope).readReplaceableCatalogDesigns(selection);
  }

  readDesignIncludedDesignIds(designId: string): Promise<readonly string[]> {
    return this.ks.design(designId, this.kitReadScope).readIncludedDesignIds();
  }

  submitChangeKitCommands(commands: readonly ChangeKitCommand[]): Promise<SetResult> {
    return this.ks.submitChangeKitCommands(commands);
  }

  clusterPieces(designId: string, pieceIds: readonly string[], clusterName: string): Promise<SetResult> {
    return this.ks.clusterPieces(designId, pieceIds, clusterName);
  }

  dragPieces(designId: string, pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.ks.dragPieces(designId, pieceIds, du, dv);
  }

  movePieces(designId: string, pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.ks.movePieces(designId, pieceIds, gap, shift, rise);
  }

  fixPieces(designId: string, pieceIds: readonly string[]): Promise<SetResult> {
    return this.ks.fixPieces(designId, pieceIds);
  }

  flattenDesign(designId: string): Promise<SetResult> {
    return this.ks.flattenDesign(designId);
  }

  expandDesign(parentDesignId: string, nestedDesignId: string): Promise<SetResult> {
    return this.ks.expandDesign(parentDesignId, nestedDesignId);
  }

  deleteConnection(designId: string, connectionId: string): Promise<SetResult> {
    return this.ks.deleteConnection(designId, connectionId);
  }

  changePieceType(designId: string, pieceId: string, newTypeId: string): Promise<SetResult> {
    return this.ks.changePieceType(designId, pieceId, newTypeId);
  }

  pasteDesignSelection(designId: string, selection: KitJsonTreeDto, plane: PlanePlain | null): Promise<SetResult> {
    return this.ks.pasteDesignSelection(designId, selection, plane);
  }

  createHangingPieces(designId: string, typeIds: readonly string[], plane: PlanePlain): Promise<SetResult> {
    return this.ks.createHangingPieces(designId, typeIds, plane);
  }

  createConnectedPiece(
    designId: string,
    parentPiece: string,
    parentPort: string,
    childType: string,
    childPort: string,
  ): Promise<SetResult> {
    return this.ks.createConnectedPiece(designId, parentPiece, parentPort, childType, childPort);
  }

  createFixedPiece(designId: string, typeId: string, plane: PlanePlain): Promise<SetResult> {
    return this.ks.createFixedPiece(designId, typeId, plane);
  }

  undo(): Promise<SetResult> {
    return this.ks.undo();
  }

  redo(): Promise<SetResult> {
    return this.ks.redo();
  }

  canUndo(): Promise<boolean> {
    return this.ks.canUndo();
  }

  canRedo(): Promise<boolean> {
    return this.ks.canRedo();
  }

  getPiecesMetadata(designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    return this.ks.getPiecesMetadata(this.kitReadScope, designId);
  }

  getPieces(designId: string): Promise<readonly PiecePlain[]> {
    return this.ks.getPieces(this.kitReadScope, designId);
  }

  getConnections(designId: string): Promise<readonly ConnectionPlain[]> {
    return this.ks.getConnections(this.kitReadScope, designId);
  }

  getDesigns(): Promise<readonly DesignShallow[]> {
    return this.ks.getDesigns(this.kitReadScope);
  }

  getTypes(): Promise<readonly TypeShallow[]> {
    return this.ks.getTypes(this.kitReadScope);
  }

  getAuthors(): Promise<readonly AuthorMetadataDto[]> {
    return this.ks.getAuthors(this.kitReadScope);
  }

  getKitMetadata(): Promise<KitMetadataDto | null> {
    return this.ks.getKitMetadata(this.kitReadScope);
  }

  backboneStatus(): Promise<BackboneStatusDto> {
    return this.ks.backboneStatus();
  }

  attachBackbone(cfg: BackboneConfig): Promise<SetResult> {
    return this.ks.attachBackbone(cfg);
  }

  detachBackbone(): Promise<SetResult> {
    return this.ks.detachBackbone();
  }

  listConflicts(): Promise<KitConflict[]> {
    return this.ks.listConflicts();
  }

  resolveConflict(id: string, strategy: ConflictResolution): Promise<SetResult> {
    return this.ks.resolveConflict(id, strategy);
  }

  syncNow(): Promise<SetResult> {
    return this.ks.syncNow();
  }
}

/** @emoji 🧾 Resolves the live {@link KitStore} behind a WASM {@link KitStoreClient}, or null for fallback clients. */
export function kitStoreFromKitStoreClient(client: KitStoreClient): KitStore | null {
  if (client instanceof WasmKitStoreClient) return client.internalKs();
  const probe = client as { internalKs?: () => KitStore };
  return probe.internalKs?.() ?? null;
}

class FallbackKitClient implements KitStoreClient {
  private readonly listeners = new Set<(ev: KitEvent) => void>();
  /** @emoji 🧭 Read scope label for host hooks (offline client has no live {@link KitStore}). */
  readonly kitReadScope: KitReadScope;
  constructor(
    private readonly kit: KitFullDto,
    readScope: KitReadScope = theKitReadScope,
  ) {
    this.kitReadScope = readScope;
  }

  fetchFullKit(): Promise<KitFullDto> {
    return Promise.resolve(this.kit);
  }

  subscribe(cb: (ev: KitEvent) => void): () => void {
    this.listeners.add(cb);
    return () => {
      this.listeners.delete(cb);
    };
  }

  dispose(): void {
    this.listeners.clear();
  }

  kitGraphql(): LiveKitRoot {
    throw new Error("kitGraphql unavailable in fallback kit client");
  }

  private notify(): void {
    const ev = { Changed: null } as KitEvent;
    for (const l of this.listeners) l(ev);
  }

  readPieceFlatPlane(_designId: string, _pieceId: string): Promise<PlanePlain | null> {
    void _designId;
    void _pieceId;
    return Promise.resolve(null);
  }
  readPieceFlatCenter(_designId: string, _pieceId: string): Promise<CoordinatePlain | null> {
    void _designId;
    void _pieceId;
    return Promise.resolve(null);
  }
  readPieceParentConnectionFull(_designId: string, _pieceId: string): Promise<ConnectionPlain | null> {
    void _designId;
    void _pieceId;
    return Promise.resolve(null);
  }
  readDesignIncludedDesigns(_designId: string): Promise<readonly IncludedDesignInfoDto[]> {
    void _designId;
    return Promise.resolve([]);
  }
  readDesignClusterableGroups(_designId: string, _selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    void _designId;
    void _selection;
    return Promise.resolve([]);
  }
  readDesignQualitySum(_designId: string, _qualityId: string): Promise<number> {
    void _designId;
    void _qualityId;
    return Promise.resolve(0);
  }
  readTypeBestRepresentation(_typeId: string, _tagIds: readonly string[]): Promise<RepresentationPlain | null> {
    void _typeId;
    void _tagIds;
    return Promise.resolve(null);
  }
  readColoredConnectors(): Promise<readonly KitColoredConnectorRowDto[]> {
    return Promise.resolve([]);
  }
  readDesignReplaceableCatalogTypes(_designId: string, _selection: readonly string[]): Promise<readonly string[]> {
    void _designId;
    void _selection;
    return Promise.resolve([]);
  }
  readDesignReplaceableCatalogDesigns(_designId: string, _selection: readonly string[]): Promise<readonly string[]> {
    void _designId;
    void _selection;
    return Promise.resolve([]);
  }
  readDesignIncludedDesignIds(_designId: string): Promise<readonly string[]> {
    void _designId;
    return Promise.resolve([]);
  }

  async submitChangeKitCommands(_commands: readonly ChangeKitCommand[]): Promise<SetResult> {
    void _commands;
    this.notify();
    return { ok: true };
  }

  async clusterPieces(_designId: string, _pieceIds: readonly string[], _clusterName: string): Promise<SetResult> {
    void _designId;
    void _pieceIds;
    void _clusterName;
    this.notify();
    return { ok: true };
  }
  async dragPieces(_designId: string, _pieceIds: readonly string[], _du: number, _dv: number): Promise<SetResult> {
    void _designId;
    void _pieceIds;
    void _du;
    void _dv;
    this.notify();
    return { ok: true };
  }
  async movePieces(_designId: string, _pieceIds: readonly string[], _gap: number, _shift: number, _rise: number): Promise<SetResult> {
    void _designId;
    void _pieceIds;
    void _gap;
    void _shift;
    void _rise;
    this.notify();
    return { ok: true };
  }
  async fixPieces(_designId: string, _pieceIds: readonly string[]): Promise<SetResult> {
    void _designId;
    void _pieceIds;
    this.notify();
    return { ok: true };
  }
  async flattenDesign(_designId: string): Promise<SetResult> {
    void _designId;
    this.notify();
    return { ok: true };
  }
  async expandDesign(_parentDesignId: string, _nestedDesignId: string): Promise<SetResult> {
    void _parentDesignId;
    void _nestedDesignId;
    this.notify();
    return { ok: true };
  }
  async deleteConnection(_designId: string, _connectionId: string): Promise<SetResult> {
    void _designId;
    void _connectionId;
    this.notify();
    return { ok: true };
  }
  async changePieceType(_designId: string, _pieceId: string, _newTypeId: string): Promise<SetResult> {
    void _designId;
    void _pieceId;
    void _newTypeId;
    this.notify();
    return { ok: true };
  }
  async pasteDesignSelection(_designId: string, _selection: KitJsonTreeDto, _plane: PlanePlain | null): Promise<SetResult> {
    void _designId;
    void _selection;
    void _plane;
    this.notify();
    return { ok: true };
  }
  async createHangingPieces(_designId: string, _typeIds: readonly string[], _plane: PlanePlain): Promise<SetResult> {
    void _designId;
    void _typeIds;
    void _plane;
    this.notify();
    return { ok: true };
  }
  async createConnectedPiece(
    _designId: string,
    _parentPiece: string,
    _parentPort: string,
    _childType: string,
    _childPort: string,
  ): Promise<SetResult> {
    void _designId;
    void _parentPiece;
    void _parentPort;
    void _childType;
    void _childPort;
    this.notify();
    return { ok: true };
  }
  async createFixedPiece(_designId: string, _typeId: string, _plane: PlanePlain): Promise<SetResult> {
    void _designId;
    void _typeId;
    void _plane;
    this.notify();
    return { ok: true };
  }
  async undo(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async redo(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async canUndo(): Promise<boolean> {
    return false;
  }
  async canRedo(): Promise<boolean> {
    return false;
  }
  async getPiecesMetadata(_designId: string): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    void _designId;
    return new Map();
  }
  async getPieces(_designId: string): Promise<readonly PiecePlain[]> {
    void _designId;
    return [];
  }
  async getConnections(_designId: string): Promise<readonly ConnectionPlain[]> {
    void _designId;
    return [];
  }
  async getDesigns(): Promise<readonly DesignShallow[]> {
    return [];
  }
  async getTypes(): Promise<readonly TypeShallow[]> {
    return [];
  }
  async getAuthors(): Promise<readonly AuthorMetadataDto[]> {
    return [];
  }
  async getKitMetadata(): Promise<KitMetadataDto | null> {
    return null;
  }
  async backboneStatus(): Promise<BackboneStatusDto> {
    return { attached: false, kind: null, backboneTip: null, pendingWipCheckpoints: 0 };
  }
  async attachBackbone(_cfg: BackboneConfig): Promise<SetResult> {
    void _cfg;
    this.notify();
    return { ok: true };
  }
  async detachBackbone(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }
  async listConflicts(): Promise<KitConflict[]> {
    return [];
  }
  async resolveConflict(_id: string, _strategy: ConflictResolution): Promise<SetResult> {
    void _id;
    void _strategy;
    this.notify();
    return { ok: true };
  }
  async syncNow(): Promise<SetResult> {
    this.notify();
    return { ok: true };
  }

  setKitReadScope(_scope: KitReadScope): void {
    void _scope;
  }

  getKitWriteScope(): KitWriteScope | null {
    return null;
  }

  setKitWriteScope(_scope: KitWriteScope | null): void {
    void _scope;
  }

  async finalizeKitWriteTransaction(): Promise<SetResult> {
    return { ok: false, error: { kind: "NotSupported", message: "finalizeKitWriteTransaction: fallback client" } };
  }

  async abortKitWriteTransaction(): Promise<SetResult> {
    return { ok: false, error: { kind: "NotSupported", message: "abortKitWriteTransaction: fallback client" } };
  }
}

export async function createKitStoreClient(opts: { initialKit: KitFullDto; forceFallback?: boolean; readScope?: KitReadScope }): Promise<KitStoreClient> {
  if (opts.forceFallback) return new FallbackKitClient(opts.initialKit);
  const ks = await KitStore.open(opts.initialKit);
  const c = new WasmKitStoreClient(ks, opts.readScope);
  await c.fetchFullKit();
  return c;
}

const facades = new WeakMap<KitStoreClient, SemioKitCommandFacade>();

export function acquireSemioKitCommandFacade(client: KitStoreClient): SemioKitCommandFacade {
  let f = facades.get(client);
  if (!f) {
    f = {
      runMutation: async (cmd: KitTypedChangeKitCommandsBatch): Promise<SetResult> => {
        if (cmd.kind !== "changeKitCommands") return { ok: false, error: { kind: "NotSupported", message: "command" } };
        return client.submitChangeKitCommands(cmd.commands);
      },
    };
    facades.set(client, f);
  }
  return f;
}

export function releaseSemioKitCommandFacade(client: KitStoreClient): void {
  facades.delete(client);
}

// #endregion 📦WasmKitStoreClient

// #region 🧩KitEntitiesMerged
// #region Constants
// Global constants MUST define shared numeric parameters.

/** Standard icon width in pixels.
 **/
export const ICON_WIDTH = 50;
/**
 * Numeric tolerance for floating-point comparisons.
 **/
export const TOLERANCE = 1e-5;

// #endregion Constants

// #region Utilities
// Removed: toArray, SeededRandom, Generator, round, jaccard, deepEqual, arraysEqual — domain logic moved to semio/rs (Requirements 1.3, 3.5)

/**
 * Zod schema for DiffStatus validation.
 **/
export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

/**
 * Enumeration of DiffStatus values.
 **/
export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}

/**
 * Type alias for Id.
 **/
export type Id = string;

// #endregion Utilities

// #region Entity IDs
// Entity identifier types and comparison functions MUST be defined here.

export type AttributeIdDto = Readonly<{ readonly id: Id }>;
export type LocationIdDto = Readonly<{ readonly id: Id }>;
export type AuthorIdDto = Readonly<{ readonly id: Id }>;
export type FileIdDto = Readonly<{ readonly id: Id }>;
export type FolderIdDto = Readonly<{ readonly id: Id }>;
export type BenchmarkIdDto = Readonly<{ readonly id: Id }>;
export type QualityIdDto = Readonly<{ readonly id: Id }>;
export type PortIdDto = Readonly<{ readonly id: Id }>;
export type PropIdDto = Readonly<{ readonly id: Id }>;
export type RepresentationIdDto = Readonly<{ readonly id: Id }>;
export type ConnectorIdDto = Readonly<{ readonly id: Id }>;
export type TypeIdDto = Readonly<{ readonly id: Id }>;
export type LayerIdDto = Readonly<{ readonly id: Id }>;
export type PieceIdDto = Readonly<{ readonly id: Id }>;
export type GroupIdDto = Readonly<{ readonly id: Id }>;
export type ConnectionIdDto = Readonly<{ readonly id: Id }>;
export type StatIdDto = Readonly<{ readonly id: Id }>;
export type DesignIdDto = Readonly<{ readonly id: Id }>;
export type KitIdDto = Readonly<{ readonly id: Id }>;
export type TagIdDto = Readonly<{ readonly id: Id }>;
export type ConceptIdDto = Readonly<{ readonly id: Id }>;
export type FamilyIdDto = Readonly<{ readonly id: Id }>;

export const AttributeIdSchema = z.object({ id: z.string() });
export const LocationIdSchema = z.object({ id: z.string() });
export const AuthorIdSchema = z.object({ id: z.string() });
export const FileIdSchema = z.object({ id: z.string() });
export const FolderIdSchema = z.object({ id: z.string() });
export const BenchmarkIdSchema = z.object({ id: z.string() });
export const QualityIdSchema = z.object({ id: z.string() });
export const PortIdSchema = z.object({ id: z.string() });
export const PropIdSchema = z.object({ id: z.string() });
export const RepresentationIdSchema = z.object({ id: z.string() });
export const ConnectorIdSchema = z.object({ id: z.string() });
export const TypeIdSchema = z.object({ id: z.string() });
export const LayerIdSchema = z.object({ id: z.string() });
export const PieceIdSchema = z.object({ id: z.string() });
export const GroupIdSchema = z.object({ id: z.string() });
export const ConnectionIdSchema = z.object({ id: z.string() });
export const StatIdSchema = z.object({ id: z.string() });
export const DesignIdSchema = z.object({ id: z.string() });
export const KitIdSchema = z.object({ id: z.string() });
export const TagIdSchema = z.object({ id: z.string() });
export const ConceptIdSchema = z.object({ id: z.string() });
export const FamilyIdSchema = z.object({ id: z.string() });

// Removed: All free create*Id, areSame*Id, get*Id functions — use Entity.createId/areSameId static methods (Requirement 3.2)

// #endregion Entity IDs

// #region Weak Entities

// #region Coordinate
export const CoordinateSchema = z.object({ u: z.number(), v: z.number() });
export type CoordinatePlain = ReadonlyDto<z.infer<typeof CoordinateSchema>>;
export class Coordinate implements CoordinatePlain {
  u!: number;
  v!: number;
  constructor(plain: CoordinatePlain) {
    Object.assign(this, CoordinateSchema.parse(plain));
  }
  static from(plain: CoordinatePlain): Coordinate { return new Coordinate(plain); }
  toPlain(): CoordinatePlain { return CoordinateSchema.parse(this as CoordinatePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Coordinate { return new Coordinate(CoordinateSchema.parse(JSON.parse(json))); }
}
export const CoordinateDiffSchema = CoordinateSchema.partial();
export type CoordinateDiff = ReadonlyDto<z.infer<typeof CoordinateDiffSchema>>;
// #endregion Coordinate

// #region Vec
export const VecSchema = z.object({ u: z.number(), v: z.number() });
export type VecPlain = ReadonlyDto<z.infer<typeof VecSchema>>;
export class Vec implements VecPlain {
  u!: number;
  v!: number;
  constructor(plain: VecPlain) { Object.assign(this, VecSchema.parse(plain)); }
  static from(plain: VecPlain): Vec { return new Vec(plain); }
  static fromPlain(plain: VecPlain): Vec { return new Vec(plain); }
  toPlain(): VecPlain { return VecSchema.parse(this as VecPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vec { return new Vec(VecSchema.parse(JSON.parse(json))); }
}
export const VecDiffSchema = VecSchema.partial();
export type VecDiff = ReadonlyDto<z.infer<typeof VecDiffSchema>>;
// #endregion Vec

// #region Point
export const PointSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type PointPlain = ReadonlyDto<z.infer<typeof PointSchema>>;
export class Point implements PointPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: PointPlain) { Object.assign(this, PointSchema.parse(plain)); }
  static from(plain: PointPlain): Point { return new Point(plain); }
  static fromPlain(plain: PointPlain): Point { return new Point(plain); }
  toPlain(): PointPlain { return PointSchema.parse(this as PointPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Point { return new Point(PointSchema.parse(JSON.parse(json))); }
}
export const PointDiffSchema = PointSchema.partial();
export type PointDiff = ReadonlyDto<z.infer<typeof PointDiffSchema>>;
// #endregion Point

// #region Vector
export const VectorSchema = z.object({ x: z.number(), y: z.number(), z: z.number() });
export type VectorPlain = ReadonlyDto<z.infer<typeof VectorSchema>>;
export class Vector implements VectorPlain {
  x!: number;
  y!: number;
  z!: number;
  constructor(plain: VectorPlain) { Object.assign(this, VectorSchema.parse(plain)); }
  static from(plain: VectorPlain): Vector { return new Vector(plain); }
  static fromPlain(plain: VectorPlain): Vector { return new Vector(plain); }
  toPlain(): VectorPlain { return VectorSchema.parse(this as VectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Vector { return new Vector(VectorSchema.parse(JSON.parse(json))); }
}
export const VectorDiffSchema = VectorSchema.partial();
export type VectorDiff = ReadonlyDto<z.infer<typeof VectorDiffSchema>>;
// #endregion Vector

// #region Plane
export const PlaneSchema = z.object({ origin: PointSchema, xAxis: VectorSchema, yAxis: VectorSchema });
export type PlanePlain = ReadonlyDto<z.infer<typeof PlaneSchema>>;
export class Plane implements PlanePlain {
  origin!: Point;
  xAxis!: Vector;
  yAxis!: Vector;
  constructor(plain: PlanePlain) {
    const p = PlaneSchema.parse(plain);
    this.origin = new Point(p.origin);
    this.xAxis = new Vector(p.xAxis);
    this.yAxis = new Vector(p.yAxis);
  }
  static from(plain: PlanePlain): Plane { return new Plane(plain); }
  static fromPlain(plain: PlanePlain): Plane { return new Plane(plain); }
  toPlain(): PlanePlain { return PlaneSchema.parse(this as PlanePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Plane { return new Plane(PlaneSchema.parse(JSON.parse(json))); }
  // Removed: averageWith, average, rounded — geometry computation moved to semio/rs (Requirement 1.14)
}
export const PlaneDiffSchema = PlaneSchema.omit({ origin: true, xAxis: true, yAxis: true })
  .extend({ origin: PointDiffSchema, xAxis: VectorDiffSchema, yAxis: VectorDiffSchema }).partial();
export type PlaneDiff = ReadonlyDto<z.infer<typeof PlaneDiffSchema>>;
// #endregion Plane

// #region Camera
export const CameraSchema = z.object({ position: PointSchema, forward: VectorSchema, up: VectorSchema });
export type CameraPlain = ReadonlyDto<z.infer<typeof CameraSchema>>;
export class Camera implements CameraPlain {
  position!: Point;
  forward!: Vector;
  up!: Vector;
  constructor(plain: CameraPlain) {
    const p = CameraSchema.parse(plain);
    this.position = new Point(p.position);
    this.forward = new Vector(p.forward);
    this.up = new Vector(p.up);
  }
  static from(plain: CameraPlain): Camera { return new Camera(plain); }
  static fromPlain(plain: CameraPlain): Camera { return new Camera(plain); }
  toPlain(): CameraPlain { return CameraSchema.parse(this as CameraPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Camera { return new Camera(CameraSchema.parse(JSON.parse(json))); }
}
export const CameraDiffSchema = CameraSchema.omit({ position: true, forward: true, up: true })
  .extend({ position: PointDiffSchema, forward: VectorDiffSchema, up: VectorDiffSchema }).partial();
export type CameraDiff = ReadonlyDto<z.infer<typeof CameraDiffSchema>>;
// #endregion Camera

// #endregion Weak Entities

// #region Attribute
const DateProperty = () => z.string().optional();
export const AttributeSchema = z.object({ id: z.string(), key: z.string(), value: z.string().optional(), definition: z.string().optional() });
export type AttributePlain = ReadonlyDto<z.infer<typeof AttributeSchema>>;
export class Attribute implements AttributePlain {
  id!: string; key!: string; value?: string; definition?: string;
  constructor(plain: AttributePlain) { Object.assign(this, AttributeSchema.parse(plain)); }
  static from(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static fromPlain(plain: AttributePlain): Attribute { return new Attribute(plain); }
  static createId(id: string): AttributeIdDto { return { id }; }
  static areSameId(a: AttributeIdDto, b: AttributeIdDto): boolean { return a.id === b.id; }
  toPlain(): AttributePlain { return AttributeSchema.parse(this as AttributePlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Attribute { return new Attribute(AttributeSchema.parse(JSON.parse(json))); }
}
export const AttributeMetadataDtoSchema = AttributeSchema;
export type AttributeMetadataDto = ReadonlyDto<z.infer<typeof AttributeMetadataDtoSchema>>;
export const AttributeShallowSchema = AttributeSchema;
export type AttributeShallow = ReadonlyDto<z.infer<typeof AttributeShallowSchema>>;
export const AttributeDiffSchema = AttributeSchema.partial();
export type AttributeDiff = ReadonlyDto<z.infer<typeof AttributeDiffSchema>>;
export const AttributesDiffSchema = z.object({
  removed: z.array(AttributeIdSchema).optional(),
  updated: z.array(z.object({ attribute: AttributeIdSchema, diff: AttributeDiffSchema })).optional(),
  added: z.array(z.any()).optional(),
});
export type AttributesDiff = ReadonlyDto<z.infer<typeof AttributesDiffSchema>>;
// #endregion Attribute

// #region Location
export const LocationSchema = z.object({ id: z.string(), longitude: z.number().optional(), latitude: z.number().optional(), altitude: z.number().optional(), attributes: z.array(AttributeSchema).optional() });
export type LocationPlain = ReadonlyDto<z.infer<typeof LocationSchema>>;
export class Location implements LocationPlain {
  id!: string; longitude?: number; latitude?: number; altitude?: number; attributes?: Attribute[];
  constructor(plain: LocationPlain) { const p = LocationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LocationPlain): Location { return new Location(plain); }
  static fromPlain(plain: LocationPlain): Location { return new Location(plain); }
  static createId(id: string): LocationIdDto { return { id }; }
  static areSameId(a: LocationIdDto, b: LocationIdDto): boolean { return a.id === b.id; }
  toPlain(): LocationPlain { return LocationSchema.parse(this as LocationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Location { return new Location(LocationSchema.parse(JSON.parse(json))); }
}
export const LocationMetadataDtoSchema = LocationSchema;
export type LocationMetadataDto = ReadonlyDto<z.infer<typeof LocationMetadataDtoSchema>>;
export const LocationShallowSchema = LocationSchema;
export type LocationShallow = ReadonlyDto<z.infer<typeof LocationShallowSchema>>;
export const LocationDiffSchema = LocationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LocationDiff = ReadonlyDto<z.infer<typeof LocationDiffSchema>>;
// #endregion Location

// #region Author
export const AuthorSchema = z.object({ id: z.string(), name: z.string(), email: z.string().optional(), role: z.string().optional(), rank: z.number().optional() });
export type AuthorPlain = ReadonlyDto<z.infer<typeof AuthorSchema>>;
export class Author implements AuthorPlain {
  id!: string; name!: string; email?: string; role?: string; rank?: number;
  constructor(plain: AuthorPlain) { Object.assign(this, AuthorSchema.parse(plain)); }
  static from(plain: AuthorPlain): Author { return new Author(plain); }
  static fromPlain(plain: AuthorPlain): Author { return new Author(plain); }
  static createId(id: string): AuthorIdDto { return { id }; }
  static areSameId(a: AuthorIdDto, b: AuthorIdDto): boolean { return a.id === b.id; }
  toPlain(): AuthorPlain { return AuthorSchema.parse(this as AuthorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Author { return new Author(AuthorSchema.parse(JSON.parse(json))); }
}
export const AuthorMetadataDtoSchema = AuthorSchema;
export type AuthorMetadataDto = ReadonlyDto<z.infer<typeof AuthorMetadataDtoSchema>>;
export const AuthorShallowSchema = AuthorSchema;
export type AuthorShallow = ReadonlyDto<z.infer<typeof AuthorShallowSchema>>;
export const AuthorDiffSchema = AuthorSchema.partial();
export type AuthorDiff = ReadonlyDto<z.infer<typeof AuthorDiffSchema>>;
export const AuthorsDiffSchema = z.object({ removed: z.array(AuthorIdSchema).optional(), updated: z.array(z.object({ author: AuthorIdSchema, diff: AuthorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type AuthorsDiff = ReadonlyDto<z.infer<typeof AuthorsDiffSchema>>;
// #endregion Author

// #region File
export const FileSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  folder: FolderIdSchema.optional(),
  url: z.string().optional(),
  remote: z.string().optional(),
  mime: z.string().optional(),
  size: z.number().optional(),
  hash: z.string().optional(),
  description: z.string().optional(),
  blob: z.union([z.string(), z.custom<Blob>((v) => typeof Blob !== "undefined" && v instanceof Blob)]).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type FilePlain = ReadonlyDto<z.infer<typeof FileSchema>>;
export class File implements FilePlain {
  id!: string;
  name?: string;
  folder?: { id: string };
  url?: string;
  remote?: string;
  mime?: string;
  size?: number;
  hash?: string;
  description?: string;
  blob?: string | Blob;
  createdAt?: string;
  updatedAt?: string;
  constructor(plain: FilePlain) { Object.assign(this, FileSchema.parse(plain)); }
  static from(plain: FilePlain): File { return new File(plain); }
  static fromPlain(plain: FilePlain): File { return new File(plain); }
  static createId(id: string): FileIdDto { return { id }; }
  static areSameId(a: FileIdDto, b: FileIdDto): boolean { return a.id === b.id; }
  toPlain(): FilePlain { return FileSchema.parse(this as FilePlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): File { return new File(FileSchema.parse(JSON.parse(json))); }
}
export const FileMetadataDtoSchema = FileSchema;
export type FileMetadataDto = ReadonlyDto<z.infer<typeof FileMetadataDtoSchema>>;
export const FileShallowSchema = FileSchema;
export type FileShallow = ReadonlyDto<z.infer<typeof FileShallowSchema>>;
export const FileDiffSchema = FileSchema.partial();
export type FileDiff = ReadonlyDto<z.infer<typeof FileDiffSchema>>;
export const FilesDiffSchema = z.object({ removed: z.array(FileIdSchema).optional(), updated: z.array(z.object({ file: FileIdSchema, diff: FileDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FilesDiff = ReadonlyDto<z.infer<typeof FilesDiffSchema>>;
// #endregion File

// #region Folder
export const FolderSchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  parent: z.object({ id: z.string() }).optional(),
  path: z.string().optional(),
  description: z.string().optional(),
});
export type FolderPlain = ReadonlyDto<z.infer<typeof FolderSchema>>;
export class Folder implements FolderPlain {
  id!: string;
  name?: string;
  parent?: { id: string };
  path?: string;
  description?: string;
  constructor(plain: FolderPlain) { Object.assign(this, FolderSchema.parse(plain)); }
  static from(plain: FolderPlain): Folder { return new Folder(plain); }
  static fromPlain(plain: FolderPlain): Folder { return new Folder(plain); }
  static createId(id: string): FolderIdDto { return { id }; }
  static areSameId(a: FolderIdDto, b: FolderIdDto): boolean { return a.id === b.id; }
  toPlain(): FolderPlain { return FolderSchema.parse(this as FolderPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Folder { return new Folder(FolderSchema.parse(JSON.parse(json))); }
}
export const FolderMetadataDtoSchema = FolderSchema;
export type FolderMetadataDto = ReadonlyDto<z.infer<typeof FolderMetadataDtoSchema>>;
export const FolderShallowSchema = FolderSchema;
export type FolderShallow = ReadonlyDto<z.infer<typeof FolderShallowSchema>>;
export const FolderDiffSchema = FolderSchema.partial();
export type FolderDiff = ReadonlyDto<z.infer<typeof FolderDiffSchema>>;
export const FoldersDiffSchema = z.object({ removed: z.array(FolderIdSchema).optional(), updated: z.array(z.object({ folder: FolderIdSchema, diff: FolderDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FoldersDiff = ReadonlyDto<z.infer<typeof FoldersDiffSchema>>;
// #endregion Folder

// #region Benchmark
export const BenchmarkSchema = z.object({ id: z.string(), name: z.string(), min: z.number().optional(), max: z.number().optional(), minExcluded: z.boolean().optional(), maxExcluded: z.boolean().optional() });
export type BenchmarkPlain = ReadonlyDto<z.infer<typeof BenchmarkSchema>>;
export class Benchmark implements BenchmarkPlain {
  id!: string; name!: string; min?: number; max?: number; minExcluded?: boolean; maxExcluded?: boolean;
  constructor(plain: BenchmarkPlain) { Object.assign(this, BenchmarkSchema.parse(plain)); }
  static from(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static fromPlain(plain: BenchmarkPlain): Benchmark { return new Benchmark(plain); }
  static createId(id: string): BenchmarkIdDto { return { id }; }
  static areSameId(a: BenchmarkIdDto, b: BenchmarkIdDto): boolean { return a.id === b.id; }
  toPlain(): BenchmarkPlain { return BenchmarkSchema.parse(this as BenchmarkPlain); }
  toJson(): string { return JSON.stringify(this.toPlain()); }
  static fromJson(json: string): Benchmark { return new Benchmark(BenchmarkSchema.parse(JSON.parse(json))); }
}
export const BenchmarkMetadataDtoSchema = BenchmarkSchema;
export type BenchmarkMetadataDto = ReadonlyDto<z.infer<typeof BenchmarkMetadataDtoSchema>>;
export const BenchmarkShallowSchema = BenchmarkSchema;
export type BenchmarkShallow = ReadonlyDto<z.infer<typeof BenchmarkShallowSchema>>;
export const BenchmarkDiffSchema = BenchmarkSchema.partial();
export type BenchmarkDiff = ReadonlyDto<z.infer<typeof BenchmarkDiffSchema>>;
export const BenchmarksDiffSchema = z.object({ removed: z.array(BenchmarkIdSchema).optional(), updated: z.array(z.object({ benchmark: BenchmarkIdSchema, diff: BenchmarkDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type BenchmarksDiff = ReadonlyDto<z.infer<typeof BenchmarksDiffSchema>>;
// #endregion Benchmark

// #region Quality
export const QualitySchema = z.object({
  id: z.string(),
  name: z.string().optional(),
  key: z.string(),
  folder: z.string().optional(),
  value: z.string().optional(),
  unit: z.string().optional(),
  definition: z.string().optional(),
  description: z.string().optional(),
  benchmarks: z.array(BenchmarkSchema).optional(),
});
export type QualityPlain = ReadonlyDto<z.infer<typeof QualitySchema>>;
export class Quality implements QualityPlain {
  id!: string;
  name?: string;
  key!: string;
  folder?: string;
  value?: string;
  unit?: string;
  definition?: string;
  description?: string;
  benchmarks?: Benchmark[];
  constructor(plain: QualityPlain) { const p = QualitySchema.parse(plain); Object.assign(this, p); this.benchmarks = p.benchmarks?.map((b) => new Benchmark(b)); }
  static from(plain: QualityPlain): Quality { return new Quality(plain); }
  static fromPlain(plain: QualityPlain): Quality { return new Quality(plain); }
  static createId(id: string): QualityIdDto { return { id }; }
  static areSameId(a: QualityIdDto, b: QualityIdDto): boolean { return a.id === b.id; }
  toPlain(): QualityPlain { return QualitySchema.parse(this as QualityPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Quality { return new Quality(QualitySchema.parse(JSON.parse(json))); }
}
export const QualityMetadataDtoSchema = QualitySchema.omit({ benchmarks: true });
export type QualityMetadataDto = ReadonlyDto<z.infer<typeof QualityMetadataDtoSchema>>;
export const QualityShallowSchema = QualitySchema;
export type QualityShallow = ReadonlyDto<z.infer<typeof QualityShallowSchema>>;
export const QualityDiffSchema = QualitySchema.partial().omit({ benchmarks: true }).extend({ benchmarks: BenchmarksDiffSchema.optional() });
export type QualityDiff = ReadonlyDto<z.infer<typeof QualityDiffSchema>>;
export const QualitiesDiffSchema = z.object({ removed: z.array(QualityIdSchema).optional(), updated: z.array(z.object({ quality: QualityIdSchema, diff: QualityDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type QualitiesDiff = ReadonlyDto<z.infer<typeof QualitiesDiffSchema>>;
// #endregion Quality

// #region Port
export const PortSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  icon: z.string().optional(),
  compatibleFamilies: z.array(FamilyIdSchema).optional(),
  mandatory: z.boolean().optional(),
  t: z.number().optional(),
  point: PointSchema.optional(),
  direction: VectorSchema.optional(),
  compatiblePorts: z.array(PortIdSchema).optional(),
  qualities: z.array(QualitySchema).optional(),
  attributes: z.array(AttributeSchema).optional(),
  maxChildren: z.number().optional(),
});
export type PortPlain = ReadonlyDto<z.infer<typeof PortSchema>>;
export class Port implements PortPlain {
  id!: string;
  name!: string;
  description?: string;
  icon?: string;
  compatibleFamilies?: FamilyIdDto[];
  mandatory?: boolean;
  t?: number;
  point?: Point;
  direction?: Vector;
  compatiblePorts?: PortIdDto[];
  qualities?: Quality[];
  attributes?: Attribute[];
  maxChildren?: number;
  constructor(plain: PortPlain) { const p = PortSchema.parse(plain); Object.assign(this, p); this.point = p.point ? new Point(p.point) : undefined; this.direction = p.direction ? new Vector(p.direction) : undefined; this.qualities = p.qualities?.map((q) => new Quality(q)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: PortPlain): Port { return new Port(plain); }
  static fromPlain(plain: PortPlain): Port { return new Port(plain); }
  static createId(id: string): PortIdDto { return { id }; }
  static areSameId(a: PortIdDto, b: PortIdDto): boolean { return a.id === b.id; }
  toPlain(): PortPlain { return PortSchema.parse(this as PortPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Port { return new Port(PortSchema.parse(JSON.parse(json))); }
}
export const PortMetadataDtoSchema = PortSchema.omit({ qualities: true, attributes: true });
export type PortMetadataDto = ReadonlyDto<z.infer<typeof PortMetadataDtoSchema>>;
export const PortShallowSchema = PortSchema;
export type PortShallow = ReadonlyDto<z.infer<typeof PortShallowSchema>>;
export const PortDiffSchema = PortSchema.partial().omit({ qualities: true, attributes: true }).extend({ qualities: QualitiesDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PortDiff = ReadonlyDto<z.infer<typeof PortDiffSchema>>;
export const PortsDiffSchema = z.object({ removed: z.array(PortIdSchema).optional(), updated: z.array(z.object({ port: PortIdSchema, diff: PortDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PortsDiff = ReadonlyDto<z.infer<typeof PortsDiffSchema>>;
// #endregion Port

// #region Family
export const FamilySchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), icon: z.string().optional(), ports: z.array(PortSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type FamilyPlain = ReadonlyDto<z.infer<typeof FamilySchema>>;
export class Family implements FamilyPlain {
  id!: string; name!: string; description?: string; icon?: string; ports?: Port[]; attributes?: Attribute[];
  constructor(plain: FamilyPlain) { const p = FamilySchema.parse(plain); Object.assign(this, p); this.ports = p.ports?.map((x) => new Port(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: FamilyPlain): Family { return new Family(plain); }
  static fromPlain(plain: FamilyPlain): Family { return new Family(plain); }
  static createId(id: string): FamilyIdDto { return { id }; }
  static areSameId(a: FamilyIdDto, b: FamilyIdDto): boolean { return a.id === b.id; }
  toPlain(): FamilyPlain { return FamilySchema.parse(this as FamilyPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Family { return new Family(FamilySchema.parse(JSON.parse(json))); }
}
export const FamilyMetadataDtoSchema = FamilySchema.omit({ ports: true, attributes: true });
export type FamilyMetadataDto = ReadonlyDto<z.infer<typeof FamilyMetadataDtoSchema>>;
export const FamilyShallowSchema = FamilySchema;
export type FamilyShallow = ReadonlyDto<z.infer<typeof FamilyShallowSchema>>;
export const FamilyDiffSchema = FamilySchema.partial().omit({ ports: true, attributes: true }).extend({ ports: PortsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type FamilyDiff = ReadonlyDto<z.infer<typeof FamilyDiffSchema>>;
export const FamiliesDiffSchema = z.object({ removed: z.array(FamilyIdSchema).optional(), updated: z.array(z.object({ family: FamilyIdSchema, diff: FamilyDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type FamiliesDiff = ReadonlyDto<z.infer<typeof FamiliesDiffSchema>>;
// #endregion Family

// #region Prop
export const PropSchema = z.object({
  id: z.coerce.string(),
  key: z.coerce.string(),
  value: z.string().optional(),
  unit: z.string().optional(),
  quality: QualityIdSchema.optional(),
});
export type PropPlain = ReadonlyDto<z.infer<typeof PropSchema>>;
export class Prop implements PropPlain {
  id!: string; key!: string; value?: string; unit?: string; quality?: QualityIdDto;
  constructor(plain: PropPlain) { Object.assign(this, PropSchema.parse(plain)); }
  static from(plain: PropPlain): Prop { return new Prop(plain); }
  static fromPlain(plain: PropPlain): Prop { return new Prop(plain); }
  static createId(id: string): PropIdDto { return { id }; }
  static areSameId(a: PropIdDto, b: PropIdDto): boolean { return a.id === b.id; }
  toPlain(): PropPlain { return PropSchema.parse(this as PropPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Prop { return new Prop(PropSchema.parse(JSON.parse(json))); }
}
export const PropMetadataDtoSchema = PropSchema;
export type PropMetadataDto = ReadonlyDto<z.infer<typeof PropMetadataDtoSchema>>;
export const PropShallowSchema = PropSchema;
export type PropShallow = ReadonlyDto<z.infer<typeof PropShallowSchema>>;
export const PropDiffSchema = PropSchema.partial();
export type PropDiff = ReadonlyDto<z.infer<typeof PropDiffSchema>>;
export const PropsDiffSchema = z.object({ removed: z.array(PropIdSchema).optional(), updated: z.array(z.object({ prop: PropIdSchema, diff: PropDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PropsDiff = ReadonlyDto<z.infer<typeof PropsDiffSchema>>;
// #endregion Prop

// #region Tag
export const TagSchema = z.object({ id: z.string(), name: z.string(), order: z.number().optional() });
export type TagPlain = ReadonlyDto<z.infer<typeof TagSchema>>;
export class Tag implements TagPlain {
  id!: string; name!: string; order?: number;
  constructor(plain: TagPlain) { Object.assign(this, TagSchema.parse(plain)); }
  static from(plain: TagPlain): Tag { return new Tag(plain); }
  static fromPlain(plain: TagPlain): Tag { return new Tag(plain); }
  static createId(id: string): TagIdDto { return { id }; }
  static areSameId(a: TagIdDto, b: TagIdDto): boolean { return a.id === b.id; }
  toPlain(): TagPlain { return TagSchema.parse(this as TagPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Tag { return new Tag(TagSchema.parse(JSON.parse(json))); }
}
export const TagMetadataDtoSchema = TagSchema;
export type TagMetadataDto = ReadonlyDto<z.infer<typeof TagMetadataDtoSchema>>;
export const TagShallowSchema = TagSchema;
export type TagShallow = ReadonlyDto<z.infer<typeof TagShallowSchema>>;
export const TagDiffSchema = TagSchema.partial();
export type TagDiff = ReadonlyDto<z.infer<typeof TagDiffSchema>>;
export const TagsDiffSchema = z.object({ removed: z.array(TagIdSchema).optional(), updated: z.array(z.object({ tag: TagIdSchema, diff: TagDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TagsDiff = ReadonlyDto<z.infer<typeof TagsDiffSchema>>;
// #endregion Tag

// #region Concept
export const ConceptSchema = z.object({ id: z.string(), name: z.string(), description: z.string().optional(), order: z.number().optional() });
export type ConceptPlain = ReadonlyDto<z.infer<typeof ConceptSchema>>;
export class Concept implements ConceptPlain {
  id!: string; name!: string; description?: string; order?: number;
  constructor(plain: ConceptPlain) { Object.assign(this, ConceptSchema.parse(plain)); }
  static from(plain: ConceptPlain): Concept { return new Concept(plain); }
  static fromPlain(plain: ConceptPlain): Concept { return new Concept(plain); }
  static createId(id: string): ConceptIdDto { return { id }; }
  static areSameId(a: ConceptIdDto, b: ConceptIdDto): boolean { return a.id === b.id; }
  toPlain(): ConceptPlain { return ConceptSchema.parse(this as ConceptPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Concept { return new Concept(ConceptSchema.parse(JSON.parse(json))); }
}
export const ConceptMetadataDtoSchema = ConceptSchema;
export type ConceptMetadataDto = ReadonlyDto<z.infer<typeof ConceptMetadataDtoSchema>>;
export const ConceptShallowSchema = ConceptSchema;
export type ConceptShallow = ReadonlyDto<z.infer<typeof ConceptShallowSchema>>;
export const ConceptDiffSchema = ConceptSchema.partial();
export type ConceptDiff = ReadonlyDto<z.infer<typeof ConceptDiffSchema>>;
export const ConceptsDiffSchema = z.object({ removed: z.array(ConceptIdSchema).optional(), updated: z.array(z.object({ concept: ConceptIdSchema, diff: ConceptDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConceptsDiff = ReadonlyDto<z.infer<typeof ConceptsDiffSchema>>;
// #endregion Concept

// #region Representation
export const RepresentationSchema = z.object({ id: z.string(), name: z.string().optional(), tags: z.array(TagIdSchema).optional(), file: FileIdSchema, description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type RepresentationPlain = ReadonlyDto<z.infer<typeof RepresentationSchema>>;
export class Representation implements RepresentationPlain {
  id!: string; name?: string; tags?: TagIdDto[]; file!: FileIdDto; description?: string; attributes?: Attribute[];
  constructor(plain: RepresentationPlain) { const p = RepresentationSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static fromPlain(plain: RepresentationPlain): Representation { return new Representation(plain); }
  static createId(id: string): RepresentationIdDto { return { id }; }
  static areSameId(a: RepresentationIdDto, b: RepresentationIdDto): boolean { return a.id === b.id; }
  toPlain(): RepresentationPlain { return RepresentationSchema.parse(this as RepresentationPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Representation { return new Representation(RepresentationSchema.parse(JSON.parse(json))); }
}
export const RepresentationMetadataDtoSchema = RepresentationSchema.omit({ tags: true, attributes: true });
export type RepresentationMetadataDto = ReadonlyDto<z.infer<typeof RepresentationMetadataDtoSchema>>;
export const RepresentationShallowSchema = RepresentationSchema;
export type RepresentationShallow = ReadonlyDto<z.infer<typeof RepresentationShallowSchema>>;
export const RepresentationDiffSchema = RepresentationSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type RepresentationDiff = ReadonlyDto<z.infer<typeof RepresentationDiffSchema>>;
export const RepresentationsDiffSchema = z.object({ removed: z.array(RepresentationIdSchema).optional(), updated: z.array(z.object({ representation: RepresentationIdSchema, diff: RepresentationDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type RepresentationsDiff = ReadonlyDto<z.infer<typeof RepresentationsDiffSchema>>;
// Removed: selectBestRepresentation, filterRepresentationsByTagIds, getAvailableTagIdsForRepresentations, getAllTagIdsFromRepresentations, findRepresentation, areSameRepresentation, SUPPORTED_3D_EXTENSIONS, isSupportedRepresentationExtension, validateRepresentationFile, RepresentationFileValidation — representation selection logic moved to semio/rs (Requirement 1.3)
// #endregion Representation

// #region Connector
export const ConnectorSchema = z.object({ id: z.string(), name: z.string().optional(), t: z.number(), point: PointSchema, direction: VectorSchema, description: z.string().optional(), port: PortIdSchema.optional(), mandatory: z.boolean().optional(), maxChildren: z.number().int().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectorPlain = ReadonlyDto<z.infer<typeof ConnectorSchema>>;
export class Connector implements ConnectorPlain {
  id!: string; name?: string; t!: number; point!: Point; direction!: Vector; description?: string; port?: PortIdDto; mandatory?: boolean; maxChildren?: number; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: ConnectorPlain) { const p = ConnectorSchema.parse(plain); Object.assign(this, p); this.point = new Point(p.point); this.direction = new Vector(p.direction); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static fromPlain(plain: ConnectorPlain): Connector { return new Connector(plain); }
  static createId(id: string): ConnectorIdDto { return { id }; }
  static areSameId(a: ConnectorIdDto, b: ConnectorIdDto): boolean { return a.id === b.id; }
  toPlain(): ConnectorPlain { return ConnectorSchema.parse(this as ConnectorPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connector { return new Connector(ConnectorSchema.parse(JSON.parse(json))); }
}
export const ConnectorMetadataDtoSchema = ConnectorSchema.omit({ props: true, attributes: true });
export type ConnectorMetadataDto = ReadonlyDto<z.infer<typeof ConnectorMetadataDtoSchema>>;
export const ConnectorShallowSchema = ConnectorSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type ConnectorShallow = ReadonlyDto<z.infer<typeof ConnectorShallowSchema>>;
export const ConnectorDiffSchema = ConnectorSchema.partial().omit({ point: true, direction: true, props: true, attributes: true }).extend({ point: PointDiffSchema.optional(), direction: VectorDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), maxChildren: z.number().int().nullable().optional() });
export type ConnectorDiff = ReadonlyDto<z.infer<typeof ConnectorDiffSchema>>;
export const ConnectorsDiffSchema = z.object({ removed: z.array(ConnectorIdSchema).optional(), updated: z.array(z.object({ connector: ConnectorIdSchema, diff: ConnectorDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectorsDiff = ReadonlyDto<z.infer<typeof ConnectorsDiffSchema>>;
// Removed: areConnectorsCompatible, unifyConnectorPortsAndCompatiblePortsForTypes, findConnector, findConnectorInType — connector compatibility moved to semio/rs (Requirement 1.5)
// #endregion Connector

// #region Type
export type EntityLifecycle = "active" | "deleted";
export const TypeSchema = z.object({
  id: z.string(),
  name: z.string(),
  parent: z.object({ id: z.string() }).optional(),
  families: z.array(FamilyIdSchema).optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  representations: z.array(RepresentationSchema).optional(),
  connectors: z.array(ConnectorSchema).optional(),
  props: z.array(PropSchema).optional(),
  stock: z.number().optional(),
  virtual: z.boolean().optional(),
  unit: z.string().optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  lifecycle: z.enum(["active", "deleted"]).optional(),
  deletedByUserId: z.string().optional(),
  deletedByDisplayName: z.string().optional(),
  deletedAt: z.string().optional(),
  deletedInChangeId: z.string().optional(),
});
export type TypePlain = ReadonlyDto<z.infer<typeof TypeSchema>>;
export class Type {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyIdDto[];
  isAbstract?: boolean;
  folder?: string;
  representations?: Representation[];
  connectors?: Connector[];
  props?: Prop[];
  stock?: number;
  virtual?: boolean;
  unit?: string;
  createdAt?: string;
  updatedAt?: string;
  location?: LocationIdDto;
  authors?: AuthorIdDto[];
  concepts?: ConceptIdDto[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  lifecycle?: EntityLifecycle;
  deletedByUserId?: string;
  deletedByDisplayName?: string;
  deletedAt?: string;
  deletedInChangeId?: string;
  constructor(plain: TypePlain) { const p = TypeSchema.parse(plain); Object.assign(this, p); this.representations = p.representations?.map((m) => new Representation(m)); this.connectors = p.connectors?.map((c) => new Connector(c)); this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: TypePlain): Type { return new Type(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Type { return Type.fromPlain(TypeSchema.parse(JSON.parse(json))); }
  toPlain(): TypePlain { return TypeSchema.parse({ ...(this as TypePlain) }); }
  static createId(id: string): TypeIdDto { return { id }; }
  static areSameId(a: TypeIdDto, b: TypeIdDto): boolean { return a.id === b.id; }
  /** @emoji 🖼️ Picks a representation for scene rendering (`@semio/ui`); first match until WASM metadata is wired. */
  static pickBestRepresentation(representations: readonly Representation[], _tagIds: readonly string[]): Representation | undefined {
    void _tagIds;
    return representations[0];
  }
}
export const TypeMetadataDtoSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true, authors: true, concepts: true });
export type TypeMetadataDto = ReadonlyDto<z.infer<typeof TypeMetadataDtoSchema>>;
export const TypeShallowSchema = TypeSchema.omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: z.array(RepresentationMetadataDtoSchema).optional(), connectors: z.array(ConnectorMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type TypeShallow = ReadonlyDto<z.infer<typeof TypeShallowSchema>>;
export const TypeDiffSchema = TypeSchema.partial().omit({ representations: true, connectors: true, props: true, attributes: true }).extend({ representations: RepresentationsDiffSchema.optional(), connectors: ConnectorsDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional(), description: z.string().nullable().optional(), icon: z.string().nullable().optional(), image: z.string().nullable().optional(), location: LocationIdSchema.nullable().optional(), folder: z.string().nullable().optional(), concepts: z.array(ConceptIdSchema).nullable().optional(), authors: z.array(AuthorIdSchema).nullable().optional(), families: z.array(FamilyIdSchema).nullable().optional(), lifecycle: z.enum(["active", "deleted"]).optional(), deletedByUserId: z.string().nullable().optional(), deletedByDisplayName: z.string().nullable().optional(), deletedAt: z.string().nullable().optional(), deletedInChangeId: z.string().nullable().optional() });
export type TypeDiff = ReadonlyDto<z.infer<typeof TypeDiffSchema>>;
export const TypesDiffSchema = z.object({ removed: z.array(TypeIdSchema).optional(), updated: z.array(z.object({ type: TypeIdSchema, diff: TypeDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type TypesDiff = ReadonlyDto<z.infer<typeof TypesDiffSchema>>;
// #endregion Type

// #region Layer
export const LayerSchema = z.object({ id: z.string(), path: z.string(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type LayerPlain = ReadonlyDto<z.infer<typeof LayerSchema>>;
export class Layer implements LayerPlain {
  id!: string; path!: string; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; attributes?: Attribute[];
  constructor(plain: LayerPlain) { const p = LayerSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: LayerPlain): Layer { return new Layer(plain); }
  static fromPlain(plain: LayerPlain): Layer { return new Layer(plain); }
  static createId(id: string): LayerIdDto { return { id }; }
  static areSameId(a: LayerIdDto, b: LayerIdDto): boolean { return a.id === b.id; }
  toPlain(): LayerPlain { return LayerSchema.parse(this as LayerPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Layer { return new Layer(LayerSchema.parse(JSON.parse(json))); }
}
export const LayerMetadataDtoSchema = LayerSchema.omit({ attributes: true });
export type LayerMetadataDto = ReadonlyDto<z.infer<typeof LayerMetadataDtoSchema>>;
export const LayerShallowSchema = LayerSchema;
export type LayerShallow = ReadonlyDto<z.infer<typeof LayerShallowSchema>>;
export const LayerDiffSchema = LayerSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type LayerDiff = ReadonlyDto<z.infer<typeof LayerDiffSchema>>;
export const LayersDiffSchema = z.object({ removed: z.array(LayerIdSchema).optional(), updated: z.array(z.object({ layer: LayerIdSchema, diff: LayerDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type LayersDiff = ReadonlyDto<z.infer<typeof LayersDiffSchema>>;
// #endregion Layer

// #region Piece
export const PieceSchema = z.object({ id: z.string(), name: z.string().optional(), type: TypeIdSchema.optional(), design: DesignIdSchema.optional(), plane: PlaneSchema.optional(), center: CoordinateSchema.optional(), scale: z.number().optional(), mirrorPlane: PlaneSchema.optional(), isHidden: z.boolean().optional(), isLocked: z.boolean().optional(), color: z.string().optional(), description: z.string().optional(), props: z.array(PropSchema).optional(), attributes: z.array(AttributeSchema).optional() });
export type PiecePlain = ReadonlyDto<z.infer<typeof PieceSchema>>;
export class Piece {
  id!: string; name?: string; type?: TypeIdDto; design?: DesignIdDto; plane?: Plane; center?: Coordinate; scale?: number; mirrorPlane?: Plane; isHidden?: boolean; isLocked?: boolean; color?: string; description?: string; props?: Prop[]; attributes?: Attribute[];
  constructor(plain: PiecePlain) { const p = PieceSchema.parse(plain); Object.assign(this, p); this.plane = p.plane ? new Plane(p.plane) : undefined; this.center = p.center ? new Coordinate(p.center) : undefined; this.mirrorPlane = p.mirrorPlane ? new Plane(p.mirrorPlane) : undefined; this.props = p.props?.map((x) => new Prop(x)); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static fromPlain(plain: PiecePlain): Piece { return new Piece(plain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Piece { return new Piece(PieceSchema.parse(JSON.parse(json))); }
  toPlain(): PiecePlain { return PieceSchema.parse(this as PiecePlain); }
  static createId(id: string): PieceIdDto { return { id }; }
  static areSameId(a: PieceIdDto, b: PieceIdDto): boolean { return a.id === b.id; }

  /** @emoji 🧭 Whether this piece wires a nested design id (schema hooks). */
  wireDesignAsPieceId(): boolean {
    return Boolean(this.design?.id);
  }

  /** @emoji 🧭 Wired type id for schema hooks. */
  wireTypeId(): { id: string } | undefined {
    return this.type ? { id: this.type.id } : undefined;
  }

  /** @emoji 🧭 Flat plane DTO for UI (structural truth in `semio/rs` reads). */
  flatPlane(): unknown {
    return this.plane ? this.plane.toPlain() : undefined;
  }

  /** @emoji 🧭 Flat center UV for UI. */
  flatCenter(): unknown {
    return this.center ? this.center.toPlain() : undefined;
  }

  /** @emoji 🧭 Alternative types for replaceable UI (populated from reads in full hosts). */
  alternativeTypes(): readonly Type[] {
    return [];
  }
}
export const PieceMetadataDtoSchema = PieceSchema.omit({ props: true, attributes: true });
export type PieceMetadataDto = ReadonlyDto<z.infer<typeof PieceMetadataDtoSchema>>;
export const PieceShallowSchema = PieceSchema.omit({ props: true }).extend({ props: z.array(PropMetadataDtoSchema).optional() });
export type PieceShallow = ReadonlyDto<z.infer<typeof PieceShallowSchema>>;
export const PieceDiffSchema = PieceSchema.partial().omit({ plane: true, props: true, attributes: true }).extend({ plane: PlaneDiffSchema.optional(), props: PropsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type PieceDiff = ReadonlyDto<z.infer<typeof PieceDiffSchema>>;
export const PiecesDiffSchema = z.object({ removed: z.array(PieceIdSchema).optional(), updated: z.array(z.object({ piece: PieceIdSchema, diff: PieceDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type PiecesDiff = ReadonlyDto<z.infer<typeof PiecesDiffSchema>>;
// Removed: isFixedPiece, findPiece, findPieceConnections, findConnectorForPieceInConnection, getPieceRepresentationFileIds, getPieceRepresentationUrls, resolvePieceTypeForFlatten — domain logic moved to semio/rs
// #endregion Piece

// #region Group
export const GroupSchema = z.object({ id: z.string(), pieces: z.array(PieceIdSchema), color: z.string().optional(), name: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type GroupPlain = ReadonlyDto<z.infer<typeof GroupSchema>>;
export class Group implements GroupPlain {
  id!: string; pieces!: PieceIdDto[]; color?: string; name?: string; description?: string; attributes?: Attribute[];
  constructor(plain: GroupPlain) { const p = GroupSchema.parse(plain); Object.assign(this, p); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  static from(plain: GroupPlain): Group { return new Group(plain); }
  static fromPlain(plain: GroupPlain): Group { return new Group(plain); }
  static createId(id: string): GroupIdDto { return { id }; }
  static areSameId(a: GroupIdDto, b: GroupIdDto): boolean { return a.id === b.id; }
  toPlain(): GroupPlain { return GroupSchema.parse(this as GroupPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Group { return new Group(GroupSchema.parse(JSON.parse(json))); }
}
export const GroupDiffSchema = GroupSchema.partial().omit({ attributes: true }).extend({ attributes: AttributesDiffSchema.optional() });
export type GroupDiff = ReadonlyDto<z.infer<typeof GroupDiffSchema>>;
export const GroupsDiffSchema = z.object({ removed: z.array(GroupIdSchema).optional(), updated: z.array(z.object({ group: GroupIdSchema, diff: GroupDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type GroupsDiff = ReadonlyDto<z.infer<typeof GroupsDiffSchema>>;
export const GroupMetadataDtoSchema = GroupSchema.omit({ pieces: true, attributes: true });
export type GroupMetadataDto = ReadonlyDto<z.infer<typeof GroupMetadataDtoSchema>>;
export const GroupShallowSchema = GroupSchema;
export type GroupShallow = ReadonlyDto<z.infer<typeof GroupShallowSchema>>;
// #endregion Group

// #region Side
export const SideSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SidePlain = ReadonlyDto<z.infer<typeof SideSchema>>;
export class Side {
  #pieceId!: string;
  #designPieceId?: string;
  #connectorId?: string;
  constructor(plain: SidePlain) { const p = SideSchema.parse(plain); this.#pieceId = p.piece.id; this.#designPieceId = p.designPiece?.id; this.#connectorId = p.connector?.id; }
  get piece(): PieceIdDto { return { id: this.#pieceId }; }
  get designPiece(): PieceIdDto | undefined { if (!this.#designPieceId) return undefined; return { id: this.#designPieceId }; }
  get connector(): ConnectorIdDto | undefined { return this.#connectorId !== undefined ? { id: this.#connectorId } : undefined; }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Side { return new Side(SideSchema.parse(JSON.parse(json))); }
  static from(plain: SidePlain): Side { return new Side(plain); }
  static fromPlain(plain: SidePlain): Side { return new Side(plain); }
  toPlain(): SidePlain { return SideSchema.parse({ piece: { id: this.#pieceId }, designPiece: this.#designPieceId ? { id: this.#designPieceId } : undefined, connector: this.#connectorId ? { id: this.#connectorId } : undefined }); }
}
export const SideDiffSchema = SideSchema.partial();
export type SideDiff = ReadonlyDto<z.infer<typeof SideDiffSchema>>;
export const SideIdSchema = z.object({ piece: PieceIdSchema, designPiece: PieceIdSchema.optional(), connector: ConnectorIdSchema.optional() });
export type SideIdPlain = ReadonlyDto<z.infer<typeof SideIdSchema>>;
export class SideId implements SideIdPlain {
  piece!: PieceIdDto; designPiece?: PieceIdDto; connector?: ConnectorIdDto;
  constructor(plain: SideIdPlain) { Object.assign(this, SideIdSchema.parse(plain)); }
  static from(plain: SideIdPlain): SideId { return new SideId(plain); }
  toPlain(): SideIdPlain { return SideIdSchema.parse(this as SideIdPlain); }
}
export const SidesDiffSchema = z.object({ removed: z.array(SideIdSchema).optional(), updated: z.array(z.object({ side: SideIdSchema, diff: SideDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type SidesDiff = ReadonlyDto<z.infer<typeof SidesDiffSchema>>;
// #endregion Side

// #region Connection
export const ConnectionSchema = z.object({ id: z.string(), connected: SideSchema, connecting: SideSchema, gap: z.number().optional(), shift: z.number().optional(), rise: z.number().optional(), rotation: z.number().optional(), turn: z.number().optional(), tilt: z.number().optional(), u: z.number().optional(), v: z.number().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional() });
export type ConnectionPlain = ReadonlyDto<z.infer<typeof ConnectionSchema>>;
export class Connection implements ConnectionPlain {
  id!: string; connected!: Side; connecting!: Side; gap?: number; shift?: number; rise?: number; rotation?: number; turn?: number; tilt?: number; u?: number; v?: number; description?: string; attributes?: Attribute[];
  constructor(plain: ConnectionPlain) { const p = ConnectionSchema.parse(plain); Object.assign(this, p); this.connected = new Side(p.connected); this.connecting = new Side(p.connecting); this.attributes = p.attributes?.map((a) => new Attribute(a)); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Connection { return new Connection(ConnectionSchema.parse(JSON.parse(json))); }
  static from(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static fromPlain(plain: ConnectionPlain): Connection { return new Connection(plain); }
  static createId(id: string): ConnectionIdDto { return { id }; }
  static areSameId(a: ConnectionIdDto, b: ConnectionIdDto): boolean { return a.id === b.id; }
  toPlain(): ConnectionPlain { return ConnectionSchema.parse({ id: this.id, connected: this.connected.toPlain(), connecting: this.connecting.toPlain(), gap: this.gap, shift: this.shift, rise: this.rise, rotation: this.rotation, turn: this.turn, tilt: this.tilt, u: this.u, v: this.v, description: this.description, attributes: this.attributes?.map((a) => a.toPlain()) } as ConnectionPlain); }
}
export const ConnectionDiffSchema = ConnectionSchema.partial().omit({ id: true, connected: true, connecting: true, attributes: true }).extend({ connected: SideDiffSchema.optional(), connecting: SideDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type ConnectionDiff = ReadonlyDto<z.infer<typeof ConnectionDiffSchema>>;
export const ConnectionsDiffSchema = z.object({ removed: z.array(ConnectionIdSchema).optional(), updated: z.array(z.object({ connection: ConnectionIdSchema, diff: ConnectionDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type ConnectionsDiff = ReadonlyDto<z.infer<typeof ConnectionsDiffSchema>>;
export const ConnectionMetadataDtoSchema = ConnectionSchema.omit({ attributes: true });
export type ConnectionMetadataDto = ReadonlyDto<z.infer<typeof ConnectionMetadataDtoSchema>>;
export const ConnectionShallowSchema = ConnectionSchema;
export type ConnectionShallow = ReadonlyDto<z.infer<typeof ConnectionShallowSchema>>;
// #endregion Connection

// #region Stat
export const StatSchema = z.object({ id: z.string(), quality: QualityIdSchema, unit: z.string().optional(), min: z.number().optional(), minExcluded: z.boolean().optional(), max: z.number().optional(), maxExcluded: z.boolean().optional() });
export type StatPlain = ReadonlyDto<z.infer<typeof StatSchema>>;
export class Stat implements StatPlain {
  id!: string; quality!: QualityIdDto; unit?: string; min?: number; minExcluded?: boolean; max?: number; maxExcluded?: boolean;
  constructor(plain: StatPlain) { Object.assign(this, StatSchema.parse(plain)); }
  static from(plain: StatPlain): Stat { return new Stat(plain); }
  static fromPlain(plain: StatPlain): Stat { return new Stat(plain); }
  static createId(id: string): StatIdDto { return { id }; }
  static areSameId(a: StatIdDto, b: StatIdDto): boolean { return a.id === b.id; }
  toPlain(): StatPlain { return StatSchema.parse(this as StatPlain); }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Stat { return new Stat(StatSchema.parse(JSON.parse(json))); }
}
export const StatDiffSchema = StatSchema.partial();
export type StatDiff = ReadonlyDto<z.infer<typeof StatDiffSchema>>;
export const StatsDiffSchema = z.object({ removed: z.array(StatIdSchema).optional(), updated: z.array(z.object({ stat: StatIdSchema, diff: StatDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type StatsDiff = ReadonlyDto<z.infer<typeof StatsDiffSchema>>;
export const StatMetadataDtoSchema = StatSchema;
export type StatMetadataDto = ReadonlyDto<z.infer<typeof StatMetadataDtoSchema>>;
export const StatShallowSchema = StatSchema;
export type StatShallow = ReadonlyDto<z.infer<typeof StatShallowSchema>>;
// #endregion Stat

// #region Design
export const DesignSchema = z.object({
  id: z.string(),
  name: z.string(),
  parent: z.object({ id: z.string() }).optional(),
  families: z.array(FamilyIdSchema).optional(),
  isAbstract: z.boolean().optional(),
  folder: z.string().optional(),
  pieces: z.array(PieceSchema).optional(),
  connections: z.array(ConnectionSchema).optional(),
  stats: z.array(StatSchema).optional(),
  props: z.array(PropSchema).optional(),
  layers: z.array(LayerSchema).optional(),
  activeLayer: LayerIdSchema.optional(),
  groups: z.array(GroupSchema).optional(),
  canScale: z.boolean().optional(),
  canMirror: z.boolean().optional(),
  unit: z.string().optional(),
  location: LocationIdSchema.optional(),
  authors: z.array(AuthorIdSchema).optional(),
  concepts: z.array(ConceptIdSchema).optional(),
  icon: z.string().optional(),
  image: z.string().optional(),
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional(),
  createdAt: DateProperty(),
  updatedAt: DateProperty(),
});
export type DesignPlain = ReadonlyDto<z.infer<typeof DesignSchema>>;

export const DesignDiffSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, authors: true, attributes: true }).partial().extend({ pieces: PiecesDiffSchema.optional(), connections: ConnectionsDiffSchema.optional(), stats: StatsDiffSchema.optional(), props: PropsDiffSchema.optional(), layers: LayersDiffSchema.optional(), groups: GroupsDiffSchema.optional(), authors: AuthorsDiffSchema.optional(), attributes: AttributesDiffSchema.optional() });
export type DesignDiff = ReadonlyDto<z.infer<typeof DesignDiffSchema>>;
export const DesignsDiffSchema = z.object({ removed: z.array(DesignIdSchema).optional(), updated: z.array(z.object({ design: DesignIdSchema, diff: DesignDiffSchema })).optional(), added: z.array(z.any()).optional() });
export type DesignsDiff = ReadonlyDto<z.infer<typeof DesignsDiffSchema>>;

/** @emoji ⚠️ Algorithm adapter / native REST error row. */
export type AlgorithmError = { readonly code: string; readonly message: string };
export type DesignDiffOperationResult = { readonly ok: true; readonly diff: DesignDiff } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };
export type OperationResult<T> = { readonly ok: true; readonly value: T } | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

/** @emoji 🔧 Gap/shift/rise knobs for structural move previews (algorithms UI). */
export type MoveVector = { readonly gap: number; readonly shift: number; readonly rise: number };

/** @emoji 📌 Paste anchoring modes for copy/paste algorithm stories. */
export type PasteDesignAnchoringKind =
  | "original"
  | "middle"
  | "centroid"
  | "bottomLeft"
  | "bottomRight"
  | "topLeft"
  | "topRight";

/** @emoji 🧠 Optional per-piece flatten cache row (TS algorithm path; opaque to callers). */
export type FlatMerkleCacheEntry = Readonly<JsonObject>;

export class Design {
  id!: string;
  name!: string;
  parent?: { id: string };
  families?: FamilyIdDto[];
  isAbstract?: boolean;
  folder?: string;
  pieces?: Piece[];
  _connections?: Connection[];
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: LayerIdDto;
  groups?: Group[];
  canScale?: boolean;
  canMirror?: boolean;
  unit?: string;
  location?: LocationIdDto;
  authors?: AuthorIdDto[];
  concepts?: ConceptIdDto[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
  createdAt!: string;
  updatedAt!: string;
  get connections(): Connection[] | undefined {
    return this._connections;
  }
  constructor(plain: DesignPlain | Design) {
    const wire: DesignPlain = plain instanceof Design ? plain.toPlain() : plain;
    const p = DesignSchema.parse(wire);
    const { connections: _wcon, pieces: _wp, ...rest } = p;
    Object.assign(this, rest);
    this.pieces = p.pieces?.map((x) => new Piece(x));
    this._connections = p.connections?.map((x) => new Connection(x));
    this.stats = p.stats?.map((x) => new Stat(x));
    this.props = p.props?.map((x) => new Prop(x));
    this.layers = p.layers?.map((x) => new Layer(x));
    this.groups = p.groups?.map((x) => new Group(x));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromPlain(plain: DesignPlain): Design { return new Design(plain); }
  toPlain(): DesignPlain {
    return DesignSchema.parse({
      ...(this as DesignPlain),
      pieces: this.pieces?.map((x) => x.toPlain()),
      connections: this._connections?.map((x) => x.toPlain()),
      stats: this.stats?.map((x) => x.toPlain()),
      props: this.props?.map((x) => x.toPlain()),
      layers: this.layers?.map((x) => x.toPlain()),
      groups: this.groups?.map((x) => x.toPlain()),
      attributes: this.attributes?.map((x) => x.toPlain()),
    });
  }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Design { return new Design(DesignSchema.parse(JSON.parse(json))); }
  static createId(id: string): DesignIdDto { return { id }; }
  static areSameId(a: DesignIdDto, b: DesignIdDto): boolean { return a.id === b.id; }

  /** @emoji 🧭 Included / sibling designs for nested-design UI (DTO navigation). */
  getDesignFamily(): Design[] {
    return [];
  }

  /** @emoji 🧾 Legacy alias for diagram consumers (`@semio/ui`). */
  getConnections(): Connection[] {
    return [...(this._connections ?? [])];
  }

  /** @emoji 🧾 Non-mutating diff overlay for MCP / diagram previews. */
  static previewWithDiff(design: Design, diff: DesignDiff): Design {
    const plain = design instanceof Design ? design.toPlain() : DesignSchema.parse(design as DesignPlain);
    const n = new Design(plain);
    n.applyDiff(diff);
    return n;
  }

  /** @emoji 🧩 Merges a structural {@link DesignDiff} into this design (pieces + connections). */
  applyDiff(diff: DesignDiff): void {
    if (diff.pieces?.removed?.length) {
      const rm = new Set(diff.pieces.removed.map((x) => x.id));
      this.pieces = (this.pieces ?? []).filter((p) => !rm.has(p.id));
    }
    if (diff.pieces?.updated?.length) {
      for (const u of diff.pieces.updated) {
        const p = (this.pieces ?? []).find((x) => x.id === u.piece.id);
        if (!p) continue;
        const d = u.diff;
        if (d.name !== undefined) p.name = d.name;
        if (d.scale !== undefined) p.scale = d.scale;
        if (d.center) {
          const c = p.center ? p.center.toPlain() : { u: 0, v: 0 };
          p.center = new Coordinate({ ...c, ...d.center });
        }
        if (d.plane && p.plane) {
          const pl = p.plane.toPlain();
          const o = d.plane.origin ? { ...pl.origin, ...d.plane.origin } : pl.origin;
          const xa = d.plane.xAxis ? { ...pl.xAxis, ...d.plane.xAxis } : pl.xAxis;
          const ya = d.plane.yAxis ? { ...pl.yAxis, ...d.plane.yAxis } : pl.yAxis;
          p.plane = new Plane({ origin: o, xAxis: xa, yAxis: ya });
        }
      }
    }
    if (diff.pieces?.added?.length) {
      this.pieces = [...(this.pieces ?? []), ...diff.pieces.added.map((x) => new Piece(PieceSchema.parse(x as PiecePlain)))];
    }
    if (diff.connections?.removed?.length) {
      const rm = new Set(diff.connections.removed.map((x) => x.id));
      this._connections = (this._connections ?? []).filter((c) => !rm.has(c.id));
    }
    if (diff.connections?.updated?.length) {
      for (const u of diff.connections.updated) {
        const c = (this._connections ?? []).find((x) => x.id === u.connection.id);
        if (!c) continue;
        Object.assign(c, u.diff);
      }
    }
    if (diff.connections?.added?.length) {
      this._connections = [
        ...(this._connections ?? []),
        ...diff.connections.added.map((x) => new Connection(ConnectionSchema.parse(x as z.infer<typeof ConnectionSchema>))),
      ];
    }
  }

  /** @emoji 🧾 Selection drag in flat UV space (piece centers only; algorithm preview). */
  dragBySelection(piecesDesign: Design, offset: CoordinatePlain): DesignDiff {
    const du = offset.u ?? 0;
    const dv = offset.v ?? 0;
    const sel = new Set((piecesDesign.pieces ?? []).map((p) => p.id));
    const updated = (this.pieces ?? [])
      .filter((p) => sel.has(p.id))
      .map((p) => {
        const c = p.center?.toPlain() ?? { u: 0, v: 0 };
        return { piece: { id: p.id }, diff: { center: { u: c.u + du, v: c.v + dv } } };
      });
    return { pieces: { updated } };
  }

  /** @emoji 🗑️ Diff removing the given pieces and connections (preview-only; kit graph unchanged). */
  deletePiecesAndConnectionsDiff(pieceIds: readonly string[], connectionIds: readonly string[]): DesignDiffOperationResult {
    return {
      ok: true,
      diff: {
        pieces: { removed: pieceIds.map((id) => ({ id })) },
        connections: { removed: connectionIds.map((id) => ({ id })) },
      },
    };
  }
}

export type DesignOperationResult =
  | { readonly ok: true; readonly design: Design; readonly diff: { forward: DesignDiff; reverse: DesignDiff } }
  | { readonly ok: false; readonly errors: readonly AlgorithmError[] };

/** @emoji 🧾 Coerces native REST flatten payloads into {@link DesignOperationResult}. */
export function normalizeDesignFlattenResult(raw: unknown): DesignOperationResult {
  return raw as DesignOperationResult;
}
/** @emoji 🧾 Coerces native REST diff payloads into {@link DesignDiffOperationResult}. */
export function normalizeDesignDiffResult(raw: unknown): DesignDiffOperationResult {
  return raw as DesignDiffOperationResult;
}
/** @emoji 🧾 Coerces native REST copy payloads into {@link OperationResult}<{@link Design}>. */
export function normalizeDesignCopyResult(raw: unknown): OperationResult<Design> {
  return raw as OperationResult<Design>;
}

export const DesignMetadataDtoSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true, authors: true, concepts: true });
export type DesignMetadataDto = ReadonlyDto<z.infer<typeof DesignMetadataDtoSchema>>;
export const DesignShallowSchema = DesignSchema.omit({ pieces: true, connections: true, stats: true, props: true, layers: true, groups: true, attributes: true }).extend({ pieces: z.array(PieceMetadataDtoSchema).optional(), connections: z.array(ConnectionMetadataDtoSchema).optional(), stats: z.array(StatMetadataDtoSchema).optional(), props: z.array(PropMetadataDtoSchema).optional(), layers: z.array(LayerMetadataDtoSchema).optional(), groups: z.array(GroupMetadataDtoSchema).optional(), attributes: z.array(AttributeMetadataDtoSchema).optional() });
export type DesignShallow = ReadonlyDto<z.infer<typeof DesignShallowSchema>>;
// Removed: addPieceToDesignDiff, setPieceInDesignDiff, removePieceFromDesignDiff, addPiecesToDesignDiff, setPiecesInDesignDiff, removePiecesFromDesignDiff, addConnectionToDesignDiff, setConnectionInDesignDiff, removeConnectionFromDesignDiff, addConnectionsToDesignDiff, setConnectionsInDesignDiff, removeConnectionsFromDesignDiff, mergeDesigns, orientDesign, duplicateDesignDiffForIsolation — design-diff builder functions moved to semio/rs (Requirement 3.7)
// #endregion Design

// #region 🧾KitStoreClientChildCommands

/** @emoji 🧾 Child DTO kinds for kit-root add/remove commands (port rows use type-scoped flows, not top-level add). */
export type KitChildEntityKind =
  | "Family"
  | "Author"
  | "Concept"
  | "Tag"
  | "Quality"
  | "File"
  | "Folder"
  | "Type"
  | "Design"
  | "Port";

/** @emoji 🧾 Parses `dto` with the matching zod schema and runs `add*` kit wires. */
export async function kitStoreClientAddChildByKind(client: KitStoreClient, childKind: string, dto: unknown): Promise<SetResult> {
  let cmds: readonly ChangeKitCommand[];
  try {
    switch (childKind) {
      case "Family":
        cmds = [{ addFamily: { family: FamilySchema.parse(dto) } }];
        break;
      case "Author":
        cmds = [{ addAuthor: { author: AuthorSchema.parse(dto) } }];
        break;
      case "Concept":
        cmds = [{ addConcept: { concept: ConceptSchema.parse(dto) } }];
        break;
      case "Tag":
        cmds = [{ addTag: { tag: TagSchema.parse(dto) } }];
        break;
      case "Quality":
        cmds = [{ addQuality: { quality: QualitySchema.parse(dto) } }];
        break;
      case "File":
        cmds = [{ addFile: { file: FileSchema.parse(dto) } }];
        break;
      case "Folder":
        cmds = [{ addFolder: { folder: FolderSchema.parse(dto) } }];
        break;
      case "Type":
        cmds = [{ addType: { type: TypeSchema.parse(dto) } }];
        break;
      case "Design":
        cmds = [{ addDesign: { design: DesignSchema.parse(dto) } }];
        break;
      case "Port":
        return { ok: false, error: { kind: "NotSupported", message: "add Port: use type-scoped kit commands" } };
      default:
        return { ok: false, error: { kind: "NotSupported", message: `add to kit: ${childKind}` } };
    }
  } catch (err) {
    return { ok: false, error: { kind: "InvalidValue", message: String(err) } };
  }
  return client.submitChangeKitCommands(cmds);
}

/** @emoji 🧾 Emits matching `remove*` kit wire for a kit-root child id. */
export async function kitStoreClientRemoveChildByKind(client: KitStoreClient, childKind: string, childId: string): Promise<SetResult> {
  const idw = { id: childId };
  let cmds: readonly ChangeKitCommand[];
  switch (childKind) {
    case "Family":
      cmds = [{ removeFamily: { familyId: idw } }];
      break;
    case "Author":
      cmds = [{ removeAuthor: { authorId: idw } }];
      break;
    case "Concept":
      cmds = [{ removeConcept: { conceptId: idw } }];
      break;
    case "Tag":
      cmds = [{ removeTag: { tagId: idw } }];
      break;
    case "Quality":
      cmds = [{ removeQuality: { qualityId: idw } }];
      break;
    case "File":
      cmds = [{ removeFile: { fileId: idw } }];
      break;
    case "Folder":
      cmds = [{ removeFolder: { folderId: idw } }];
      break;
    case "Type":
      cmds = [{ removeType: { typeId: idw } }];
      break;
    case "Design":
      cmds = [{ removeDesign: { designId: idw } }];
      break;
    case "Port":
      return { ok: false, error: { kind: "NotSupported", message: "remove Port: use type-scoped kit commands" } };
    default:
      return { ok: false, error: { kind: "NotSupported", message: `remove from kit: ${childKind}` } };
  }
  return client.submitChangeKitCommands(cmds);
}

/** @emoji 🧾 Adds a piece under a design (`addPiece` wire). */
export async function kitStoreClientAddPiece(client: KitStoreClient, designId: string, piece: unknown): Promise<SetResult> {
  return client.submitChangeKitCommands([
    { changeDesignCommands: { designId: { id: designId }, commands: [{ addPiece: { piece: PieceSchema.parse(piece) } }] } },
  ]);
}

/** @emoji 🧾 Removes a piece from a design (`removePiece` wire). */
export async function kitStoreClientRemovePiece(client: KitStoreClient, designId: string, pieceId: string): Promise<SetResult> {
  return client.submitChangeKitCommands([
    { changeDesignCommands: { designId: { id: designId }, commands: [{ removePiece: { pieceId: { id: pieceId } } }] } },
  ]);
}

/** @emoji 🧾 Adds a connection under a design (`addConnection` wire). */
export async function kitStoreClientAddConnection(client: KitStoreClient, designId: string, connection: unknown): Promise<SetResult> {
  return client.submitChangeKitCommands([
    {
      changeDesignCommands: {
        designId: { id: designId },
        commands: [{ addConnection: { connection: ConnectionSchema.parse(connection) } }],
      },
    },
  ]);
}

// #endregion 🧾KitStoreClientChildCommands

// #region Kit
export const KitKindSchema = z.enum(["dev", "local", "archive", "remote", "transport"]);
export type KitKind = ReadonlyDto<z.infer<typeof KitKindSchema>>;
export const ALL_KIT_KINDS: readonly KitKind[] = KitKindSchema.options;

export const KitFullDtoSchema = z.object({ id: z.string(), name: z.string(), version: z.string().optional(), types: z.array(TypeSchema).optional(), designs: z.array(DesignSchema).optional(), tags: z.array(TagSchema).optional(), concepts: z.array(ConceptSchema).optional(), families: z.array(FamilySchema).optional(), qualities: z.array(QualitySchema).optional(), files: z.array(FileSchema).optional(), folders: z.array(FolderSchema).optional(), authors: z.array(AuthorSchema).optional(), remote: z.string().optional(), homepage: z.string().optional(), license: z.string().optional(), preview: z.string().optional(), icon: z.string().optional(), image: z.string().optional(), description: z.string().optional(), attributes: z.array(AttributeSchema).optional(), createdAt: DateProperty(), updatedAt: DateProperty() });
export type KitFullDto = ReadonlyDto<z.infer<typeof KitFullDtoSchema>>;

function semioCoerceKitFullDtoFromJson(v: KitJsonTreeDto | KitFullDto): KitFullDto {
  return KitFullDtoSchema.parse(v);
}

function semioParseTypeShallowArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly TypeShallow[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) =>
    row && typeof row === "object" && !Array.isArray(row)
      ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject))
      : row,
  );
  const r = z.array(TypeShallowSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseDesignShallowArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly DesignShallow[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) =>
    row && typeof row === "object" && !Array.isArray(row)
      ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject))
      : row,
  );
  const r = z.array(DesignShallowSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseKitIdDtoArray(v: KitJsonTreeDto | string | undefined | null): readonly KitIdDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v);
  const out: KitIdDto[] = [];
  for (const x of xs) {
    if (x != null && typeof x === "object" && !Array.isArray(x) && "id" in x && typeof (x as { id: KitJsonTreeDto }).id === "string")
      out.push({ id: (x as { id: string }).id });
    else if (typeof x === "string") out.push({ id: x });
  }
  return out;
}

function semioParseTypeMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly TypeMetadataDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) =>
    row && typeof row === "object" && !Array.isArray(row)
      ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject))
      : row,
  );
  const r = z.array(TypeMetadataDtoSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseDesignMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly DesignMetadataDto[] {
  const xs = kitGraphqlJsonToReadonlyArray(v).map((row) =>
    row && typeof row === "object" && !Array.isArray(row)
      ? __stripTopLevelJsonNulls(__normalizeTypeOrDesignMetadataRow(row as JsonObject))
      : row,
  );
  const r = z.array(DesignMetadataDtoSchema).safeParse(xs);
  return r.success ? r.data : [];
}

function semioParseAuthorMetadataArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly AuthorMetadataDto[] {
  const r = z.array(AuthorMetadataDtoSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

function semioParseKitMetadataJson(v: KitJsonTreeDto | undefined | null): KitMetadataDto | null {
  if (v == null || typeof v !== "object" || Array.isArray(v)) return null;
  return v as KitMetadataDto;
}

function semioParseColoredConnectorRowsJson(v: KitJsonTreeDto | readonly KitJsonTreeDto[] | undefined | null): readonly KitColoredConnectorRowDto[] {
  if (Array.isArray(v)) {
    const out: KitColoredConnectorRowDto[] = [];
    for (const row of v) {
      if (row && typeof row === "object" && !Array.isArray(row) && "color" in row) {
        const r = row as { typeId?: { id?: string }; connectorId?: { id?: string }; color?: string };
        if (typeof r.color === "string" && r.typeId && r.connectorId) {
          const tid = typeof r.typeId.id === "string" ? r.typeId.id : "";
          const cid = typeof r.connectorId.id === "string" ? r.connectorId.id : "";
          if (tid && cid) out.push({ typeId: { id: tid }, connectorId: { id: cid }, color: r.color });
        }
      }
    }
    return out;
  }
  return [];
}

function semioParsePiecePlainArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly PiecePlain[] {
  const r = z.array(PieceSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

function semioParseConnectionPlainArrayJson(v: KitJsonTreeDto | string | undefined | null): readonly ConnectionPlain[] {
  const r = z.array(ConnectionSchema).safeParse(kitGraphqlJsonToReadonlyArray(v));
  return r.success ? r.data : [];
}

const includedDesignInfoJsonZod = z.object({
  id: z.string(),
  designId: z.string(),
  connectionKind: z.string(),
  center: PointSchema.nullable().optional(),
  plane: PlaneSchema.nullable().optional(),
  externalConnections: z.array(ConnectionSchema).optional(),
});

const piecePlacementRowJsonZod = z.object({
  pieceId: z.string(),
  plane: PlaneSchema,
  center: PointSchema,
  fixedPieceId: z.string(),
  parentPieceId: z.string().nullable(),
  depth: z.number(),
  path: z.array(z.string()),
});

function semioParseDesignIncludedDesignArrayJson(v: KitJsonTreeDto | readonly KitJsonTreeDto[] | undefined | null): readonly IncludedDesignInfoDto[] {
  const r = z.array(includedDesignInfoJsonZod).safeParse(Array.isArray(v) ? v : kitGraphqlJsonToReadonlyArray(v));
  return r.success ? (r.data as readonly IncludedDesignInfoDto[]) : [];
}

function semioParsePiecePlacementMapJson(rows: readonly unknown[] | undefined | null): ReadonlyMap<string, PiecePlacementRowDto> {
  const m = new Map<string, PiecePlacementRowDto>();
  if (!Array.isArray(rows)) return m;
  for (const r of rows) {
    const row = piecePlacementRowJsonZod.safeParse(r);
    if (row.success) m.set(row.data.pieceId, row.data);
  }
  return m;
}

function semioParsePlaneNullableJson(v: KitJsonTreeDto | undefined | null): PlanePlain | null {
  const p = PlaneSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseCoordinateNullableJson(v: KitJsonTreeDto | undefined | null): CoordinatePlain | null {
  const p = CoordinateSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseConnectionNullableJson(v: KitJsonTreeDto | undefined | null): ConnectionPlain | null {
  const p = ConnectionSchema.safeParse(v);
  return p.success ? p.data : null;
}

function semioParseRepresentationNullableJson(v: KitJsonTreeDto | undefined | null): RepresentationPlain | null {
  const p = RepresentationSchema.safeParse(v);
  return p.success ? p.data : null;
}

/** @emoji 🧾 Fills missing `folders[].path` from legacy `name` + `parent` before {@link FolderSchema} parse. */
export function normalizeKitFullDtoFolderPaths(dto: KitFullDto): KitFullDto {
  const foldersUnknown = (dto as { folders?: unknown }).folders;
  if (!Array.isArray(foldersUnknown) || foldersUnknown.length === 0) return dto;
  const list = foldersUnknown as Array<JsonObject>;
  const byId = new Map<string, JsonObject>();
  for (const row of list) {
    if (row && typeof row.id === "string") byId.set(row.id, row);
  }
  const resolvePath = (f: JsonObject, visiting: Set<string>): string => {
    const fid = typeof f.id === "string" ? f.id : "";
    const existing = f.path;
    if (typeof existing === "string" && existing.length > 0) return existing;
    if (fid && visiting.has(fid)) return String(f.name ?? fid);
    if (fid) visiting.add(fid);
    const seg = String((f.name as string | undefined) ?? (fid || "folder"));
    const parent = f.parent as { id?: string } | undefined;
    const pid = parent?.id != null ? String(parent.id) : "";
    if (pid && byId.has(pid)) {
      const base = resolvePath(byId.get(pid)!, visiting);
      if (fid) visiting.delete(fid);
      return base ? `${base}/${seg}` : seg;
    }
    if (fid) visiting.delete(fid);
    return seg;
  };
  const nextFolders = list.map((row) => ({ ...row, path: resolvePath(row, new Set()) }));
  return { ...(dto as object), folders: nextFolders } as unknown as KitFullDto;
}

export class Kit {
  /** @emoji 📌 Anchoring kinds exposed to copy/paste algorithm UI. */
  static readonly pasteDesignAnchoringKinds: readonly PasteDesignAnchoringKind[] = [
    "original",
    "middle",
    "centroid",
    "bottomLeft",
    "bottomRight",
    "topLeft",
    "topRight",
  ];

  /** @emoji 🧭 Normalizes plain/DTO kit records to a {@link Kit} entity (replaces legacy `Kit.ensure`). */
  static ensure(kit: Kit | KitFullDto): Kit {
    return kit instanceof Kit ? kit : Kit.fromPlain(kit as KitFullDto);
  }

  /** @emoji 📋 Copy selection (TS path stub — use REST language or extend with KitStore batch). */
  copyDesignOp(_design: Design, _pieceIds: readonly string[], _connectionIds: readonly string[]): OperationResult<Design> {
    void _design;
    void _pieceIds;
    void _connectionIds;
    return { ok: false, errors: [{ code: "native.copy.ts", message: "nativeCopyDesign(ts): not wired to WASM batch yet; switch language or implement batch copy." }] };
  }

  /** @emoji 📋 Paste selection (TS path stub). */
  pasteDesignOp(_source: Design, _target: Design, _anchoring: string, _coordinate: CoordinatePlain | undefined): DesignDiff {
    void _source;
    void _target;
    void _anchoring;
    void _coordinate;
    return {};
  }

  id!: string; name!: string; version?: string; types?: Type[]; designs?: Design[]; tags?: Tag[]; concepts?: Concept[]; families?: Family[]; qualities?: Quality[]; files?: File[]; folders?: Folder[]; authors?: Author[]; remote?: string; homepage?: string; license?: string; preview?: string; icon?: string; image?: string; description?: string; attributes?: Attribute[]; createdAt!: string; updatedAt!: string;
  constructor(data: KitFullDto) {
    const p = KitFullDtoSchema.parse(normalizeKitFullDtoFolderPaths(data));
    Object.assign(this, p);
    this.types = p.types?.map((t) => new Type(t));
    this.designs = p.designs?.map((d) => new Design(d));
    this.tags = p.tags?.map((t) => new Tag(t));
    this.concepts = p.concepts?.map((c) => new Concept(c));
    this.families = p.families?.map((f) => new Family(f));
    this.qualities = p.qualities?.map((q) => new Quality(q));
    this.files = p.files?.map((f) => new File(f));
    this.folders = p.folders?.map((f) => new Folder(f));
    this.authors = p.authors?.map((a) => new Author(a));
    this.attributes = p.attributes?.map((a) => new Attribute(a));
  }
  static fromPlain(data: KitFullDto): Kit { return new Kit(data); }
  toPlain(): KitFullDto {
    return KitFullDtoSchema.parse({
      ...(this as KitFullDto),
      types: this.types?.map((t) => t.toPlain()),
      designs: this.designs?.map((d) => d.toPlain()),
      tags: this.tags?.map((t) => t.toPlain()),
      concepts: this.concepts?.map((c) => c.toPlain()),
      families: this.families?.map((f) => f.toPlain()),
      qualities: this.qualities?.map((q) => q.toPlain()),
      files: this.files?.map((f) => f.toPlain()),
      folders: this.folders?.map((f) => f.toPlain()),
      authors: this.authors?.map((a) => a.toPlain()),
      attributes: this.attributes?.map((a) => a.toPlain()),
    });
  }
  serialize(): string { return JSON.stringify(this.toPlain()); }
  static deserialize(json: string): Kit { return Kit.fromPlain(KitFullDtoSchema.parse(JSON.parse(json))); }
  toJSON(): KitFullDto { return this.toPlain(); }
  static createId(id: string): KitIdDto { return { id }; }
  static areSameId(a: KitIdDto, b: KitIdDto): boolean { return a.id === b.id; }

  /** @emoji 🧭 Resolve a design by id (DTO graph navigation for React schema hooks). */
  findDesign(id: string): Design | undefined {
    return this.designs?.find((d) => d.id === id);
  }

  /** @emoji 🧭 Resolve a type by id. */
  findType(id: string): Type | undefined {
    return this.types?.find((t) => t.id === id);
  }

  /** @emoji 🧭 Flatten / parent metadata map (DTO host; WASM bridge may supply richer maps). */
  piecesMetadataFor(_designId: string): { ok: true; diff: Map<string, { parentPieceId?: string }> } | { ok: false; diff?: undefined } {
    void _designId;
    return { ok: true, diff: new Map() };
  }

  /** @emoji 🧭 Parent piece for `pieceId` via connection graph (connecting → connected). */
  findParentPieceInDesign(designId: string, pieceId: string): Piece | undefined {
    const d = this.findDesign(designId);
    if (!d?._connections || !d.pieces) return undefined;
    for (const c of d._connections) {
      const connectingId = c.connecting?.piece?.id;
      if (connectingId !== pieceId) continue;
      const parentId = c.connected?.piece?.id;
      if (!parentId) return undefined;
      return d.pieces.find((p) => p.id === parentId);
    }
    return undefined;
  }

  /** @emoji 🧭 Parent connection whose connecting side matches `pieceId`. */
  findParentConnectionForPieceInDesign(designId: string, pieceId: string): Connection | undefined {
    const d = this.findDesign(designId);
    if (!d?._connections) return undefined;
    for (const c of d._connections) {
      if (c.connecting?.piece?.id === pieceId) return c;
    }
    return undefined;
  }

  /** @emoji 🧭 Child pieces: connections where connected side is `parentPieceId` and connecting side is another piece. */
  findChildrenPiecesInDesign(designId: string, parentPieceId: string): Piece[] {
    const d = this.findDesign(designId);
    if (!d?._connections || !d.pieces) return [];
    const out: Piece[] = [];
    for (const c of d._connections) {
      if (c.connected?.piece?.id !== parentPieceId) continue;
      const childId = c.connecting?.piece?.id;
      if (!childId) continue;
      const p = d.pieces.find((x) => x.id === childId);
      if (p) out.push(p);
    }
    return out;
  }

  /**
   * @emoji 🧭 Sync flatten preview for MCP / `@semio/ui` (identity plane fallback until async WASM is threaded here).
   */
  flattenDesignCachedOp(
    designId: string,
    _prev?: { [pieceId: string]: FlatMerkleCacheEntry },
  ): { result: DesignOperationResult; cache: { [pieceId: string]: FlatMerkleCacheEntry } } {
    void _prev;
    const design = this.designs?.find((d) => d.id === designId);
    if (!design) {
      return {
        result: {
          ok: false,
          errors: [{ code: "mcp-flatten.design-not-found", message: `design ${designId} missing on kit` }],
        },
        cache: {},
      };
    }
    const defaultPlane = { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } };
    const conns = design.connections ?? [];
    const forward: DesignDiff = {
      pieces: {
        updated: (design.pieces ?? []).map((p) => ({
          piece: { id: p.id },
          diff: {
            plane: (p.plane?.toPlain() as unknown) ?? defaultPlane,
            center: p.center?.toPlain() ?? { u: 0, v: 0 },
          },
        })),
      },
      connections: conns.length ? { removed: conns.map((c) => ({ id: c.id })) } : undefined,
    };
    return { result: { ok: true, design, diff: { forward, reverse: {} } }, cache: {} };
  }
}
export type KitLike = Kit | KitFullDto;

// #region KitHostStores
/** @emoji 🧭 Client-side v7/UUID id for empty kit records when not using WASM. */
export function id(): string {
  if (typeof globalThis !== "undefined" && globalThis.crypto && typeof (globalThis.crypto as Crypto).randomUUID === "function") return (globalThis.crypto as Crypto).randomUUID()!;
  return `k-${Date.now()}-${((Math.random() * 0x1_0000_0000) | 0).toString(16)}`;
}

/** @emoji 🧭 DTO/entity to `Kit` (react / kit registry). */
export function asKitInstance(input: KitLike): Kit {
  return input instanceof Kit ? input : Kit.fromPlain(input as KitFullDto);
}

/**
 * @emoji 🧾 Pulls the authoritative DTO from `kitClient` into a host {@link KitHostStore} (no React; call after GQL events).
 */
/** @emoji 🧾 Minimal bridge surface used when applying WASM snapshots onto a host store. */
export async function applyKitClientSnapshotToLocalStore(kitClient: SemioKitBridge, store: KitHostStore): Promise<void> {
  try {
    const incoming = await kitClient.fetchFullKit();
    const curJson = store.getSnapshot().kit.toJSON();
    if (JSON.stringify(incoming) === JSON.stringify(curJson)) return;
    store.replace(asKitInstance(incoming));
  } catch {
    try {
      const incoming = await kitClient.fetchFullKit();
      store.replace(asKitInstance(incoming));
    } catch {
      /* ignore */
    }
  }
}

/** @emoji 🧭 Local/sync facet on every kit store snapshot (WASM or file-backed; hooks read `sync.readonly` etc). */
export type KitSyncSnapshot = { status: string; dirty: boolean; readonly: boolean; lastSyncedAt: string | null; error: unknown | null };
export const DEFAULT_KIT_SYNC: Readonly<KitSyncSnapshot> = Object.freeze({ status: "idle", dirty: false, readonly: false, lastSyncedAt: null, error: null });
export type KitHostStoreSnapshot = { kit: Kit; sync: KitSyncSnapshot };
export type KitHostStore = { getSnapshot(): KitHostStoreSnapshot; subscribe(onChange: () => void): () => void; replace(kit: Kit): void };

export class InMemoryKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal Used by `inferPersistenceFromInit` in @semio/react. */
  readonly name = "InMemoryKitStore";
  constructor(seed: KitLike) {
    this._kit = seed instanceof Kit ? seed : Kit.fromPlain(seed as KitFullDto);
  }
  getSnapshot(): KitHostStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) {
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

export type KitJsonFileAdapter = { read: () => Promise<string>; write: (json: string) => Promise<void> };
/** @emoji 🧾 Folder persistence adapter (Electron passes two path segments for `createDirectory`). */
export type KitFolderAdapter = {
  readKit: () => Promise<Uint8Array | undefined>;
  writeKit: (bytes: Uint8Array) => void | Promise<void>;
  readFile: (path: string) => Promise<Blob | undefined>;
  writeFile: (path: string, blob: Blob) => Promise<void>;
  deleteFile: (path: string) => Promise<void>;
  createDirectory: ((path: string) => Promise<void>) | ((folderPath: string, directoryPath: string) => Promise<void>);
  moveEntry: (from: string, to: string) => Promise<void>;
  listFiles: () => Promise<string[]>;
  watch?: (callback: () => void) => () => void;
};

export class JsonFileKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "JsonFileKitStore";
  private constructor(private readonly adapter: KitJsonFileAdapter, seed: Kit) { this._kit = seed; }
  static async create(adapter: KitJsonFileAdapter) {
    const json = await adapter.read();
    const seed = json.trim() === "" ? asKitInstance({ id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }) : Kit.fromPlain(JSON.parse(json) as KitFullDto);
    return new JsonFileKitStore(adapter, seed);
  }
  getSnapshot(): KitHostStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) {
    this._kit = kit;
    for (const l of this.listeners) l();
    void this.adapter.write(JSON.stringify(kit.toJSON()));
  }
}

export class FolderKitStore implements KitHostStore {
  private listeners = new Set<() => void>();
  private _kit: Kit;
  /** @internal */
  readonly name = "FolderKitStore";
  private constructor(private readonly adapter: KitFolderAdapter, seed: Kit) { this._kit = seed; }
  static async create(adapter: KitFolderAdapter, initial?: KitFullDto) {
    const bytes = await adapter.readKit();
    if (bytes != null && bytes.length > 0) {
      try {
        const t = new TextDecoder().decode(bytes);
        return new FolderKitStore(adapter, Kit.fromPlain(JSON.parse(t) as KitFullDto));
      } catch {
        /* fall through */
      }
    }
    return new FolderKitStore(adapter, asKitInstance(initial ?? { id: id(), name: "Untitled", createdAt: new Date().toISOString(), updatedAt: new Date().toISOString() }));
  }
  getSnapshot(): KitHostStoreSnapshot {
    return { kit: this._kit, sync: DEFAULT_KIT_SYNC };
  }
  subscribe(onChange: () => void) { this.listeners.add(onChange); return () => { this.listeners.delete(onChange); }; }
  replace(kit: Kit) {
    this._kit = kit;
    for (const l of this.listeners) l();
    void (async () => {
      try {
        const enc = new TextEncoder().encode(JSON.stringify(kit.toJSON()));
        await this.adapter.writeKit(enc);
      } catch {
        /* ignore */
      }
    })();
  }
}

export async function createJsonFileKitStore(adapter: KitJsonFileAdapter) { return await JsonFileKitStore.create(adapter); }
export async function createFolderKitStore(adapter: KitFolderAdapter, initial?: KitFullDto) { return await FolderKitStore.create(adapter, initial); }

export type SessionKitStoreConfig = { serverUrl: string; sessionId?: string; kitName?: string; personId?: string; clientId?: string; authToken?: string; readOnly?: boolean };
/** @emoji 🧭 Placeholder session store: in-memory until hub sync is host-wired. */
export async function createSessionKitStore(config: SessionKitStoreConfig) {
  const t = new Date().toISOString();
  const store = new InMemoryKitStore(asKitInstance({ id: id(), name: config.kitName ?? "Remote", createdAt: t, updatedAt: t, remote: config.serverUrl }));
  (store as InMemoryKitStore & { __semioSessionConfig?: SessionKitStoreConfig }).__semioSessionConfig = config;
  return store;
}
// #endregion KitHostStores

// #region KitFileHelpers
// @emoji 🧾 Transport-side kit file URLs, object URLs, and flattened kit ports (no domain diffs; mirrors kit JSON shape).

/**
 * @emoji 🧾 Upload/download surface used by `getKitFileProvider` / sketchpad `FileProvider` (aligned names, not re-exporting sketchpad).
 */
export type KitFileProvider = {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
};

/**
 * @emoji 🧾 Factory resolved once per opened kit; sketchpad sets this on `KitFileState`.
 */
export type KitFileProviderFactory = (kitId: string) => Promise<KitFileProvider>;

/**
 * @emoji 🧾 Per-`KitHostStore` blob/object URL and provider resolution cache (host-only; not serialized in kit).
 */
export type KitFileState = {
  objectUrls: Map<string, string>;
  providerUrls: Map<string, string>;
  blobs: Map<string, Blob>;
  pendingBlobDownloads: Map<string, Promise<string | null>>;
  providerFactory?: KitFileProviderFactory;
  /** @internal Last provider returned from {@link getKitFileProvider} for sync hooks. */
  _lastSyncProvider?: KitFileProvider;
  /** @internal */
  _cachedProviderByKitId?: Map<string, KitFileProvider>;
};

const kitFileStateByStore = new WeakMap<KitHostStore, KitFileState>();

function newKitFileState(): KitFileState {
  return { objectUrls: new Map(), providerUrls: new Map(), blobs: new Map(), pendingBlobDownloads: new Map() };
}

/** @emoji 🧾 Lazily created host cache keyed by the live `KitHostStore` (same identity as open kit). */
export function getOrCreateKitFileState(kitStore: KitHostStore): KitFileState {
  let st = kitFileStateByStore.get(kitStore);
  if (!st) {
    st = newKitFileState();
    kitFileStateByStore.set(kitStore, st);
  }
  return st;
}

const defaultKitFileProviderFactory: KitFileProviderFactory = async (kitId: string) => {
  const storage = new Map<string, Blob>();
  const key = (k: string, f: string, p: string) => `${k}/${f}/${p}`;
  return {
    upload: async (k, f, p, blob) => { storage.set(key(k, f, p), blob); return `memory://${key(k, f, p)}`; },
    download: async (k, f, p) => { const b = storage.get(key(k, f, p)); if (!b) throw new Error(`missing ${key(k, f, p)}`); return b; },
    delete: async (k, f, p) => { storage.delete(key(k, f, p)); },
    getUrl: (k, f, p) => `memory://${key(k, f, p)}`,
  };
};

/** @emoji 🧾 Async resolve + cache; warms {@link getExistingKitFileProvider} after first await. */
export async function getKitFileProvider(kitStore: KitHostStore, kitId: string): Promise<KitFileProvider> {
  const st = getOrCreateKitFileState(kitStore);
  st._cachedProviderByKitId = st._cachedProviderByKitId ?? new Map();
  const hit = st._cachedProviderByKitId.get(kitId);
  if (hit) { st._lastSyncProvider = hit; return hit; }
  const factory = st.providerFactory ?? defaultKitFileProviderFactory;
  const p = await factory(kitId);
  st._cachedProviderByKitId.set(kitId, p);
  st._lastSyncProvider = p;
  return p;
}

/** @emoji 🧾 Synchronous best-effort provider (after at least one {@link getKitFileProvider} call for this store). */
export function getExistingKitFileProvider(kitStore: KitHostStore): KitFileProvider | undefined {
  return getOrCreateKitFileState(kitStore)._lastSyncProvider;
}

/** @emoji 🧾 Relative path segment for sidecar / provider I/O (matches sketchpad memory layout `kitId/fileId/path`). */
export function getKitFileStoragePath(kit: Kit, file: { id: string }): string {
  void kit;
  return `files/${file.id}`;
}

export function isBrowserReadableFileUrl(u: string): boolean {
  return u.startsWith("blob:") || u.startsWith("data:") || u.startsWith("http://") || u.startsWith("https://");
}

/** @emoji 🧾 Prefer in-memory object URL, then embedded data/file URL fields. */
export function getReadableKitFileUrl(fileState: KitFileState, file: { id: string; url?: string; remote?: string }): string | null {
  const o = fileState.objectUrls.get(file.id);
  if (o) return o;
  const p = fileState.providerUrls.get(file.id);
  if (p && isBrowserReadableFileUrl(p)) return p;
  if (file.url && isBrowserReadableFileUrl(file.url)) return file.url;
  if (file.remote && isBrowserReadableFileUrl(file.remote)) return file.remote;
  return null;
}

/**
 * @emoji 🧾 Merged file-id → best readable URL for UI maps (`useKitStoredFileUrls`).
 */
export function getStoredKitFileUrls(kitStore: KitHostStore): Map<string, string> {
  const kit = kitStore.getSnapshot().kit;
  const st = getOrCreateKitFileState(kitStore);
  const out = new Map<string, string>();
  for (const f of kit.files ?? []) {
    const u = getReadableKitFileUrl(st, f);
    if (u) out.set(f.id, u);
  }
  for (const [k, v] of st.objectUrls) if (!out.has(k)) out.set(k, v);
  for (const [k, v] of st.providerUrls) if (!out.has(k) && isBrowserReadableFileUrl(v)) out.set(k, v);
  return out;
}

/** @emoji 🧾 Registers a `blob:` URL in {@link KitFileState.objectUrls} (revokes prior for same `fileId`). */
export function createKitFileObjectUrl(kitStore: KitHostStore, fileId: string, blob: Blob): string {
  const st = getOrCreateKitFileState(kitStore);
  const prev = st.objectUrls.get(fileId);
  if (prev) { try { URL.revokeObjectURL(prev); } catch { /* ignore */ } }
  const url = URL.createObjectURL(blob);
  st.objectUrls.set(fileId, url);
  return url;
}

export async function fetchReadableKitFileBlob(u: string): Promise<Blob | null> {
  try {
    const r = await fetch(u);
    if (!r.ok) return null;
    return await r.blob();
  } catch {
    return null;
  }
}

/**
 * @emoji 🧾 All ports defined on families (read-only helper for schema/UI).
 */
export function getKitPorts(kit: Kit): Port[] {
  const out: Port[] = [];
  for (const fam of kit.families ?? []) for (const p of fam.ports ?? []) out.push(p);
  return out;
}
// #endregion KitFileHelpers

// #region KitStoreBinaryFacet
export type KitBinaryStore = KitHostStore & {
  readFile?: (path: string) => Promise<Blob | null>;
  writeFile?: (path: string, blob: Blob) => Promise<void>;
  deleteFile?: (path: string) => Promise<void>;
  createDirectory?: (path: string) => Promise<void>;
  moveEntry?: (from: string, to: string) => Promise<void>;
};
// #endregion KitStoreBinaryFacet

export const KitDiffSchema = z.object({ types: TypesDiffSchema.optional(), designs: DesignsDiffSchema.optional() }).passthrough();
export type KitDiff = ReadonlyDto<z.infer<typeof KitDiffSchema>>;
// #endregion Kit

// #region KitImportHelpers
/** @emoji 🧾 Decode kit bytes as JSON DTO (host handles archives before calling). */
export function importKitToPlain(buf: ArrayBuffer | Uint8Array): KitFullDto {
  const u8 = buf instanceof Uint8Array ? buf : new Uint8Array(buf);
  const text = new TextDecoder().decode(u8);
  return KitFullDtoSchema.parse(JSON.parse(text));
}
// #endregion KitImportHelpers

// #region EntityKitStores
/** @emoji 🧭 Arbitrary kit entity handle: patch fields and subscribe to rs {@link KitEvent} stream. */
export class KitEntityStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly entityKind: string,
    public readonly id: string,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  async patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    void field;
    void value;
    return Promise.resolve({
      ok: false,
      error: { kind: "NotSupported", message: "use typed KitStore.submitChangeKitCommands or entity store methods" },
    });
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if ("Changed" in ev && (ev as { Changed?: null }).Changed === null) {
        this._version += 1;
        handler(ev);
        return;
      }
      const hi = (ev as { HashInvalidated?: { entity?: { kind?: string; id?: string } } }).HashInvalidated;
      if (hi?.entity?.id === this.id && hi.entity.kind === this.entityKind) {
        this._version += 1;
        handler(ev);
        return;
      }
      if (jsonSubtreeHasIdKey(ev, `${this.entityKind.charAt(0).toLowerCase() + this.entityKind.slice(1)}_id`, this.id)) {
        this._version += 1;
        handler(ev);
      }
    });
  }
}

/** @emoji 🧭 Per-design kit handle: GraphQL reads and semantic design mutations on {@link KitStore}. */
export class DesignStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesDesign(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async metadata(): Promise<DesignMetadataDto> {
    const out = await this.root.read(this.readScope, [{ readKitDesignsMetadataCommand: null }]);
    const designs = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitDesignsMetadataCommand?: { designs?: JsonValue } }).readKitDesignsMetadataCommand?.designs,
    );
    const row = designs.find((d: unknown) => d && typeof d === "object" && String((d as { id?: string }).id) === this.id);
    if (!row) throw new Error(`design metadata not found: ${this.id}`);
    return DesignMetadataDtoSchema.parse(row);
  }

  async shallow(): Promise<DesignShallow> {
    const out = await this.root.read(this.readScope, [{ readKitDesignsShallowCommand: null }]);
    const designs = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitDesignsShallowCommand?: { designs?: JsonValue } }).readKitDesignsShallowCommand?.designs,
    );
    const row = designs.find((d: unknown) => d && typeof d === "object" && String((d as { id?: string }).id) === this.id);
    if (!row) throw new Error(`design shallow not found: ${this.id}`);
    return DesignShallowSchema.parse(row);
  }

  /** @emoji 🧾 Full design DTO from a kit snapshot (rs materialized truth). */
  async full(): Promise<DesignPlain> {
    const kit = (await this.root.materializedLiveJsonForReadScope(this.readScope)) as KitFullDto;
    const raw = (kit.designs ?? []).find((d) => d.id === this.id);
    if (!raw) throw new Error(`design not found: ${this.id}`);
    return DesignSchema.parse(raw);
  }

  async pieces(): Promise<readonly PieceStore[]> {
    const rows = await this.root.getPieces(this.readScope, this.id);
    return rows.map((p) => this.root.piece(this.id, String(p.id), this.readScope));
  }

  piece(pieceId: string): PieceStore {
    return this.root.piece(this.id, pieceId, this.readScope);
  }

  async connections(): Promise<readonly ConnectionStore[]> {
    const rows = await this.root.getConnections(this.readScope, this.id);
    return rows.map((c) => this.root.connection(this.id, String(c.id), this.readScope));
  }

  connection(connectionId: string): ConnectionStore {
    return this.root.connection(this.id, connectionId, this.readScope);
  }

  /** @emoji 🧾 Live design graph reads routed like {@link LiveDesign}. */
  private liveDesign(): LiveDesign {
    return new LiveDesign(this.root, this.readScope, this.id);
  }

  readIncludedDesigns(): Promise<readonly IncludedDesignInfoDto[]> {
    return this.liveDesign().readIncludedDesigns();
  }

  readClusterableGroups(selection: readonly string[]): Promise<readonly (readonly KitIdDto[])[]> {
    return this.liveDesign().readClusterableGroups(selection);
  }

  readQualitySum(qualityId: string): Promise<number> {
    return this.liveDesign().readQualitySum(qualityId);
  }

  readReplaceableCatalogTypes(selection: readonly string[]): Promise<readonly string[]> {
    return this.liveDesign().readReplaceableCatalog(selection).then((v) => v.types);
  }

  readReplaceableCatalogDesigns(selection: readonly string[]): Promise<readonly string[]> {
    return this.liveDesign().readReplaceableCatalog(selection).then((v) => v.designs);
  }

  readIncludedDesignIds(): Promise<readonly string[]> {
    return this.liveDesign().readIncludedDesignIds().then((v) => (Array.isArray(v) ? v : []));
  }

  /** @emoji 🧾 Per-piece placement metadata rows (`getPiecesMetadata`). */
  readPiecesPlacementMetadataMap(): Promise<ReadonlyMap<string, PiecePlacementRowDto>> {
    return this.root.getPiecesMetadata(this.readScope, this.id);
  }

  /** @emoji 🧾 Full piece DTO rows for this design (`getPieces`). */
  readPiecesFullRows(): Promise<readonly PiecePlain[]> {
    return this.root.getPieces(this.readScope, this.id);
  }

  /** @emoji 🧾 Full connection DTO rows for this design (`getConnections`). */
  readConnectionsFullRows(): Promise<readonly ConnectionPlain[]> {
    return this.root.getConnections(this.readScope, this.id);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      { changeDesignCommands: { designId: { id: this.id }, commands: [{ name: { name } }] } },
    ]);
  }

  cluster(pieceIds: readonly string[], name: string): Promise<SetResult> {
    return this.root.clusterPieces(this.id, pieceIds, name);
  }

  drag(pieceIds: readonly string[], du: number, dv: number): Promise<SetResult> {
    return this.root.dragPieces(this.id, pieceIds, du, dv);
  }

  move(pieceIds: readonly string[], gap: number, shift: number, rise: number): Promise<SetResult> {
    return this.root.movePieces(this.id, pieceIds, gap, shift, rise);
  }

  fix(pieceIds: readonly string[]): Promise<SetResult> {
    return this.root.fixPieces(this.id, pieceIds);
  }

  flatten(): Promise<SetResult> {
    return this.root.flattenDesign(this.id);
  }

  expand(nestedDesignId: string): Promise<SetResult> {
    return this.root.expandDesign(this.id, nestedDesignId);
  }

  paste(selection: KitJsonTreeDto, plane?: PlanePlain | null): Promise<SetResult> {
    return this.root.pasteDesignSelection(this.id, selection, plane ?? null);
  }

  createHangingPieces(typeIds: readonly string[], plane: PlanePlain): Promise<SetResult> {
    return this.root.createHangingPieces(this.id, typeIds, plane);
  }

  createConnectedPiece(parentPiece: string, parentPort: string, childType: string, childPort: string): Promise<SetResult> {
    return this.root.createConnectedPiece(this.id, parentPiece, parentPort, childType, childPort);
  }

  createFixedPiece(typeId: string, plane: PlanePlain): Promise<SetResult> {
    return this.root.createFixedPiece(this.id, typeId, plane);
  }

  addPiece(dto: PiecePlain): Promise<SetResult> {
    const piece = PieceSchema.parse(dto);
    return this.root.submitChangeKitCommands([
      { changeDesignCommands: { designId: { id: this.id }, commands: [{ addPiece: { piece } }] } },
    ]);
  }

  removePiece(pieceId: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      { changeDesignCommands: { designId: { id: this.id }, commands: [{ removePiece: { pieceId: { id: pieceId } } }] } },
    ]);
  }
}

/** @emoji 🧾 GraphQL `TypeMetadataObject` uses JSON `null` for absent fields; strip those before Zod DTO parse. */
function __coerceTypeMetadataGqlRow(row: JsonObject): JsonObject {
  const out = { ...row };
  for (const k of Object.keys(out)) {
    if (out[k] === null) delete out[k];
  }
  return out;
}

/** @emoji 🧭 Per-kind kit handle (semio domain kind, not TS typeof). */
export class TypeStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesType(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async metadata(): Promise<TypeMetadataDto> {
    const out = await this.root.read(this.readScope, [{ readKitTypesMetadataCommand: null }]);
    const types = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitTypesMetadataCommand?: { types?: JsonValue } }).readKitTypesMetadataCommand?.types,
    );
    const row = types.find((t: unknown) => t && typeof t === "object" && String((t as { id?: string }).id) === this.id);
    if (!row) throw new Error(`kind metadata not found: ${this.id}`);
    return TypeMetadataDtoSchema.parse(__coerceTypeMetadataGqlRow(row as JsonObject));
  }

  async shallow(): Promise<TypeShallow> {
    const out = await this.root.read(this.readScope, [{ readKitTypesShallowCommand: null }]);
    const types = kitGraphqlJsonToReadonlyArray(
      (out[0] as { readKitTypesShallowCommand?: { types?: JsonValue } }).readKitTypesShallowCommand?.types,
    );
    const row = types.find((t: unknown) => t && typeof t === "object" && String((t as { id?: string }).id) === this.id);
    if (!row) throw new Error(`kind shallow not found: ${this.id}`);
    return TypeShallowSchema.parse(row);
  }

  async full(): Promise<TypePlain> {
    const kit = (await this.root.materializedLiveJsonForReadScope(this.readScope)) as KitFullDto;
    const raw = (kit.types ?? []).find((t) => t.id === this.id);
    if (!raw) throw new Error(`kind not found: ${this.id}`);
    return TypeSchema.parse(raw);
  }

  /** @emoji 🧾 Best representation for tag ids (`readTypeBestRepresentationCommand`). */
  readBestRepresentation(tagIds: readonly string[]): Promise<RepresentationPlain | null> {
    return new LiveType(this.root, this.readScope, this.id).readBestRepresentation(tagIds);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      { changeTypeCommands: { typeId: { id: this.id }, commands: [{ name: { name } }] } },
    ]);
  }

  addRepresentation(dto: unknown): Promise<SetResult> {
    const representation = RepresentationSchema.parse(dto);
    return this.root.submitChangeKitCommands([
      { changeTypeCommands: { typeId: { id: this.id }, commands: [{ addRepresentation: { representation } }] } },
    ]);
  }

  addConnector(dto: unknown): Promise<SetResult> {
    const connector = ConnectorSchema.parse(dto);
    return this.root.submitChangeKitCommands([
      { changeTypeCommands: { typeId: { id: this.id }, commands: [{ addConnector: { connector } }] } },
    ]);
  }

  addProp(dto: unknown): Promise<SetResult> {
    const prop = PropSchema.parse(dto);
    return this.root.submitChangeKitCommands([
      { changeTypeCommands: { typeId: { id: this.id }, commands: [{ addTypeProp: { prop } }] } },
    ]);
  }

  removeChild(childKind: string, childId: string): Promise<SetResult> {
    if (childKind === "Representation") {
      return this.root.submitChangeKitCommands([
        { changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeRepresentation: { id: { id: childId } } }] } },
      ]);
    }
    if (childKind === "Connector") {
      return this.root.submitChangeKitCommands([
        { changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeConnector: { connectorId: { id: childId } } }] } },
      ]);
    }
    if (childKind === "Prop") {
      return this.root.submitChangeKitCommands([
        { changeTypeCommands: { typeId: { id: this.id }, commands: [{ removeTypeProp: { propId: { id: childId } } }] } },
      ]);
    }
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `removeChild: ${childKind}` } });
  }
}

/** @emoji 🧭 Piece scoped to one design id plus piece id. */
export class PieceStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly designId: string,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesPiece(ev, this.designId, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  /** @emoji 🧾 Flattened placement plane in world space (`readPieceFlatPlaneCommand`). */
  readFlatPlane(): Promise<PlanePlain | null> {
    return new LivePiece(this.root, this.readScope, this.designId, this.id).readFlatPlane();
  }

  /** @emoji 🧾 Flattened placement center (`readPieceFlatCenterCommand`). */
  readFlatCenter(): Promise<CoordinatePlain | null> {
    return new LivePiece(this.root, this.readScope, this.designId, this.id).readFlatCenter();
  }

  /** @emoji 🧾 Parent connection row when connected (`readPieceParentConnectionFullCommand`). */
  readParentConnectionFull(): Promise<ConnectionPlain | null> {
    return new LivePiece(this.root, this.readScope, this.designId, this.id).readParentConnectionFull();
  }

  async full(): Promise<PiecePlain> {
    const pieces = await this.root.getPieces(this.readScope, this.designId);
    const row = pieces.find((p) => String(p.id) === this.id);
    if (!row) throw new Error(`piece not found: ${this.id}`);
    return PieceSchema.parse(row);
  }

  setPlane(plane: PlanePlain): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      kitChangeDesignPiece(this.designId, this.id, [{ plane: { plane: plane as KitJsonTreeDto } }]),
    ]);
  }

  setCenter(center: CoordinatePlain): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      kitChangeDesignPiece(this.designId, this.id, [{ center: { center: center as KitJsonTreeDto } }]),
    ]);
  }

  setScale(scale: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ scale: { scale } }])]);
  }

  setColor(color: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ color: { color } }])]);
  }

  hide(isHidden: boolean): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ hidden: { hidden: isHidden } }])]);
  }

  lock(isLocked: boolean): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ locked: { locked: isLocked } }])]);
  }

  addProp(dto: unknown): Promise<SetResult> {
    const prop = PropSchema.parse(dto);
    return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ addProp: { prop } }])]);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "name") return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ name: { name: String(value) } }])]);
    if (field === "description")
      return this.root.submitChangeKitCommands([kitChangeDesignPiece(this.designId, this.id, [{ description: { description: value as string | null } }])]);
    if (field === "type" || field === "typeId")
      return this.root.submitChangeKitCommands([
        kitChangeDesignPiece(this.designId, this.id, [{ type: { typeId: value && typeof value === "object" && "id" in (value as object) ? (value as { id: string }) : { id: String(value) } } }]),
      ]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `piece field: ${field}` } });
  }
}

/** @emoji 🧭 Connection scoped to one design id plus connection id. */
export class ConnectionStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly designId: string,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesConnection(ev, this.designId, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<ConnectionPlain> {
    const connections = await this.root.getConnections(this.readScope, this.designId);
    const row = connections.find((c: unknown) => c && typeof c === "object" && String((c as { id?: string }).id) === this.id);
    if (!row) throw new Error(`connection not found: ${this.id}`);
    return ConnectionSchema.parse(row);
  }

  setGap(gap: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ gap: { value: gap } }])]);
  }

  setShift(shift: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ shift: { value: shift } }])]);
  }

  setRotation(rotation: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ rotation: { value: rotation } }])]);
  }

  setTilt(tilt: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ tilt: { value: tilt } }])]);
  }

  setTurn(turn: number): Promise<SetResult> {
    return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ turn: { value: turn } }])]);
  }

  delete(): Promise<SetResult> {
    return this.root.deleteConnection(this.designId, this.id);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "rise") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ rise: { value: Number(value) } }])]);
    if (field === "description")
      return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ description: { value: value as string | null } }])]);
    if (field === "u" || field === "x") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ x: { value: Number(value) } }])]);
    if (field === "v" || field === "y") return this.root.submitChangeKitCommands([kitChangeDesignConnection(this.designId, this.id, [{ y: { value: Number(value) } }])]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `connection field: ${field}` } });
  }
}

/** @emoji 🧭 Kit family row (ports live under families in rs). */
export class FamilyStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFamily(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FamilyPlain> {
    const kit = (await this.root.materializedLiveJsonForReadScope(this.readScope)) as KitFullDto;
    const raw = (kit.families ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`family not found: ${this.id}`);
    return FamilySchema.parse(raw);
  }

  setName(name: string): Promise<SetResult> {
    return this.root.submitChangeKitCommands([
      { changeFamilyCommands: { familyId: { id: this.id }, commands: [{ name: { name } }] } },
    ]);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    if (field === "description")
      return this.root.submitChangeKitCommands([
        { changeFamilyCommands: { familyId: { id: this.id }, commands: [{ description: { description: value as string | null } }] } },
      ]);
    if (field === "icon")
      return this.root.submitChangeKitCommands([
        { changeFamilyCommands: { familyId: { id: this.id }, commands: [{ icon: { icon: value as string | null } }] } },
      ]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `family field: ${field}` } });
  }
}

/** @emoji 🧭 Kit file blob row. */
export class FileStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFile(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FilePlain> {
    const kit = (await this.root.materializedLiveJsonForReadScope(this.readScope)) as KitFullDto;
    const raw = (kit.files ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`file not found: ${this.id}`);
    return FileSchema.parse(raw);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    const fid = { id: this.id };
    if (field === "url") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ url: { url: String(value) } }] } }]);
    if (field === "mime") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ mime: { mime: value as string | null } }] } }]);
    if (field === "size") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ size: { size: value as number | null } }] } }]);
    if (field === "hash") return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ hash: { hash: value as string | null } }] } }]);
    if (field === "description")
      return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ description: { description: value as string | null } }] } }]);
    if (field === "created" || field === "createdAt")
      return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ created: { created: value as string | null } }] } }]);
    if (field === "updated" || field === "updatedAt")
      return this.root.submitChangeKitCommands([{ changeFileCommands: { fileId: fid, commands: [{ updated: { updated: value as string | null } }] } }]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `file field: ${field}` } });
  }
}

/** @emoji 🧭 Kit folder row. */
export class FolderStore {
  private _version = 0;
  constructor(
    public readonly root: KitStore,
    public readonly id: string,
    public readonly readScope: KitReadScope = theKitReadScope,
  ) {}

  get readVersion(): number {
    return this._version;
  }

  subscribe(handler: (e: KitEvent) => void): Unsubscribe {
    return this.root.subscribe((ev) => {
      if (!kitEventTouchesFolder(ev, this.id)) return;
      this._version += 1;
      handler(ev);
    });
  }

  async full(): Promise<FolderPlain> {
    const kit = (await this.root.materializedLiveJsonForReadScope(this.readScope)) as KitFullDto;
    const raw = (kit.folders ?? []).find((f) => f.id === this.id);
    if (!raw) throw new Error(`folder not found: ${this.id}`);
    return FolderSchema.parse(raw);
  }

  patchField(field: string, value: SchemaEntityFieldValue): Promise<SetResult> {
    const folderId = { id: this.id };
    if (field === "path") return this.root.submitChangeKitCommands([{ changeFolderCommands: { folderId, commands: [{ path: { path: String(value) } }] } }]);
    if (field === "description")
      return this.root.submitChangeKitCommands([{ changeFolderCommands: { folderId, commands: [{ description: { description: value as string | null } }] } }]);
    return Promise.resolve({ ok: false, error: { kind: "NotSupported", message: `folder field: ${field}` } });
  }
}
// #endregion EntityKitStores
// #endregion 🧩KitEntitiesMerged

// #endregion 🧩KitWasmBridgeMerged

// #region 🧪EmbeddedTests
if (process.env["SEMIO_JS_RUN_EMBEDDED_TESTS"] === "1") {
  const { describe, it, expect } = await import("vitest");

  describe("semio-js KitStore", () => {
    it("KIT_SCOPED_FULL_DTO_QUERY wires Query.kit(scope) { fullDto }", () => {
      expect(KIT_SCOPED_FULL_DTO_QUERY).toContain("kit(scope:");
      expect(KIT_SCOPED_FULL_DTO_QUERY).toMatch(/fullDto/);
    });

    it("KitStore has no JS snapshot() full-read method (use theKit / materialized GraphQL only)", () => {
      type Snap = { snapshot?: () => unknown };
      const snap: Snap = KitStore.prototype as unknown as Snap;
      expect(snap.snapshot).toBeUndefined();
    });

    it("opens dedicated worker wasm and returns typed full kit DTO from GraphQL", async () => {
      const minimalKit: KitFullDto = {
        id: "test-kit",
        name: "TestKit",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "type-1", name: "Wall", connectors: [] }],
        designs: [{ id: "design-1", name: "Floor1", pieces: [], connections: [] }],
      };
      const ks = await KitStore.open(minimalKit);
      const snap = await ks.theKit();
      expect(snap.id).toBe("test-kit");
      expect(snap.name).toBe("TestKit");
      const typeStores = await ks.types();
      expect(typeStores.map((t) => t.id)).toEqual(["type-1"]);
      const designStores = await ks.designs();
      expect(designStores.map((d) => d.id)).toEqual(["design-1"]);
      const r = await ks.type("type-1").setName("BigWall");
      expect(typeof r.ok).toBe("boolean");
      const meta = await ks.type("type-1").metadata();
      expect(meta.id).toBe("type-1");
      expect(meta.name).toBe("Wall");
      await ks.dispose();
    });

    it("kitReadScopeKey normalizes the main line scope for cache keys", () => {
      expect(kitReadScopeKey(theKitReadScope)).toBe(JSON.stringify(kitReadScopeToGraphQLInput(theKitReadScope)));
    });

    it("kitChangeSemanticKindToGraphQl maps GraphQL enum + other label", () => {
      expect(kitChangeSemanticKindToGraphQl("ADD_PIECE", null)).toBe("addPiece");
      expect(kitChangeSemanticKindToGraphQl("OTHER", "addFamily")).toEqual({ other: "addFamily" });
      expect(kitChangeSemanticKindToGraphQl("OTHER", null)).toBe("inferred");
    });

    it("normalizeKitEventFromSubscription passes flat classified mutation rows", () => {
      const raw = {
        renamedDesign: {
          designId: "d1",
          change: { forward: [] as const, inverse: [] as const, kind: "modifyDesign" as const },
        },
      };
      const out = normalizeKitEventFromSubscription(raw);
      expect(out).toBeDefined();
      expect(isKitClassifiedMutationEvent(out!)).toBe(true);
      if (isKitClassifiedMutationEvent(out!)) {
        expect("renamedDesign" in out && out.renamedDesign.designId).toBe("d1");
      }
    });

    it("kitEventTouchesDesignStrict matches renamedDesign kit events", () => {
      const ev = {
        renamedDesign: { designId: "dx", change: { forward: [], inverse: [] } },
      } as const satisfies KitClassifiedMutationEvent;
      expect(kitEventTouchesDesignStrict(ev, "dx")).toBe(true);
      expect(kitEventTouchesDesignStrict(ev, "other")).toBe(false);
    });

    it("read batch returns typed rows", async () => {
      const minimalKit: KitFullDto = {
        id: "read-kit",
        name: "R",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      const batch: ReadBatch = [{ readKitTypesShallowCommand: null }, { readKitTypeIdsCommand: null }];
      const res = await ks.read(theKitReadScope, batch);
      expect(res.length).toBe(2);
      await ks.dispose();
    });

    it("designRowIds and kindRowIds align with design() and type() factory lists", async () => {
      const minimalKit: KitFullDto = {
        id: "row-ids-kit",
        name: "R",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "ta", name: "A", connectors: [] }],
        designs: [{ id: "da", name: "D", pieces: [], connections: [] }],
      };
      const ks = await KitStore.open(minimalKit);
      expect(await ks.designRowIds()).toEqual((await ks.designs()).map((d) => d.id));
      expect(await ks.kindRowIds()).toEqual((await ks.types()).map((t) => t.id));
      await ks.dispose();
    });

    it("PieceStore readFlatPlane is defined on the owning store (delegates to live read wire)", async () => {
      const minimalKit: KitFullDto = {
        id: "piece-flat-kit",
        name: "P",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "t1", name: "T", connectors: [] }],
        designs: [
          {
            id: "d1",
            name: "D",
            pieces: [
              {
                id: "p1",
                name: "Piece1",
                type: { id: "t1" },
                plane: { origin: { x: 0, y: 0, z: 0 }, xAxis: { x: 1, y: 0, z: 0 }, yAxis: { x: 0, y: 1, z: 0 } },
                center: { u: 0, v: 0 },
                scale: 1,
                color: "#000000",
                props: [],
                attributes: [],
              },
            ],
            connections: [],
          },
        ],
      };
      const ks = await KitStore.open(minimalKit);
      expect(typeof ks.piece("d1", "p1").readFlatPlane).toBe("function");
      expect(typeof ks.design("d1").readClusterableGroups).toBe("function");
      expect(typeof ks.type("t1").readBestRepresentation).toBe("function");
      await ks.dispose();
    });

    it("rejects theKit() after dispose", async () => {
      const minimalKit: KitFullDto = {
        id: "dispose-kit",
        name: "D",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      await ks.dispose();
      await expect(ks.theKit()).rejects.toThrow(/disposed/i);
    });

    it("subscribe returns Unsubscribe and does not expose events$", async () => {
      const minimalKit: KitFullDto = {
        id: "sub-kit",
        name: "S",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      let n = 0;
      const off = ks.subscribe(() => {
        n += 1;
      });
      expect(typeof off).toBe("function");
      off();
      await ks.dispose();
      type KitStorePublicKeys = keyof KitStore;
      type MustNotIncludeEvents = "events$" extends KitStorePublicKeys ? never : true;
      const _compileAssert: MustNotIncludeEvents = true;
      expect(_compileAssert).toBe(true);
      expect(n).toBeGreaterThanOrEqual(0);
    });

    it("theKit, vcsState, materializeAt root, and undo/redo flags round-trip", async () => {
      const minimalKit: KitFullDto = {
        id: "vcs-kit",
        name: "V",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      const snap = await ks.theKit();
      const snap2 = await ks.theKit();
      expect(snap2.id).toBe(snap.id);
      const vcs = await ks.vcsState();
      expect(vcs != null && typeof vcs === "object").toBe(true);
      const mat = await ks.materializeAt("");
      expect(mat.id).toBe(snap.id);
      expect(typeof (await ks.canUndo())).toBe("boolean");
      expect(typeof (await ks.canRedo())).toBe("boolean");
      await ks.dispose();
    });

    it("compile-time: KitStore public surface excludes rxjs-style stream fields", () => {
      type KitStorePublicKeys = keyof KitStore;
      type MustNotLeakRx =
        "events$" extends KitStorePublicKeys
          ? never
          : "pipe" extends KitStorePublicKeys
            ? never
            : "_trySubscribe" extends KitStorePublicKeys
              ? never
              : true;
      const _assert: MustNotLeakRx = true;
      expect(_assert).toBe(true);
    });
  });

  describe("semio-js GraphQL wire contract", () => {
    it("KIT_STORE_BATCH_MUTATION and KIT_EVENT_STREAM_SUBSCRIPTION align with schema.graphql", async () => {
      const { readFileSync } = await import("node:fs");
      const { resolve, dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      let sdl = "";
      const here = dirname(fileURLToPath(import.meta.url));
      for (const p of [resolve(here, "../graphql/schema.graphql"), resolve(process.cwd(), "semio/graphql/schema.graphql")]) {
        try {
          sdl = readFileSync(p, "utf8");
          if (sdl.length > 100) break;
        } catch {
          /* try next */
        }
      }
      expect(sdl.length).toBeGreaterThan(100);
      expect(sdl).toContain("type KitStoreMutation");
      expect(sdl).toContain("batch(input: KitStoreBatchInput!");
      expect(sdl).toContain("type KitStoreBatchResult");
      expect(sdl).toMatch(/eventStream\s*[:(]/);
      expect(KIT_STORE_BATCH_MUTATION).toContain("mutation($input: KitStoreBatchInput!)");
      expect(KIT_STORE_BATCH_MUTATION).toContain("kitStore { batch(input: $input)");
      expect(KIT_STORE_BATCH_MUTATION).toContain("results { kind");
      expect(KIT_EVENT_STREAM_SUBSCRIPTION).toBe("subscription { eventStream }");
    });
  });

  describe("semio-js kit event entity filters", () => {
    it("kitEventTouchesDesignStrict matches nested Design payload", () => {
      const ev = { Design: { design_id: "d1", event: { Piece: { piece_id: "p1", event: "Changed" } } } } as KitEvent;
      expect(kitEventTouchesDesignStrict(ev, "d1")).toBe(true);
      expect(kitEventTouchesDesignStrict(ev, "d2")).toBe(false);
    });

    it("kitEventTouchesPiece ignores bare Changed", () => {
      expect(kitEventTouchesPiece({ Changed: null } as KitEvent, "d1", "p1")).toBe(false);
    });

    it("kitEventTouchesPiece matches FlattenInvalidated piece list", () => {
      const ev = { FlattenInvalidated: { design: "d1", pieces: ["p1"] } } as KitEvent;
      expect(kitEventTouchesPiece(ev, "d1", "p1")).toBe(true);
      expect(kitEventTouchesPiece(ev, "d1", "p2")).toBe(false);
    });

    it("kitEventTouchesDesign matches rs-shaped design name field change", () => {
      const ev = { Design: { design_id: "design-a", event: { FieldChanged: "Name" } } } as KitEvent;
      expect(kitEventTouchesDesign(ev, "design-a")).toBe(true);
      expect(kitEventTouchesDesign(ev, "other")).toBe(false);
    });

    it("kitEventTouchesTypeStrict matches Type payload", () => {
      const ev = { Type: { type_id: "t1", event: "Changed" } } as KitEvent;
      expect(kitEventTouchesTypeStrict(ev, "t1")).toBe(true);
      expect(kitEventTouchesTypeStrict(ev, "t2")).toBe(false);
    });
  });

  describe("semio-js entity stores", () => {
    it("TypeStore metadata and shallow read paths resolve", async () => {
      const minimalKit: KitFullDto = {
        id: "meta-type-kit",
        name: "K",
        createdAt: "2020-01-01T00:00:00.000Z",
        updatedAt: "2020-01-01T00:00:00.000Z",
        types: [{ id: "type-z", name: "Zed", connectors: [] }],
        designs: [],
      };
      const ks = await KitStore.open(minimalKit);
      const t = ks.type("type-z");
      const meta = await t.metadata();
      expect(meta.id).toBe("type-z");
      expect(meta.name).toBe("Zed");
      const sh = await t.shallow();
      expect(sh.id).toBe("type-z");
      await ks.dispose();
    });
  });

  describe("semio-js kit store wire helpers", () => {
    it("piecePatchToChangeCommands maps plane and type ref", () => {
      const cmds = piecePatchToChangeCommands({ plane: { x: 1 }, type: { id: "t1" } });
      expect(cmds.length).toBe(2);
      expect(cmds.some((c) => "plane" in c)).toBe(true);
      expect(cmds.some((c) => "type" in c && (c as { type: { typeId: { id: string } } }).type.typeId.id === "t1")).toBe(true);
    });

    it("connectionDiffKeyForDataKey maps u to x", () => {
      expect(connectionDiffKeyForDataKey("u")).toBe("x");
      expect(connectionDiffKeyForDataKey("gap")).toBe("gap");
    });

    it("buildSchemaEntityChangeCommands returns nested piece wire with design id", () => {
      const cmds = buildSchemaEntityChangeCommands("Piece", "p1", "color", "#fff", "d1");
      expect(cmds.length).toBe(1);
      expect(cmds[0]).toMatchObject({
        changeDesignCommands: { designId: { id: "d1" } },
      });
    });

    it("kitStoreClientUpdatePiece forwards to submitChangeKitCommands", async () => {
      let last: readonly ChangeKitCommand[] | undefined;
      const client = {
        getKitWriteScope: () => null,
        setKitWriteScope: () => {},
        finalizeKitWriteTransaction: async () => ({ ok: true as const }),
        abortKitWriteTransaction: async () => ({ ok: true as const }),
        submitChangeKitCommands: async (cs: readonly ChangeKitCommand[]) => {
          last = cs;
          return { ok: true as const };
        },
      } as unknown as KitStoreClient;
      await kitStoreClientUpdatePiece(client, "d1", "p1", { name: "N" });
      expect(last?.length).toBe(1);
    });
  });
}
// #endregion 🧪EmbeddedTests
