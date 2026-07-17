// #region Header
/** @emoji 🪞 `@semio-tech/vcs-core` — hand-written wire-compatible TypeScript mirror of `vcs/rs`
 * (`vcs::DocumentVcsStore`). `vcs/rs/pkg/vcs.d.ts` (wasm-bindgen output) exports nothing but `init`,
 * so this is NOT a wrapper around the wasm package — every shape below is typed by hand from
 * `vcs/rs/lib.rs`, kept field-for-field camelCase-identical to its `#[serde(rename_all = "camelCase")]`
 * structs so a stored envelope round-trips byte-identical between the two runtimes. Backbone/persistence
 * (`Backbone`, `pump`/`flushOutbound`, `folder://`/`file://`/`remote://` IO) is out of scope here — that
 * lives in `framework/sync`'s actor layer on the Rust side and has no TS mirror yet. */
// #endregion Header

// #region 🔌Adapters
import type { Hasher } from "./hasher.ts";
import { createDefaultHasher } from "./hasher.ts";
// #endregion 🔌Adapters

//#region 🔖Ids
let idCounter = 0;

/** @emoji 🆔 Allocates stable ids for document VCS entities. Mirrors `vcs::create_document_vcs_id`
 * (a process-wide counter) — collisions only matter across a single store's lifetime here. */
export function createDocumentVcsId(prefix: string): string {
  idCounter += 1;
  return `${prefix}-${idCounter}`;
}

/** @emoji 🕰️ Mirrors `vcs::now_iso` — despite the name, this is a decimal ms timestamp string, not
 * RFC3339; wire-compatibility requires reproducing that quirk rather than "fixing" it. */
function nowIso(): string {
  return String(Date.now());
}
//#endregion 🔖Ids

//#region 🔖Schemas
/** @emoji ⏱️ Mirrors `framework/core/rs`'s `HybridLogicalTimestamp`. */
export type HybridLogicalTimestamp = {
  readonly actor: number;
  readonly physicalMs: number;
  readonly logical: number;
};

/** @emoji ↩️ Mirrors `framework/core/rs`'s `UndoPolicy` (`#[serde(rename_all = "camelCase")]`). */
export type UndoPolicy = "exactBaseOnly" | "transformAgainstConcurrent" | "semanticUndo" | "compensatingAction";

export type DocumentBackboneRef = { readonly uri: string };

export type Author = {
  readonly id: string;
  readonly name: string;
  readonly avatar?: string;
};

export type OperationMeta = {
  readonly operationId: string;
  readonly dependencies?: readonly string[];
  readonly baseVersion: number;
  readonly authorId: string;
  readonly timestamp: HybridLogicalTimestamp;
  readonly undoPolicy: UndoPolicy;
  readonly payloadHash?: string;
};

export type Edit<Op> = {
  id: string;
  actor?: string;
  forwards: Op[];
  backwards: Op[];
  operationMeta: OperationMeta[];
  description?: string;
  coalesceKey?: string;
  sequenceNumber: number;
  startedAt: string;
  finishedAt?: string;
};

export type Change = {
  readonly id: string;
  readonly editIds: string[];
  readonly description?: string;
  readonly savedAt: string;
};

export type Checkpoint = {
  readonly id: string;
  readonly changeIds: string[];
  readonly parentId?: string;
  readonly authors: Author[];
  readonly message?: string;
  readonly timestamp: string;
};

export type Alternative = {
  readonly id: string;
  readonly name: string;
  checkpointIds: string[];
};

export type DocumentVcs<P, Op> = {
  initialProjection: P;
  edits: Edit<Op>[];
  changes: Change[];
  checkpoints: Checkpoint[];
  alternatives: Alternative[];
};

export type DocumentVcsEnvelope<P, Op> = {
  readonly schema: string;
  readonly id: string;
  vcs: DocumentVcs<P, Op>;
  backbone?: DocumentBackboneRef;
  activeAlternativeId?: string;
};

/** @emoji 🕹️ Mirrors `vcs::DocumentVcsCommand` (`#[serde(tag = "kind", rename_all = "camelCase")]`). */
export type DocumentVcsCommand<Op> =
  | { readonly kind: "apply"; readonly operations: Op[]; readonly description?: string }
  | { readonly kind: "undo" }
  | { readonly kind: "redo" }
  | { readonly kind: "undoWithPolicy"; readonly policy: UndoPolicy; readonly semanticCommand?: string }
  | { readonly kind: "commitCheckpoint"; readonly message?: string; readonly authors?: Author[] }
  | { readonly kind: "createAlternative"; readonly name: string }
  | { readonly kind: "switchAlternative"; readonly alternativeId: string }
  | { readonly kind: "checkoutCheckpoint"; readonly checkpointId: string }
  | { readonly kind: "amendLast"; readonly operations: Op[]; readonly coalesceKey?: string };
//#endregion 🔖Schemas

//#region 🔖Errors
export type VcsErrorKind =
  | "unknownEdit"
  | "unknownChange"
  | "unknownAlternative"
  | "noCheckpoint"
  | "emptyApply"
  | "nothingToUndo"
  | "foreignEdit"
  | "nothingToRedo"
  | "serialize"
  | "deserialize"
  | "backbone"
  | "remoteSyncNotImplemented";

/** @emoji 🚨 Mirrors `vcs::VcsError`'s variants (see the `#[error(...)]` messages in `vcs/rs/lib.rs`). */
export class VcsError extends Error {
  constructor(
    readonly kind: VcsErrorKind,
    message: string,
  ) {
    super(message);
    this.name = "VcsError";
  }
}

const unknownEdit = (id: string) => new VcsError("unknownEdit", `unknown edit id: ${id}`);
const unknownChange = (id: string) => new VcsError("unknownChange", `unknown change id: ${id}`);
const unknownAlternative = (id: string) => new VcsError("unknownAlternative", `unknown alternative id: ${id}`);
const noCheckpoint = () => new VcsError("noCheckpoint", "no checkpoint for alternative");
const emptyApply = () => new VcsError("emptyApply", "empty apply command");
const nothingToUndo = () => new VcsError("nothingToUndo", "nothing to undo");
const foreignEdit = (id: string) => new VcsError("foreignEdit", `cannot undo edit authored by another actor: ${id}`);
const nothingToRedo = () => new VcsError("nothingToRedo", "nothing to redo");
const backboneError = (message: string) => new VcsError("backbone", message);
//#endregion 🔖Errors

//#region 🔖Operation
/** @emoji 🔁 Per-document-schema operation contract: plain serializable `Op` values plus the pure
 * functions that apply/invert them. Mirrors `vcs::Operation<P>`, minus `OperationDiff`/`absorb`
 * (concurrent-diff merging) — that CRDT machinery is deliberately out of scope for this mirror; a real
 * `OsOp` port is a later ticket's job. `backwards` mirrors Rust's `Operation::backwards`: given the
 * PRE-op projection, return the ops that undo `op`, in forward order (the store reverses them). */
export interface OperationHandlers<P, Op> {
  forwards(op: Op, projection: P): P;
  backwards(op: Op, projection: P): Op[];
  operationId?(op: Op): string | undefined;
  dependencies?(op: Op): readonly string[];
  baseVersion?(op: Op): number;
  authorId?(op: Op): string | undefined;
  timestamp?(op: Op): HybridLogicalTimestamp | undefined;
  undoPolicy?(op: Op): UndoPolicy;
}

