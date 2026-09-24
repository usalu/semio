// #region Header
/** 📌️ Closed browser twin of the hub-materialized Check In contract (`🦀️.rs`, `🔣️.json`). */
// #endregion Header

export const DOCUMENT_CHECK_IN_MAX_BYTES = 4096;
export const DOCUMENT_CHECK_IN_LABEL_MAX_CHARS = 256;
export const DOCUMENT_CHECK_IN_SCHEMA_V1 = "semio.hub.document-check-in/v1";
export const DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1 = "semio.hub.document-check-in-status/v1";
const MAX_SAFE = Number.MAX_SAFE_INTEGER;
const ID_MAX_BYTES = 256;

export type EditedArtifactFrontierV1 = Readonly<{
  documentId: string;
  headEditOrdinal: number;
  headEditId: string;
  lastCommitSeq: number;
  chainSha256: string;
}>;

export type DocumentCheckInV1 = Readonly<{
  schema: typeof DOCUMENT_CHECK_IN_SCHEMA_V1;
  requestId: string;
  head: EditedArtifactFrontierV1;
  label?: string;
}>;

export type DocumentCheckInPhaseV1 = "accepted" | "materializing" | "publishing" | "ready" | "failed" | "cancelled";
export type DocumentCheckInRefusalV1 = "stale-head" | "behind-active-checkpoint" | "active-checkpoint-changed" | "ledger-not-replayable" | "codec-refused" | "authority-changed" | "unavailable";

export type DocumentCheckInReadyV1 = Readonly<{
  checkpointId: string;
  parentCheckpointId: string;
  baseline: EditedArtifactFrontierV1;
}>;

export type DocumentCheckInStatusV1 = Readonly<{
  schema: typeof DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1;
  requestId: string;
  phase: DocumentCheckInPhaseV1;
  progress: Readonly<{ completedUnits: number; totalUnits: number }>;
  label?: string;
  ready?: DocumentCheckInReadyV1;
  refusal?: DocumentCheckInRefusalV1;
}>;

const PHASES: ReadonlySet<string> = new Set(["accepted", "materializing", "publishing", "ready", "failed", "cancelled"]);
const REFUSALS: ReadonlySet<string> = new Set(["stale-head", "behind-active-checkpoint", "active-checkpoint-changed", "ledger-not-replayable", "codec-refused", "authority-changed", "unavailable"]);
const encoder = new TextEncoder();

function record(value: unknown): Readonly<Record<string, unknown>> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? (value as Readonly<Record<string, unknown>>) : null;
}

function exactKeys(value: Readonly<Record<string, unknown>>, required: readonly string[], optional: readonly string[] = []): boolean {
  const keys = Object.keys(value);
  return required.every((key) => key in value) && keys.every((key) => required.includes(key) || optional.includes(key));
}

function requestId(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{32}$/.test(value) && /[1-9a-f]/.test(value);
}

function hash(value: unknown): value is string {
  return typeof value === "string" && /^[0-9a-f]{64}$/.test(value) && /[1-9a-f]/.test(value);
}

function text(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && encoder.encode(value).length <= ID_MAX_BYTES && !/[\u0000-\u001f\u007f-\u009f]/.test(value);
}

function label(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && [...value].length <= DOCUMENT_CHECK_IN_LABEL_MAX_CHARS && value.trim() === value && !/[\u0000-\u001f\u007f-\u009f]/.test(value);
}

function positive(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 1 && value <= MAX_SAFE;
}

function natural(value: unknown): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= 0 && value <= MAX_SAFE;
}