function applyOperation<P, Op>(handlers: OperationHandlers<P, Op>, projection: P, operation: Op): P {
  return handlers.forwards(operation, projection);
}

/** @emoji 🖋️ Derives an edit's authoring actor from its first operation's metadata. Mirrors
 * `vcs::edit_actor_from_meta`. */
function editActorFromMeta(operationMeta: OperationMeta[]): string | undefined {
  return operationMeta[0]?.authorId;
}

type ReplayResult<P, Op> = {
  readonly forwards: Op[];
  readonly backwards: Op[];
  readonly operationMeta: OperationMeta[];
  readonly post: P;
};

/** @emoji 🔂 Replays `operations` over `preProjection`. Mirrors `DocumentVcsStore::replay_operations`:
 * each op's own (possibly multi-element) backwards list is computed against its PRE-state and reversed
 * before being appended — the overall `backwards` array stays in forward op order across ops. */
function replayOperations<P, Op>(handlers: OperationHandlers<P, Op>, hasher: Hasher, preProjection: P, operations: Op[]): ReplayResult<P, Op> {
  let projection = preProjection;
  const forwards: Op[] = [];
  const backwards: Op[] = [];
  const operationMeta: OperationMeta[] = [];
  for (const operation of operations) {
    const back = handlers.backwards(operation, projection).slice().reverse();
    backwards.push(...back);
    operationMeta.push({
      operationId: handlers.operationId?.(operation) ?? createDocumentVcsId("operation"),
      dependencies: handlers.dependencies?.(operation) ?? [],
      baseVersion: handlers.baseVersion?.(operation) ?? 0,
      authorId: handlers.authorId?.(operation) ?? "local",
      timestamp: handlers.timestamp?.(operation) ?? { actor: 0, physicalMs: Date.now(), logical: 0 },
      undoPolicy: handlers.undoPolicy?.(operation) ?? "exactBaseOnly",
      payloadHash: hasher.hash(new TextEncoder().encode(JSON.stringify(operation))),
    });
    projection = applyOperation(handlers, projection, operation);
    forwards.push(operation);
  }
  return { forwards, backwards, operationMeta, post: projection };
}
//#endregion 🔖Operation

//#region 🔖Materialize
/** @emoji 🏗️ Mirrors `vcs::create_document_vcs_envelope`. */
export function createDocumentVcsEnvelope<P, Op>(schema: string, id: string, initialProjection: P, backbone?: DocumentBackboneRef): DocumentVcsEnvelope<P, Op> {
  return {
    schema,
    id,
    vcs: { initialProjection, edits: [], changes: [], checkpoints: [], alternatives: [] },
    backbone,
    activeAlternativeId: undefined,
  };
}

/** @emoji 🔗 Mirrors `vcs::edit_ids_for_changes`. */
function editIdsForChanges<P, Op>(envelope: DocumentVcsEnvelope<P, Op>, changeIds: readonly string[]): string[] {
  const editIds: string[] = [];
  for (const changeId of changeIds) {
    const change = envelope.vcs.changes.find((entry) => entry.id === changeId);
    if (change) editIds.push(...change.editIds);
  }
  return editIds;
}

/** @emoji 🎞️ Mirrors `vcs::materialize_document_projection` — replays only `Edit.forwards`. */
export function materializeDocumentProjection<P, Op>(handlers: OperationHandlers<P, Op>, envelope: DocumentVcsEnvelope<P, Op>, appliedEditIds: readonly string[]): P {
  let projection = envelope.vcs.initialProjection;
  for (const editId of appliedEditIds) {
    const edit = envelope.vcs.edits.find((entry) => entry.id === editId);
    if (!edit) throw unknownEdit(editId);
    for (const operation of edit.forwards) {
      projection = applyOperation(handlers, projection, operation);
    }
  }
  return projection;
}

/** @emoji 🧾 Mirrors `vcs::uncommitted_edit_ids`. */
function uncommittedEditIds<P, Op>(envelope: DocumentVcsEnvelope<P, Op>, appliedEditIds: readonly string[]): string[] {
  const committed = new Set<string>();
  for (const change of envelope.vcs.changes) for (const editId of change.editIds) committed.add(editId);
  return appliedEditIds.filter((id) => !committed.has(id));
}
//#endregion 🔖Materialize

//#region 🔖History
/** @emoji 📜 One row of a checkpoint history/ancestor graph. Mirrors `vcs::HistoryColumn`; consumed by
 * the eventual `vcs/react` `HistoryTable`. */
export type HistoryColumn = {
  readonly checkpointId: string;
  readonly timestamp: string;
  readonly labels: string[];
  readonly authors: Author[];
  readonly parentCheckpointId?: string;
  readonly description?: string;
  readonly lane: number;
  readonly alternativeIds: string[];
};

function checkpointAlternatives<P, Op>(envelope: DocumentVcsEnvelope<P, Op>, checkpointId: string): Alternative[] {
  return envelope.vcs.alternatives.filter((alternative) => alternative.checkpointIds.includes(checkpointId));
}

function isCheckpointMainOnly<P, Op>(envelope: DocumentVcsEnvelope<P, Op>, checkpointId: string): boolean {
  return checkpointAlternatives(envelope, checkpointId).length === 0;
}

function hasMainOnlyDescendant<P, Op>(envelope: DocumentVcsEnvelope<P, Op>, childrenOf: Map<string, string[]>, checkpointId: string, seen: Set<string>): boolean {
  if (seen.has(checkpointId)) return false;
  seen.add(checkpointId);
  for (const childId of childrenOf.get(checkpointId) ?? []) {
    if (isCheckpointMainOnly(envelope, childId) || hasMainOnlyDescendant(envelope, childrenOf, childId, seen)) return true;
  }
  return false;
}

/** @emoji 🛤️ Assigns each checkpoint a swimlane. Mirrors `vcs::assign_history_checkpoint_lanes`. */
function assignHistoryCheckpointLanes<P, Op>(envelope: DocumentVcsEnvelope<P, Op>): Map<string, number> {
  const laneByAlternative = new Map<string, number>();
  envelope.vcs.alternatives.forEach((alternative, index) => laneByAlternative.set(alternative.id, index + 1));
  const childrenOf = new Map<string, string[]>();
  for (const checkpoint of envelope.vcs.checkpoints) {
    if (checkpoint.parentId) {
      const children = childrenOf.get(checkpoint.parentId) ?? [];
      children.push(checkpoint.id);
      childrenOf.set(checkpoint.parentId, children);
    }
  }
  const laneByCheckpointId = new Map<string, number>();
  for (const checkpoint of envelope.vcs.checkpoints) {
    if (!checkpoint.parentId) {
      laneByCheckpointId.set(checkpoint.id, 0);
      continue;
    }
    const seen = new Set<string>();
    if (isCheckpointMainOnly(envelope, checkpoint.id) || hasMainOnlyDescendant(envelope, childrenOf, checkpoint.id, seen)) {
      laneByCheckpointId.set(checkpoint.id, 0);
      continue;
    }
    const lanes = checkpointAlternatives(envelope, checkpoint.id).map((alternative) => laneByAlternative.get(alternative.id) ?? 0);
    const lane = lanes.length === 1 ? lanes[0]! : Math.min(...lanes, 0);
    laneByCheckpointId.set(checkpoint.id, lane);
  }
  return laneByCheckpointId;
}

/** @emoji 📜 Builds the ancestor-graph rows for a checkpoint history view (newest first). Mirrors
 * `vcs::build_history_columns`; the future `vcs/react` `HistoryTable` consumes this array directly. */
export function buildHistoryColumns<P, Op>(envelope: DocumentVcsEnvelope<P, Op>): HistoryColumn[] {
  const laneByCheckpointId = assignHistoryCheckpointLanes(envelope);
  return envelope.vcs.checkpoints
    .slice()
    .reverse()
    .map((checkpoint, index) => {
      const alternatives = checkpointAlternatives(envelope, checkpoint.id);
      const alternativeIds = alternatives.map((alternative) => alternative.id);
      const labels = alternatives.map((alternative) => alternative.name);
      if (labels.length === 0 && index === 0) labels.push("main");
      return {
        checkpointId: checkpoint.id,
        timestamp: checkpoint.timestamp,
        labels,
        authors: checkpoint.authors,
        parentCheckpointId: checkpoint.parentId,
        description: checkpoint.message,
        lane: laneByCheckpointId.get(checkpoint.id) ?? 0,
        alternativeIds,
      };
    });
}
//#endregion 🔖History

//#region 🔖Sync
/** @emoji 🕸️ Mirrors `framework/core/rs`'s `OpEnvelope` — the causal wire envelope a remote peer sends.
 * Deliberately re-declared here (not imported from `framework/product/os/core`) since `vcs/core` sits
 * below the product/OS layer; see that bundle's own `OpEnvelope` twin for precedent on this duplication. */
export type OpEnvelope = {
  readonly id: string;
  readonly actor: string;
  readonly document: string;
  readonly schemaVersion: string;
  readonly deps?: readonly string[];
  readonly payloadHash: string;
  readonly diff: { readonly schemaId: string; readonly payload: unknown };
  readonly inverse: {
    readonly targetOperation: string;
    readonly inverseDiff: { readonly schemaId: string; readonly payload: unknown };
    readonly baseVersion: number;
    readonly dependencies?: readonly string[];
    readonly undoPolicy: UndoPolicy;
  };
};

/** @emoji 📦 Mirrors `vcs::edit_from_op_envelope`. */
function editFromOpEnvelope<Op>(envelope: OpEnvelope): Edit<Op> {
  return envelope.diff.payload as Edit<Op>;
}

/** @emoji 🕸️ Minimal causal buffer: an envelope with unmet `deps` waits until they land. Mirrors
 * `framework/core/rs`'s `OpDag`, simplified to ordered-by-deps replay per this ticket's scope (no
 * `Duplicate`/`Pending`/`AlreadyApplied` result enum — `insert` just no-ops on a repeat id). */
class CausalBuffer {
  private readonly envelopes = new Map<string, OpEnvelope>();
  private readonly applied = new Set<string>();
  private readonly appliedOrder: string[] = [];
  private pending: string[] = [];
  private drained = 0;

  insert(envelope: OpEnvelope): void {
    const id = envelope.id;
    if (this.applied.has(id) || this.envelopes.has(id)) return;
    const deps = envelope.deps ?? [];
    if (deps.some((dep) => !this.applied.has(dep))) {
      this.envelopes.set(id, envelope);
      this.pending.push(id);
      return;
    }
    this.envelopes.set(id, envelope);
    this.markApplied(id);
    this.drainReady();
  }

  private markApplied(id: string): void {
    this.applied.add(id);
    this.appliedOrder.push(id);
    this.pending = this.pending.filter((pendingId) => pendingId !== id);
  }

  private drainReady(): void {
    for (;;) {
      const ready = this.pending.filter((id) => (this.envelopes.get(id)?.deps ?? []).every((dep) => this.applied.has(dep)));
      if (ready.length === 0) return;
      for (const id of ready) this.markApplied(id);
    }
  }

  drainAppliedEnvelopes(): OpEnvelope[] {
    const fresh = this.appliedOrder.slice(this.drained);
    this.drained = this.appliedOrder.length;
    return fresh.map((id) => this.envelopes.get(id)!);
  }
}
//#endregion 🔖Sync

//#region 🔖DocumentVcsStore
export type DocumentVcsStoreOptions = { readonly hasher?: Hasher };

/** @emoji 🧑‍🤝‍🧑 Structural mirror of `vcs::StudioMember` — a document facade a {@link StudioVcsHost}
 * can register without knowing its concrete `P`/`Op`. Rust needs `as_any_mut` to downcast a `dyn
 * StudioMember` back to a concrete `DocumentVcsStore<P, Op>`; TS needs no such escape hatch since a
 * caller already holds its own typed `DocumentVcsStore` reference — register that same object (it
 * implements this interface directly) and keep dispatching through your typed handle. */
export interface StudioMember {
  documentId(): string;
  isDirty(): boolean;
  commitCheckpoint(message: string, authors: Author[]): string;
  currentCheckpointId(): string | undefined;
  currentAlternativeId(): string | undefined;
  checkout(checkpointId: string, alternativeId: string): void;
  createAlternative(name: string): string;
  lastLocalEditTimestamp(): HybridLogicalTimestamp | undefined;
  lastUndoneLocalEditTimestamp(): HybridLogicalTimestamp | undefined;
  undo(): void;
  redo(): void;
}

/** @emoji 🗄️ Mirrors `vcs::DocumentVcsStore` — apply/undo/redo/commit/branch dispatch over a generic
 * `DocumentVcsEnvelope<P, Op>`, plus `ingestRemote`/`mergeRemoteSnapshot` for causal/whole-snapshot
 * sync. No `Backbone`/`pump`/`flushOutbound` — that IO-owning plumbing has no TS mirror yet (see the
 * module header). Implements {@link StudioMember} directly (mirrors Rust's blanket
 * `impl<P, Op> StudioMember for DocumentVcsStore<P, Op>`). */
export class DocumentVcsStore<P, Op> implements StudioMember {
  private readonly hasher: Hasher;
  private readonly causalBuffer = new CausalBuffer();
  private appliedEditIds_: string[] = [];
  private redoEditIds_: string[] = [];
  private editSequence = 0;
  private generation_ = 0;
  private currentCheckpointId_: string | undefined;
  private localActorId_: string | undefined;

  constructor(
    private envelope: DocumentVcsEnvelope<P, Op>,
    private readonly handlers: OperationHandlers<P, Op>,
    options: DocumentVcsStoreOptions = {},
  ) {
    this.hasher = options.hasher ?? createDefaultHasher();
    this.currentCheckpointId_ = envelope.vcs.checkpoints.at(-1)?.id;
  }

  //#region 🔖Accessors
  getEnvelope(): DocumentVcsEnvelope<P, Op> {
    return this.envelope;
  }

  appliedEditIds(): readonly string[] {
    return this.appliedEditIds_;
  }

  redoEditIds(): readonly string[] {
    return this.redoEditIds_;
  }

  generation(): number {
    return this.generation_;
  }

  currentCheckpointId(): string | undefined {
    return this.currentCheckpointId_;
  }