/** 🌊️ An edited, committed ledger head in its exact wire grammar. */
export function isEditedArtifactFrontierV1(value: unknown): value is EditedArtifactFrontierV1 {
  const frontier = record(value);
  return (
    frontier !== null &&
    exactKeys(frontier, ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainSha256"]) &&
    text(frontier.documentId) &&
    positive(frontier.headEditOrdinal) &&
    text(frontier.headEditId) &&
    positive(frontier.lastCommitSeq) &&
    hash(frontier.chainSha256)
  );
}

function canonicalFrontier(frontier: EditedArtifactFrontierV1): EditedArtifactFrontierV1 {
  return { documentId: frontier.documentId, headEditOrdinal: frontier.headEditOrdinal, headEditId: frontier.headEditId, lastCommitSeq: frontier.lastCommitSeq, chainSha256: frontier.chainSha256 };
}

/** 📤️ The canonical request body, or a thrown error for a request the hub would refuse unread. */
export function documentCheckInCanonicalJson(request: DocumentCheckInV1): string {
  const canonical = { schema: request.schema, requestId: request.requestId, head: canonicalFrontier(request.head), ...(request.label === undefined ? {} : { label: request.label }) };
  if (canonical.schema !== DOCUMENT_CHECK_IN_SCHEMA_V1 || !requestId(canonical.requestId) || !isEditedArtifactFrontierV1(canonical.head) || (canonical.label !== undefined && !label(canonical.label))) throw new Error("document check-in: invalid request");
  const source = JSON.stringify(canonical);
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) throw new Error("document check-in: request exceeds its byte ceiling");
  return source;
}

/** 📥️ One exact canonical request; `null` for padding, reordering, overposting or bounds. */
export function parseDocumentCheckInV1(source: string): DocumentCheckInV1 | null {
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) return null;
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch {
    return null;
  }
  const value = record(parsed);
  if (value === null || !exactKeys(value, ["schema", "requestId", "head"], ["label"]) || value.schema !== DOCUMENT_CHECK_IN_SCHEMA_V1 || !requestId(value.requestId) || !isEditedArtifactFrontierV1(value.head) || (value.label !== undefined && !label(value.label))) return null;
  const request = value as DocumentCheckInV1;
  return documentCheckInCanonicalJson(request) === source ? request : null;
}

function ready(value: unknown): value is DocumentCheckInReadyV1 {
  const candidate = record(value);
  return candidate !== null && exactKeys(candidate, ["checkpointId", "parentCheckpointId", "baseline"]) && hash(candidate.checkpointId) && hash(candidate.parentCheckpointId) && isEditedArtifactFrontierV1(candidate.baseline);
}

function canonicalStatus(status: DocumentCheckInStatusV1): string {
  return JSON.stringify({
    schema: status.schema,
    requestId: status.requestId,
    phase: status.phase,
    progress: { completedUnits: status.progress.completedUnits, totalUnits: status.progress.totalUnits },
    ...(status.label === undefined ? {} : { label: status.label }),
    ...(status.ready === undefined ? {} : { ready: { checkpointId: status.ready.checkpointId, parentCheckpointId: status.ready.parentCheckpointId, baseline: canonicalFrontier(status.ready.baseline) } }),
    ...(status.refusal === undefined ? {} : { refusal: status.refusal }),
  });
}

/** 🧾️ One exact hub status: only Ready names a checkpoint, only Failed a refusal, progress never overruns. */
export function parseDocumentCheckInStatusV1(source: string): DocumentCheckInStatusV1 | null {
  if (encoder.encode(source).length > DOCUMENT_CHECK_IN_MAX_BYTES) return null;
  let parsed: unknown;
  try {
    parsed = JSON.parse(source);
  } catch {
    return null;
  }
  const value = record(parsed);
  const progress = record(value?.progress);
  if (
    value === null ||
    progress === null ||
    !exactKeys(value, ["schema", "requestId", "phase", "progress"], ["label", "ready", "refusal"]) ||
    !exactKeys(progress, ["completedUnits", "totalUnits"]) ||
    value.schema !== DOCUMENT_CHECK_IN_STATUS_SCHEMA_V1 ||
    !requestId(value.requestId) ||
    typeof value.phase !== "string" ||
    !PHASES.has(value.phase) ||
    !natural(progress.completedUnits) ||
    !natural(progress.totalUnits) ||
    progress.completedUnits > progress.totalUnits ||
    (value.label !== undefined && !label(value.label)) ||
    (value.ready !== undefined && !ready(value.ready)) ||
    (value.refusal !== undefined && (typeof value.refusal !== "string" || !REFUSALS.has(value.refusal))) ||
    (value.phase === "ready") !== (value.ready !== undefined) ||
    (value.phase === "failed") !== (value.refusal !== undefined)
  )
    return null;
  const status = value as DocumentCheckInStatusV1;
  return canonicalStatus(status) === source ? status : null;
}