  setCurrentCheckpointId(checkpointId: string | undefined): void {
    this.currentCheckpointId_ = checkpointId;
  }

  localActorId(): string | undefined {
    return this.localActorId_;
  }

  setLocalActorId(actorId: string | undefined): void {
    this.localActorId_ = actorId;
  }

  /** @emoji 🔧 The most recently created/amended edit's `(forwards, backwards, operationMeta)`. */
  editOperations(): { readonly forwards: readonly Op[]; readonly backwards: readonly Op[]; readonly operationMeta: readonly OperationMeta[] } | undefined {
    const edit = this.envelope.vcs.edits.at(-1);
    return edit && { forwards: edit.forwards, backwards: edit.backwards, operationMeta: edit.operationMeta };
  }

  /** @emoji 📜 Ancestor-graph rows for this store's checkpoint history. See {@link buildHistoryColumns}. */
  historyColumns(): HistoryColumn[] {
    return buildHistoryColumns(this.envelope);
  }

  projection(): P {
    return materializeDocumentProjection(this.handlers, this.envelope, this.appliedEditIds_);
  }
  //#endregion 🔖Accessors

  //#region 🔖SetState
  /** @emoji 💾 Restores full store state (including redo), mirroring `DocumentVcsStore::set_state`. */
  setState(envelope: DocumentVcsEnvelope<P, Op>, appliedEditIds: string[], redoEditIds: string[] = []): void {
    this.editSequence = Math.max(0, ...envelope.vcs.edits.map((edit) => edit.sequenceNumber));
    this.currentCheckpointId_ = envelope.vcs.checkpoints.at(-1)?.id;
    this.envelope = envelope;
    this.appliedEditIds_ = appliedEditIds;
    this.redoEditIds_ = redoEditIds;
    this.bump();
  }
  //#endregion 🔖SetState

  //#region 🔖Dispatch
  dispatch(command: DocumentVcsCommand<Op>): void {
    this.dispatchInner(command);
  }

  private dispatchInner(command: DocumentVcsCommand<Op>): void {
    switch (command.kind) {
      case "undo":
        this.dispatchInner({ kind: "undoWithPolicy", policy: "exactBaseOnly", semanticCommand: undefined });
        return;
      case "undoWithPolicy":
        this.dispatchUndoWithPolicy(command.policy, command.semanticCommand);
        return;
      case "redo":
        this.dispatchRedo();
        return;
      case "commitCheckpoint":
        this.dispatchCommitCheckpoint(command.message, command.authors ?? []);
        return;
      case "createAlternative":
        this.dispatchCreateAlternative(command.name);
        return;
      case "switchAlternative":
        this.dispatchSwitchAlternative(command.alternativeId);
        return;
      case "checkoutCheckpoint":
        this.dispatchCheckoutCheckpoint(command.checkpointId);
        return;
      case "apply":
        this.dispatchApply(command.operations, command.description);
        return;
      case "amendLast":
        this.dispatchAmendLast(command.operations, command.coalesceKey);
        return;
    }
  }

  private dispatchUndoWithPolicy(policy: UndoPolicy, semanticCommand: string | undefined): void {
    if (policy === "transformAgainstConcurrent") {
      const position = [...this.appliedEditIds_].map((id, index) => [id, index] as const).reverse().find(([id]) => this.editIsLocal(id))?.[1];
      if (position === undefined) throw nothingToUndo();
      const [removed] = this.appliedEditIds_.splice(position, 1);
      this.redoEditIds_.push(removed!);
      this.bump();
      return;
    }
    if (policy === "semanticUndo" || policy === "compensatingAction") {
      if (semanticCommand === undefined) throw backboneError("semantic undo requires compensating command");
    }
    const last = this.appliedEditIds_.at(-1);
    if (last === undefined) throw nothingToUndo();
    if (!this.editIsLocal(last)) throw foreignEdit(last);
    this.appliedEditIds_.pop();
    this.redoEditIds_.push(last);
    this.bump();
  }

  private dispatchRedo(): void {
    const next = this.redoEditIds_.pop();
    if (next === undefined) throw nothingToRedo();
    this.appliedEditIds_.push(next);
    this.bump();
  }

  private dispatchCommitCheckpoint(message: string | undefined, authors: Author[]): void {
    const pending = uncommittedEditIds(this.envelope, this.appliedEditIds_);
    if (pending.length === 0) return;
    const change: Change = { id: createDocumentVcsId("change"), editIds: pending, description: message, savedAt: nowIso() };
    const parent = this.currentCheckpointId_ ? this.envelope.vcs.checkpoints.find((checkpoint) => checkpoint.id === this.currentCheckpointId_) : undefined;
    const changeIds = [...(parent?.changeIds ?? []), change.id];
    const checkpoint: Checkpoint = { id: createDocumentVcsId("checkpoint"), changeIds, parentId: parent?.id, authors, message, timestamp: nowIso() };
    this.envelope.vcs.changes.push(change);
    this.envelope.vcs.checkpoints.push(checkpoint);
    if (this.envelope.activeAlternativeId) {
      const alternative = this.envelope.vcs.alternatives.find((entry) => entry.id === this.envelope.activeAlternativeId);
      alternative?.checkpointIds.push(checkpoint.id);
    }
    this.currentCheckpointId_ = checkpoint.id;
    this.bump();
  }

  private dispatchCreateAlternative(name: string): void {
    if (this.envelope.vcs.checkpoints.length === 0) this.dispatchInner({ kind: "commitCheckpoint", message: undefined, authors: [] });
    const checkpointId = this.currentCheckpointId_ ?? this.envelope.vcs.checkpoints.at(-1)?.id;
    if (!checkpointId) throw noCheckpoint();
    const alternativeId = createDocumentVcsId("alternative");
    this.envelope.vcs.alternatives.push({ id: alternativeId, name, checkpointIds: [checkpointId] });
    this.envelope.activeAlternativeId = alternativeId;
    this.checkoutCheckpointInternal(checkpointId);
    this.bump();
  }

  private dispatchSwitchAlternative(alternativeId: string): void {
    const alternative = this.envelope.vcs.alternatives.find((entry) => entry.id === alternativeId);
    if (!alternative) throw unknownAlternative(alternativeId);
    const checkpointId = alternative.checkpointIds.at(-1);
    if (!checkpointId) throw noCheckpoint();
    if (!this.envelope.vcs.checkpoints.some((checkpoint) => checkpoint.id === checkpointId)) throw noCheckpoint();
    this.checkoutCheckpointInternal(checkpointId);
    this.envelope.activeAlternativeId = alternativeId;
    this.bump();
  }

  private dispatchCheckoutCheckpoint(checkpointId: string): void {
    if (!this.envelope.vcs.checkpoints.some((checkpoint) => checkpoint.id === checkpointId)) throw unknownChange(checkpointId);
    this.checkoutCheckpointInternal(checkpointId);
    this.envelope.activeAlternativeId = this.envelope.vcs.alternatives.find((alternative) => alternative.checkpointIds.at(-1) === checkpointId)?.id;
    this.bump();
  }

  private dispatchApply(operations: Op[], description: string | undefined): void {
    if (operations.length === 0) throw emptyApply();
    const startedAt = nowIso();
    const preProjection = this.projection();
    const { forwards, backwards, operationMeta } = replayOperations(this.handlers, this.hasher, preProjection, operations);
    const actor = editActorFromMeta(operationMeta);
    this.localActorId_ = actor;
    this.editSequence += 1;
    const edit: Edit<Op> = {
      id: createDocumentVcsId("edit"),
      actor,
      forwards,
      backwards,
      operationMeta,
      description,
      coalesceKey: undefined,
      sequenceNumber: this.editSequence,
      startedAt,
      finishedAt: nowIso(),
    };
    this.appliedEditIds_.push(edit.id);
    this.envelope.vcs.edits.push(edit);
    this.redoEditIds_ = [];
    this.bump();
  }

  private dispatchAmendLast(operations: Op[], coalesceKey: string | undefined): void {
    if (operations.length === 0) throw emptyApply();
    const lastId = this.appliedEditIds_.at(-1);
    const amendTarget =
      coalesceKey !== undefined && lastId !== undefined && uncommittedEditIds(this.envelope, this.appliedEditIds_).includes(lastId) && this.envelope.vcs.edits.find((edit) => edit.id === lastId)?.coalesceKey === coalesceKey
        ? lastId
        : undefined;
    if (amendTarget) {
      const preIds = this.appliedEditIds_.slice(0, -1);
      const preProjection = materializeDocumentProjection(this.handlers, this.envelope, preIds);
      const target = this.envelope.vcs.edits.find((edit) => edit.id === amendTarget)!;
      const combined = [...target.forwards, ...operations];
      const { forwards, backwards, operationMeta } = replayOperations(this.handlers, this.hasher, preProjection, combined);
      target.forwards = forwards;
      target.backwards = backwards;
      target.operationMeta = operationMeta;
      target.finishedAt = nowIso();
      this.redoEditIds_ = [];
      this.bump();
      return;
    }
    const startedAt = nowIso();
    const preProjection = this.projection();
    const { forwards, backwards, operationMeta } = replayOperations(this.handlers, this.hasher, preProjection, operations);
    const actor = editActorFromMeta(operationMeta);
    this.localActorId_ = actor;
    this.editSequence += 1;
    const edit: Edit<Op> = {
      id: createDocumentVcsId("edit"),
      actor,
      forwards,
      backwards,
      operationMeta,
      description: undefined,
      coalesceKey,
      sequenceNumber: this.editSequence,
      startedAt,
      finishedAt: nowIso(),
    };
    this.appliedEditIds_.push(edit.id);
    this.envelope.vcs.edits.push(edit);
    this.redoEditIds_ = [];
    this.bump();
  }

  /** @emoji 🧭 Mirrors `DocumentVcsStore::checkout_checkpoint_internal`. */
  private checkoutCheckpointInternal(checkpointId: string): void {
    const checkpoint = this.envelope.vcs.checkpoints.find((entry) => entry.id === checkpointId);
    this.appliedEditIds_ = checkpoint ? editIdsForChanges(this.envelope, checkpoint.changeIds) : [];
    this.redoEditIds_ = [];
    this.currentCheckpointId_ = checkpointId;
  }

  /** @emoji 🖋️ Mirrors `DocumentVcsStore::edit_is_local`: unauthored edits count as local. */
  private editIsLocal(editId: string): boolean {
    const edit = this.envelope.vcs.edits.find((entry) => entry.id === editId);
    if (!edit) return false;
    return edit.actor === undefined || edit.actor === this.localActorId_;
  }

  private bump(): void {
    this.generation_ += 1;
  }
  //#endregion 🔖Dispatch

  //#region 🔖Sync
  /** @emoji 🕸️ Feeds a remote {@link OpEnvelope} through the causal buffer, applying it (and any
   * now-unblocked dependents) into the edit timeline. Mirrors `DocumentVcsStore::ingest_remote`. */
  ingestRemote(envelope: OpEnvelope): void {
    this.causalBuffer.insert(envelope);
    for (const applied of this.causalBuffer.drainAppliedEnvelopes()) this.ingestEnvelope(applied);
  }

  private ingestEnvelope(envelope: OpEnvelope): void {
    const edit = editFromOpEnvelope<Op>(envelope);
    edit.actor = envelope.actor;
    if (this.envelope.vcs.edits.some((existing) => existing.id === edit.id)) return;
    this.editSequence = Math.max(this.editSequence, edit.sequenceNumber);
    this.envelope.vcs.edits.push(edit);
    this.appliedEditIds_.push(edit.id);
    this.bump();
  }

  /** @emoji 📦 Merges a remote whole-envelope snapshot. Mirrors `DocumentVcsStore::merge_remote_snapshot`:
   * an empty local timeline adopts the remote wholesale; otherwise edits/changes/checkpoints/alternatives
   * merge by id, keeping every local entry. */
  mergeRemoteSnapshot(envelopeJson: string): void {
    const remote = JSON.parse(envelopeJson) as DocumentVcsEnvelope<P, Op>;
    if (this.envelope.vcs.edits.length === 0) {
      const applied = remote.vcs.edits.map((edit) => edit.id);
      this.editSequence = Math.max(0, ...remote.vcs.edits.map((edit) => edit.sequenceNumber));
      const backbone = this.envelope.backbone;
      this.envelope = remote;
      this.envelope.backbone = backbone;
      this.appliedEditIds_ = applied;
      this.redoEditIds_ = [];
      this.bump();
      return;
    }
    const existingEditIds = new Set(this.envelope.vcs.edits.map((edit) => edit.id));
    for (const edit of remote.vcs.edits) {
      if (existingEditIds.has(edit.id)) continue;
      this.editSequence = Math.max(this.editSequence, edit.sequenceNumber);
      this.appliedEditIds_.push(edit.id);
      this.envelope.vcs.edits.push(edit);
    }
    mergeById(this.envelope.vcs.changes, remote.vcs.changes, (change) => change.id);
    mergeById(this.envelope.vcs.checkpoints, remote.vcs.checkpoints, (checkpoint) => checkpoint.id);
    mergeById(this.envelope.vcs.alternatives, remote.vcs.alternatives, (alternative) => alternative.id);
    this.bump();
  }
  //#endregion 🔖Sync

  //#region 🔖StudioMember
  /** @emoji 🧑‍🤝‍🧑 {@link StudioMember} implementation — mirrors `impl<P, Op> StudioMember for
   * DocumentVcsStore<P, Op>` in `vcs/rs/lib.rs`. */
  documentId(): string {
    return this.envelope.id;
  }

  isDirty(): boolean {
    return uncommittedEditIds(this.envelope, this.appliedEditIds_).length > 0;
  }

  commitCheckpoint(message: string, authors: Author[]): string {
    this.dispatch({ kind: "commitCheckpoint", message, authors });
    if (!this.currentCheckpointId_) throw noCheckpoint();
    return this.currentCheckpointId_;
  }

  currentAlternativeId(): string | undefined {
    return this.envelope.activeAlternativeId;
  }

  checkout(checkpointId: string, alternativeId: string): void {
    if (alternativeId) {
      const alternative = this.envelope.vcs.alternatives.find((entry) => entry.id === alternativeId);
      if (alternative?.checkpointIds.at(-1) === checkpointId) {
        this.dispatch({ kind: "switchAlternative", alternativeId });
        return;
      }
    }
    this.dispatch({ kind: "checkoutCheckpoint", checkpointId });
  }

  createAlternative(name: string): string {
    this.dispatch({ kind: "createAlternative", name });
    if (!this.envelope.activeAlternativeId) throw noCheckpoint();
    return this.envelope.activeAlternativeId;
  }

  lastLocalEditTimestamp(): HybridLogicalTimestamp | undefined {
    for (let index = this.appliedEditIds_.length - 1; index >= 0; index -= 1) {
      const editId = this.appliedEditIds_[index]!;
      if (!this.editIsLocal(editId)) continue;
      const timestamp = this.envelope.vcs.edits.find((edit) => edit.id === editId)?.operationMeta.at(-1)?.timestamp;
      if (timestamp) return timestamp;
    }
    return undefined;
  }

  lastUndoneLocalEditTimestamp(): HybridLogicalTimestamp | undefined {
    for (let index = this.redoEditIds_.length - 1; index >= 0; index -= 1) {
      const editId = this.redoEditIds_[index]!;
      if (!this.editIsLocal(editId)) continue;
      const timestamp = this.envelope.vcs.edits.find((edit) => edit.id === editId)?.operationMeta.at(-1)?.timestamp;
      if (timestamp) return timestamp;
    }
    return undefined;
  }

  undo(): void {
    this.dispatch({ kind: "undo" });
  }

  redo(): void {
    this.dispatch({ kind: "redo" });
  }
  //#endregion 🔖StudioMember
}

/** @emoji 🔀 Mirrors `vcs::merge_by_id`: appends remote items whose id isn't already present locally. */
function mergeById<T>(local: T[], remote: T[], idOf: (item: T) => string): void {
  const existing = new Set(local.map(idOf));
  for (const item of remote) {
    const id = idOf(item);
    if (!existing.has(id)) {
      existing.add(id);
      local.push(item);
    }
  }
}
//#endregion 🔖DocumentVcsStore

//#region 🔖Studio
/** @emoji 📌 One member document's position at the moment a `StudioCheckpoint` was recorded. Mirrors
 * `vcs::StudioMemberPin`. `alternativeId` is `""` when the member had no active alternative. */
export type StudioMemberPin = {
  readonly documentId: string;
  readonly checkpointId: string;
  readonly alternativeId: string;
};

/** @emoji 🗄️ A studio-wide checkpoint: one pin per registered member. Mirrors `vcs::StudioCheckpoint`. */
export type StudioCheckpoint = {
  readonly id: string;
  readonly parentId?: string;
  readonly message: string;
  readonly authors: Author[];
  readonly timestamp: HybridLogicalTimestamp;
  readonly members: StudioMemberPin[];
};

export type StudioAlternative = {
  readonly id: string;
  readonly name: string;
  checkpointIds: string[];
};

/** @emoji 🗄️ Projection of the `"os.studio.history"` meta-document. Mirrors `vcs::StudioHistoryProjection`. */
export type StudioHistoryProjection = {
  checkpoints: StudioCheckpoint[];
  alternatives: StudioAlternative[];
  activeAlternativeId?: string;
};

/** @emoji 🕹️ Mirrors `vcs::StudioHistoryOp` (`#[serde(tag = "op", rename_all = "camelCase")]`). */
export type StudioHistoryOp =
  | { readonly op: "commitStudioCheckpoint"; readonly checkpoint: StudioCheckpoint }
  | { readonly op: "createStudioAlternative"; readonly alternative: StudioAlternative }
  | { readonly op: "switchStudioAlternative"; readonly alternativeId: string }
  | { readonly op: "removeStudioCheckpoint"; readonly checkpointId: string }
  | { readonly op: "removeStudioAlternative"; readonly alternativeId: string }
  | { readonly op: "setActiveStudioAlternative"; readonly alternativeId?: string };

/** @emoji 🔁 Mirrors `Operation<StudioHistoryProjection> for StudioHistoryOp` (`diff`/`apply` collapsed
 * into a direct projection transform, since the sparse `StudioHistoryDiff`/`absorb` CRDT-merge type has
 * no TS mirror — out of scope, see the module header). `backwards` takes the PRE-op projection, exactly
 * like every other {@link OperationHandlers}. */
export const studioHistoryOpHandlers: OperationHandlers<StudioHistoryProjection, StudioHistoryOp> = {
  forwards(op, projection) {
    switch (op.op) {
      case "commitStudioCheckpoint":
        return { ...projection, checkpoints: [...projection.checkpoints, op.checkpoint] };
      case "createStudioAlternative":
        return { ...projection, alternatives: [...projection.alternatives, op.alternative], activeAlternativeId: op.alternative.id };
      case "switchStudioAlternative":
        return { ...projection, activeAlternativeId: op.alternativeId };
      case "removeStudioCheckpoint":
        return { ...projection, checkpoints: projection.checkpoints.filter((checkpoint) => checkpoint.id !== op.checkpointId) };
      case "removeStudioAlternative":
        return { ...projection, alternatives: projection.alternatives.filter((alternative) => alternative.id !== op.alternativeId) };
      case "setActiveStudioAlternative":
        return { ...projection, activeAlternativeId: op.alternativeId };
    }
  },
  backwards(op, projection) {
    switch (op.op) {
      case "commitStudioCheckpoint":
        return [{ op: "removeStudioCheckpoint", checkpointId: op.checkpoint.id }];
      case "createStudioAlternative":
        return [{ op: "setActiveStudioAlternative", alternativeId: projection.activeAlternativeId }, { op: "removeStudioAlternative", alternativeId: op.alternative.id }];
      case "switchStudioAlternative":
        return [{ op: "setActiveStudioAlternative", alternativeId: projection.activeAlternativeId }];
      case "removeStudioCheckpoint": {
        const checkpoint = projection.checkpoints.find((entry) => entry.id === op.checkpointId);
        return checkpoint ? [{ op: "commitStudioCheckpoint", checkpoint }] : [];
      }
      case "removeStudioAlternative": {
        const alternative = projection.alternatives.find((entry) => entry.id === op.alternativeId);
        return alternative ? [{ op: "createStudioAlternative", alternative }] : [];
      }
      case "setActiveStudioAlternative":
        return [{ op: "setActiveStudioAlternative", alternativeId: projection.activeAlternativeId }];
    }
  },
};

/** @emoji 🏛️ Composes many {@link StudioMember} documents under one studio-wide checkpoint/alternative
 * timeline, stored in a dogfooded `"os.studio.history"` meta-document. Mirrors `vcs::StudioVcsHost`. */
export class StudioVcsHost {
  private readonly meta: DocumentVcsStore<StudioHistoryProjection, StudioHistoryOp>;
  private readonly members = new Map<string, StudioMember>();

  constructor(metaEnvelope: DocumentVcsEnvelope<StudioHistoryProjection, StudioHistoryOp>) {
    this.meta = new DocumentVcsStore(metaEnvelope, studioHistoryOpHandlers);
  }

  registerMember(member: StudioMember): void {
    this.members.set(member.documentId(), member);
  }

  unregisterMember(documentId: string): StudioMember | undefined {
    const member = this.members.get(documentId);
    this.members.delete(documentId);
    return member;
  }

  member(documentId: string): StudioMember | undefined {
    return this.members.get(documentId);
  }

  metaProjection(): StudioHistoryProjection {
    return this.meta.projection();
  }

  /** @emoji 💾 Commits every dirty member, pins each member's resulting `(checkpoint, alternative)`,
   * and records one `StudioCheckpoint` on the meta-document — applied and committed there too. */
  commitStudioCheckpoint(message: string, authors: Author[]): string {
    const documentIds = [...this.members.keys()].sort();
    const pins: StudioMemberPin[] = [];
    for (const documentId of documentIds) {
      const member = this.members.get(documentId)!;
      if (member.isDirty()) member.commitCheckpoint(message, authors);
      const checkpointId = member.currentCheckpointId();
      if (!checkpointId) throw noCheckpoint();
      pins.push({ documentId, checkpointId, alternativeId: member.currentAlternativeId() ?? "" });
    }
    const checkpointId = createDocumentVcsId("studio-checkpoint");
    const parentId = this.meta.projection().checkpoints.at(-1)?.id;
    const checkpoint: StudioCheckpoint = { id: checkpointId, parentId, message, authors, timestamp: { actor: 0, physicalMs: Date.now(), logical: 0 }, members: pins };
    this.meta.dispatch({ kind: "apply", operations: [{ op: "commitStudioCheckpoint", checkpoint }], description: message });
    this.meta.dispatch({ kind: "commitCheckpoint", message: undefined, authors: [] });
    return checkpointId;
  }

  /** @emoji 🌿 Records a `StudioAlternative` pinned at the current studio checkpoint tip. */
  createStudioAlternative(name: string): string {
    const checkpointId = this.meta.projection().checkpoints.at(-1)?.id;
    const alternativeId = createDocumentVcsId("studio-alternative");
    const alternative: StudioAlternative = { id: alternativeId, name, checkpointIds: checkpointId ? [checkpointId] : [] };
    this.meta.dispatch({ kind: "apply", operations: [{ op: "createStudioAlternative", alternative }] });
    return alternativeId;
  }

  /** @emoji 🔀 Fans out to every member pinned by `checkpointId`'s `StudioCheckpoint`, restoring each
   * to its exact recorded `(checkpoint, alternative)`. */
  checkoutStudioCheckpoint(checkpointId: string): void {
    const checkpoint = this.meta.projection().checkpoints.find((entry) => entry.id === checkpointId);
    if (!checkpoint) throw noCheckpoint();
    for (const pin of checkpoint.members) this.members.get(pin.documentId)?.checkout(pin.checkpointId, pin.alternativeId);
  }

  /** @emoji 🔀 Switches the studio's active alternative and fans out to its tip checkpoint's pins. */
  switchStudioAlternative(alternativeId: string): void {
    const alternative = this.meta.projection().alternatives.find((entry) => entry.id === alternativeId);
    if (!alternative) throw unknownAlternative(alternativeId);
    const checkpointId = alternative.checkpointIds.at(-1);
    if (!checkpointId) throw noCheckpoint();
    this.meta.dispatch({ kind: "apply", operations: [{ op: "switchStudioAlternative", alternativeId }] });
    this.checkoutStudioCheckpoint(checkpointId);
  }

  /** @emoji ↩️ Derived, local-only undo: targets whichever registered member has the most recent
   * `lastLocalEditTimestamp` (by physicalMs, then logical) and undoes just that member. Never
   * dispatched against the meta-document. Mirrors `StudioVcsHost::undo`. */
  undo(): void {
    const target = this.mostRecentMember((member) => member.lastLocalEditTimestamp());
    if (!target) throw nothingToUndo();
    this.members.get(target)!.undo();
  }

  /** @emoji ↪️ Derived, local-only redo: mirrors {@link undo}, targeting the member with the most
   * recent `lastUndoneLocalEditTimestamp`. */
  redo(): void {
    const target = this.mostRecentMember((member) => member.lastUndoneLocalEditTimestamp());
    if (!target) throw nothingToRedo();
    this.members.get(target)!.redo();
  }

  private mostRecentMember(timestampOf: (member: StudioMember) => HybridLogicalTimestamp | undefined): string | undefined {
    let best: { readonly documentId: string; readonly physicalMs: number; readonly logical: number } | undefined;
    for (const [documentId, member] of this.members) {
      const timestamp = timestampOf(member);
      if (!timestamp) continue;
      if (!best || timestamp.physicalMs > best.physicalMs || (timestamp.physicalMs === best.physicalMs && timestamp.logical > best.logical)) {
        best = { documentId, physicalMs: timestamp.physicalMs, logical: timestamp.logical };
      }
    }
    return best?.documentId;
  }
}
//#endregion 🔖Studio

//#region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;
  const { readFileSync } = await import("node:fs");
  const { fileURLToPath } = await import("node:url");

  type CounterProjection = { n: number };
  type CounterOp = { op: "setN"; n: number };

  const counterHandlers: OperationHandlers<CounterProjection, CounterOp> = {
    forwards: (op) => ({ n: op.n }),
    backwards: (_op, projection) => [{ op: "setN", n: projection.n }],
  };

  const newCounterStore = (id = "demo") => new DocumentVcsStore<CounterProjection, CounterOp>(createDocumentVcsEnvelope("demo/v1", id, { n: 0 }), counterHandlers);

  describe("@semio-tech/vcs-core DocumentVcsStore", () => {
    it("materializes forward ops on apply", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      expect(store.projection().n).toBe(1);
      expect(store.getEnvelope().vcs.edits).toHaveLength(1);
    });

    it("round-trips undo/redo", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      store.dispatch({ kind: "undo" });
      expect(store.projection().n).toBe(0);
      store.dispatch({ kind: "redo" });
      expect(store.projection().n).toBe(1);
    });

    it("computes backwards from pre-apply state", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 5 }] });
      expect(store.getEnvelope().vcs.edits[0]!.backwards).toEqual([{ op: "setN", n: 0 }]);
    });

    it("wraps edits into a change on commitCheckpoint", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      store.dispatch({ kind: "commitCheckpoint", message: "init", authors: [{ id: "a1", name: "Alice" }] });
      expect(store.getEnvelope().vcs.changes).toHaveLength(1);
      expect(store.getEnvelope().vcs.checkpoints).toHaveLength(1);
      expect(store.getEnvelope().vcs.checkpoints[0]!.message).toBe("init");
    });

    it("restores applied edits on checkoutCheckpoint", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      store.dispatch({ kind: "commitCheckpoint", message: "c1", authors: [] });
      const checkpointId = store.getEnvelope().vcs.checkpoints[0]!.id;
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 9 }] });
      expect(store.projection().n).toBe(9);
      store.dispatch({ kind: "checkoutCheckpoint", checkpointId });
      expect(store.projection().n).toBe(1);
    });

    it("rejects undo of a foreign edit", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      const edit = store.getEnvelope().vcs.edits[0]!;
      edit.actor = "peer-actor";
      expect(() => store.dispatch({ kind: "undo" })).toThrow(/authored by another actor/i);
    });

    it("amendLast coalesces into the last uncommitted edit by coalesceKey", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "amendLast", operations: [{ op: "setN", n: 1 }], coalesceKey: "gesture-1" });
      store.dispatch({ kind: "amendLast", operations: [{ op: "setN", n: 2 }], coalesceKey: "gesture-1" });
      expect(store.getEnvelope().vcs.edits).toHaveLength(1);
      expect(store.projection().n).toBe(2);
      const edit = store.getEnvelope().vcs.edits[0]!;
      expect(edit.forwards).toEqual([
        { op: "setN", n: 1 },
        { op: "setN", n: 2 },
      ]);
      expect(edit.backwards).toEqual([
        { op: "setN", n: 0 },
        { op: "setN", n: 1 },
      ]);
    });

    it("merges a remote snapshot onto an empty local timeline", () => {
      const local = newCounterStore();
      const remote = newCounterStore();
      remote.dispatch({ kind: "apply", operations: [{ op: "setN", n: 3 }] });
      local.mergeRemoteSnapshot(JSON.stringify(remote.getEnvelope()));
      expect(local.projection().n).toBe(3);
    });

    it("ingests a remote OpEnvelope with satisfied deps", () => {
      const peer = newCounterStore();
      peer.dispatch({ kind: "apply", operations: [{ op: "setN", n: 7 }] });
      const peerEdit = peer.getEnvelope().vcs.edits[0]!;
      const opEnvelope: OpEnvelope = {
        id: peerEdit.id,
        actor: "peer",
        document: "demo",
        schemaVersion: "demo/v1",
        deps: [],
        payloadHash: "irrelevant-for-this-mirror",
        diff: { schemaId: "demo/v1", payload: peerEdit },
        inverse: { targetOperation: peerEdit.id, inverseDiff: { schemaId: "demo/v1", payload: { backwards: peerEdit.backwards } }, baseVersion: peerEdit.sequenceNumber, dependencies: [], undoPolicy: "exactBaseOnly" },
      };
      const local = newCounterStore();
      local.ingestRemote(opEnvelope);
      expect(local.projection().n).toBe(7);
      expect(local.getEnvelope().vcs.edits[0]!.actor).toBe("peer");
    });

    it("buildHistoryColumns labels the newest unlabeled checkpoint 'main'", () => {
      const store = newCounterStore();
      store.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      store.dispatch({ kind: "commitCheckpoint", message: "init", authors: [] });
      const columns = store.historyColumns();
      expect(columns).toHaveLength(1);
      expect(columns[0]!.labels).toEqual(["main"]);
      expect(columns[0]!.lane).toBe(0);
    });

    it("replays the hand-authored checkpoint/undo/redo/branch fixture", () => {
      type Fixture = {
        readonly initialProjection: CounterProjection;
        readonly commands: DocumentVcsCommand<CounterOp>[];
        readonly expectedProjectionNAfterEachCommand: number[];
      };
      const fixturePath = fileURLToPath(new URL("./fixtures/checkpoint-undo-redo-branch.json", import.meta.url));
      const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
      const store = new DocumentVcsStore<CounterProjection, CounterOp>(createDocumentVcsEnvelope("demo/v1", "demo", fixture.initialProjection), counterHandlers);
      const actual = fixture.commands.map((command) => {
        store.dispatch(command);
        return store.projection().n;
      });
      expect(actual).toEqual(fixture.expectedProjectionNAfterEachCommand);
    });
  });

  describe("@semio-tech/vcs-core StudioVcsHost", () => {
    const newStudioHost = () => new StudioVcsHost(createDocumentVcsEnvelope<StudioHistoryProjection, StudioHistoryOp>("os.studio.history/v1", "studio", { checkpoints: [], alternatives: [] }));

    it("studio checkpoint commits dirty members and pins their checkpoints", () => {
      const memberA = newCounterStore("member-a");
      memberA.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });

      const memberB = newCounterStore("member-b");
      memberB.dispatch({ kind: "apply", operations: [{ op: "setN", n: 5 }] });
      memberB.dispatch({ kind: "commitCheckpoint", message: "b-init", authors: [] });
      const memberBCheckpoint = memberB.currentCheckpointId()!;

      const host = newStudioHost();
      host.registerMember(memberA);
      host.registerMember(memberB);

      const studioCheckpointId = host.commitStudioCheckpoint("studio init", [{ id: "a1", name: "Alice" }]);

      const projection = host.metaProjection();
      expect(projection.checkpoints).toHaveLength(1);
      const checkpoint = projection.checkpoints[0]!;
      expect(checkpoint.id).toBe(studioCheckpointId);
      expect(checkpoint.members).toHaveLength(2);
      const pinB = checkpoint.members.find((pin) => pin.documentId === "member-b")!;
      expect(pinB.checkpointId).toBe(memberBCheckpoint);
      expect(host.member("member-a")!.isDirty()).toBe(false);
    });

    it("studio checkout checkpoint fans out and restores pinned member state", () => {
      const memberA = newCounterStore("member-a");
      const host = newStudioHost();
      host.registerMember(memberA);

      memberA.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      const studioCheckpoint1 = host.commitStudioCheckpoint("first", []);

      memberA.dispatch({ kind: "apply", operations: [{ op: "setN", n: 2 }] });
      host.commitStudioCheckpoint("second", []);
      expect(memberA.projection().n).toBe(2);

      host.checkoutStudioCheckpoint(studioCheckpoint1);
      expect(memberA.projection().n).toBe(1);
    });

    it("studio switch alternative fans out and restores pinned member state", () => {
      const memberA = newCounterStore("member-a");
      const host = newStudioHost();
      host.registerMember(memberA);

      memberA.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1 }] });
      host.commitStudioCheckpoint("root", []);

      const altId = host.createStudioAlternative("branch-a");

      memberA.dispatch({ kind: "apply", operations: [{ op: "setN", n: 2 }] });
      expect(memberA.projection().n).toBe(2);

      host.switchStudioAlternative(altId);
      expect(memberA.projection().n).toBe(1);
    });

    it("studio undo and redo target the member with the most recent local edit by HLT", () => {
      type TimestampedOp = { op: "setN"; n: number; physicalMs: number };
      const timestampedHandlers: OperationHandlers<CounterProjection, TimestampedOp> = {
        forwards: (op) => ({ n: op.n }),
        backwards: (_op, projection) => [{ op: "setN", n: projection.n, physicalMs: 0 }],
        timestamp: (op) => ({ actor: 0, physicalMs: op.physicalMs, logical: 0 }),
      };
      const memberEarly = new DocumentVcsStore<CounterProjection, TimestampedOp>(createDocumentVcsEnvelope("demo-ts/v1", "member-early", { n: 0 }), timestampedHandlers);
      memberEarly.dispatch({ kind: "apply", operations: [{ op: "setN", n: 1, physicalMs: 1_000 }] });

      const memberLate = new DocumentVcsStore<CounterProjection, TimestampedOp>(createDocumentVcsEnvelope("demo-ts/v1", "member-late", { n: 0 }), timestampedHandlers);
      memberLate.dispatch({ kind: "apply", operations: [{ op: "setN", n: 9, physicalMs: 2_000 }] });

      const host = newStudioHost();
      host.registerMember(memberEarly);
      host.registerMember(memberLate);

      host.undo();
      expect(memberEarly.projection().n).toBe(1);
      expect(memberLate.projection().n).toBe(0);

      host.redo();
      expect(memberLate.projection().n).toBe(9);
    });
  });
}
//#endregion 🧪Tests
