import { type EditedArtifactFrontierV1, isEditedArtifactFrontierV1 as editedArtifactFrontierIsValid } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/📌️document-check-in-v1/🟦️.ts";

function documentOpenObject(value: unknown, required: readonly string[], optional: readonly string[] = []): Record<string, unknown> {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new Error("document-open.invalid-object");
  const object = value as Record<string, unknown>;
  const accepted = new Set([...required, ...optional]);
  // 🩺️ The refusal names the keys it refused on. This check is the closed-object gate for the plan
  // and for every object nested inside it, and a bare code left a caller unable to tell an added
  // field from a missing one, in which object — including the common case where the body is not a
  // plan at all but a two-field `document-open-plan-error/v1`, whose `code` is the real answer.
  const unexpected = Object.keys(object).filter((key) => !accepted.has(key));
  const missing = required.filter((key) => !(key in object));
  if (unexpected.length !== 0 || missing.length !== 0) throw new Error(`document-open.invalid-fields unexpected=[${unexpected.join(",")}] missing=[${missing.join(",")}] accepted=[${[...accepted].join(",")}]`);
  return object;
}

function documentOpenText(value: unknown, maxBytes = 256): string {
  if (typeof value !== "string" || value.length === 0 || new TextEncoder().encode(value).length > maxBytes || /\p{Cc}/u.test(value)) throw new Error("document-open.invalid-text");
  return value;
}

function documentOpenInteger(value: unknown, positive = false): number {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < (positive ? 1 : 0)) throw new Error("document-open.invalid-integer");
  return value;
}

function editedArtifactFrontier(value: unknown): EditedArtifactFrontierV1 {
  if (!editedArtifactFrontierIsValid(value)) throw new Error("edited-artifact-frontier.invalid");
  return { documentId: value.documentId, headEditOrdinal: value.headEditOrdinal, headEditId: value.headEditId, lastCommitSeq: value.lastCommitSeq, chainSha256: value.chainSha256 };
}

//#region 💡️InferencePort
/** 🧯️ Exact maximum accepted bytes for one inference request or approval body — the hub's own
 * `REQUEST_MAX_BYTES`, shared verbatim by every transport so a client can never post a body the
 * route would only reject. */
export const GIS_MAP_INFERENCE_REQUEST_MAX_BYTES = 1024;
/** 🧯️ Exact maximum accepted bytes for one bounded owner-private response body. */
export const GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES = 16 * 1024;
/** 📈️ Highest progress cursor the hub's append-only bounded progress table admits. */
export const GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR = 16;
/** 📃️ Highest number of lifecycle events one owner-private page may carry. */
export const GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS = 8;
/** ⏳️ Highest job lifetime the hub admits for one submitted job. */
export const GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS = 120_000;
/** 🔖️ The one inference service the GIS Map port may name. */
export const GIS_MAP_INFERENCE_SERVICE_ID = "s.gis.gismap.inference";

/** 📤️ The closed client intent one submit carries. It names a service and a lifetime and nothing
 * else: no model, provider, prompt, budget, base pack or proposal ever crosses this boundary. */
export interface GisMapInferenceJobRequestV1 {
  schema: "semio.hub.inference-request/v1";
  version: 1;
  requestId: string;
  serviceId: typeof GIS_MAP_INFERENCE_SERVICE_ID;
  policyVersion: 1;
  lifetimeMs: number;
}

/** ✅️ The closed body one approval carries: the offered job and the exact hash the server itself
 * published. A client never computes this hash — it only echoes back what `offered` reported. */
export interface GisMapInferenceApprovalRequestV1 {
  schema: "semio.hub.inference-approval/v1";
  version: 1;
  jobId: string;
  proposalHash: string;
}

/** 🖥️ The hub's own job lifecycle vocabulary, mirrored exactly. */
export type GisMapInferenceJobStateV1 = "accepted" | "running" | "succeeded" | "failed" | "cancelled";
/** 🖥️ The hub's own proposal lifecycle vocabulary, mirrored exactly. */
export type GisMapInferenceProposalStateV1 = "none" | "offered" | "approved" | "stale" | "cancelled";

/** 🧾️ The closed receipt one accepted submit returns; it never carries private result or base bytes. */
export interface GisMapInferenceJobReceiptV1 {
  schema: "semio.hub.inference-job-receipt/v1";
  jobId: string;
  state: GisMapInferenceJobStateV1;
  proposalState: GisMapInferenceProposalStateV1;
  proposalHash: string | null;
  cursor: number;
  expiresAtMs: number;
}

/** 📈️ One owner-private progress row. */
export interface GisMapInferenceProgressV1 {
  cursor: number;
  runEpoch: number;
  completed: number;
  total: number;
  atMs: number;
}

/** 🗓️ One owner-private lifecycle event. */
export interface GisMapInferenceEventV1 {
  ordinal: number;
  kind: string;
  atMs: number;
}

/** 🗺️ Hub-validated, owner-private bounds geometry safe for host review. */
export interface GisMapInferencePreviewV1 {
  schema: "semio.hub.gis-map-inference-preview/v1";
  jobId: string;
  proposalHash: string;
  regionId: string;
  ring: readonly [readonly [number, number], readonly [number, number], readonly [number, number], readonly [number, number], readonly [number, number]];
}

/** 📃️ The owner-private bounded page one events, cancel or poll read returns. */
export interface GisMapInferenceEventPageV1 {
  schema: "semio.hub.inference-job-events/v1";
  jobId: string;
  state: GisMapInferenceJobStateV1;
  proposalState: GisMapInferenceProposalStateV1;
  cancelRequested: boolean;
  stale: boolean;
  proposalHash: string | null;
  preview?: GisMapInferencePreviewV1;
  events: readonly GisMapInferenceEventV1[];
  progress: readonly GisMapInferenceProgressV1[];
  nextCursor: number;
}

/** ✅️ The closed approval outcome; `applied` is true only after a real committed-WAL witness. */
export interface GisMapInferenceApprovalReceiptV1 {
  schema: "semio.hub.inference-approval-receipt/v1";
  jobId: string;
  mutationId: string;
  commandHash: string;
  proposalHash: string;
  applied: boolean;
  undo: GisMapApprovalUndoHandleV1;
}

/** ↩️ Owner-bound durable undo locator minted only from a committed GIS approval witness. */
export interface GisMapApprovalUndoHandleV1 {
  targetId: string;
  expectedCurrent: EditedArtifactFrontierV1;
}

/** 📨️ Closed undo intent; inverse bytes never cross this boundary. */
export interface GisMapApprovalUndoRequestV1 {
  schema: "semio.hub.gis-map-approval-undo/v1";
  version: 1;
  targetId: string;
  idempotencyKey: string;
  expectedCurrent: EditedArtifactFrontierV1;
}

/** 🧾️ Durable inverse receipt published only after a second verified WAL decision. */
export interface GisMapApprovalUndoReceiptV1 {
  schema: "semio.hub.gis-map-approval-undo-receipt/v1";
  targetId: string;
  originalJobId: string;
  mutationId: string;
  commandHash: string;
  applied: boolean;
  replayed: boolean;
  frontier: EditedArtifactFrontierV1;
}

/** 🚦️ The complete published failure vocabulary the four authenticated routes may answer with,
 * plus the two the client itself may reach: `transport` for an indeterminate call and
 * `lease-unverified` for a port refused before any request existed. */
export type GisMapInferencePortCodeV1 =
  | "inference.unavailable"
  | "inference.denied"
  | "inference.not-found"
  | "inference.invalid"
  | "inference.bounds"
  | "inference.conflict"
  | "inference.capacity"
  | "inference.expired"
  | "inference.cancelled"
  | "approval.commit-unavailable"
  | "inference.storage"
  | "inference.transport"
  | "inference.lease-unverified";

/** 💡️ The complete rendered lifecycle of one host-owned ephemeral inference port. `idle` and
 * `submitting` have no server counterpart at all (nothing has been accepted yet), `approving`
 * corresponds to the server's `approval-prepared`, and the four terminals are exactly the packet's
 * `applied | cancelled | stale | failed`. */
export type GisMapInferencePortPhaseV1 = "idle" | "submitting" | "running" | "offered" | "approving" | "indeterminate" | "applied" | "cancelled" | "stale" | "failed";

/** 💡️ Complete renderer-visible state of one document's port. It carries a phase, the server's own
 * job id, its bounded progress cursor, the hash the server published, whether a cancel was
 * requested, and one closed failure code — never a receipt, bearer, origin, path, base pack,
 * proposal body or user identity, and never anything persisted into the document. */
export interface GisMapInferencePortStatusV1 {
  phase: GisMapInferencePortPhaseV1;
  jobId: string | null;
  cursor: number;
  completed: number;
  total: number;
  proposalHash: string | null;
  preview?: GisMapInferencePreviewV1;
  cancelRequested: boolean;
  code: GisMapInferencePortCodeV1 | null;
}

/** 🎬️ Every input the port's state machine accepts. `start`/`approve`/`cancel` are operator
 * intents, `receipt`/`page`/`approval` are exact server answers, `lease-unverified` is the
 * precondition refusal, `failed` is one closed transport/route rejection, and `clear` retires the
 * port. Nothing else can move a phase. */
export type GisMapInferencePortEventV1 =
  | { kind: "start" }
  | { kind: "lease-unverified" }
  | { kind: "receipt"; receipt: GisMapInferenceJobReceiptV1 }
  | { kind: "page"; page: GisMapInferenceEventPageV1 }
  | { kind: "approve" }
  | { kind: "approval"; receipt: GisMapInferenceApprovalReceiptV1 }
  | { kind: "cancel" }
  | { kind: "indeterminate"; code: GisMapInferencePortCodeV1 }
  | { kind: "failed"; code: GisMapInferencePortCodeV1 }
  | { kind: "clear" };

/** 💤️ The one starting value; a document with no port has exactly this. */
export function idleGisMapInferencePortStatusV1(): GisMapInferencePortStatusV1 {
  return { phase: "idle", jobId: null, cursor: 0, completed: 0, total: 0, proposalHash: null, cancelRequested: false, code: null };
}

/** 🏁️ A terminal phase accepts no further server answer — only an explicit `clear`. */
export function gisMapInferencePortTerminalV1(phase: GisMapInferencePortPhaseV1): boolean {
  return phase === "applied" || phase === "cancelled" || phase === "stale" || phase === "failed";
}

/** 🔊️ ARIA live-region politeness: work in flight announces politely, every terminal asserts. */
export function gisMapInferencePortRoleV1(phase: GisMapInferencePortPhaseV1): "status" | "alert" {
  return gisMapInferencePortTerminalV1(phase) ? "alert" : "status";
}

/** 🗺️ Equirectangular host overlay of one offered preview ring. Inspection only; never document state. */
export type GisMapInferencePreviewOverlayV1 = {
  readonly regionId: string;
  readonly viewBox: "0 0 100 100";
  readonly path: string;
};

export type GisMapInferencePortAffordancesV1 = {
  readonly request: boolean;
  readonly cancel: boolean;
  readonly reject: boolean;
  readonly approve: boolean;
  readonly overlay: boolean;
};

function gisMapInferenceOverlayCoordV1(value: number): string {
  return Number.isInteger(value) ? String(value) : String(value);
}

/** 🗺️ Projects the server ring onto a closed SVG path in a 100×100 viewBox. */
export function projectGisMapInferencePreviewOverlayV1(preview: GisMapInferencePreviewV1): GisMapInferencePreviewOverlayV1 {
  const lons = preview.ring.map((point) => point[0]);
  const lats = preview.ring.map((point) => point[1]);
  const minLon = Math.min(...lons);
  const maxLon = Math.max(...lons);
  const minLat = Math.min(...lats);
  const maxLat = Math.max(...lats);
  const lonSpan = maxLon - minLon || 1;
  const latSpan = maxLat - minLat || 1;
  const points = preview.ring.map(([lon, lat]) => `${gisMapInferenceOverlayCoordV1(((lon - minLon) / lonSpan) * 100)} ${gisMapInferenceOverlayCoordV1(((maxLat - lat) / latSpan) * 100)}`);
  return { regionId: preview.regionId, viewBox: "0 0 100 100", path: `M ${points.join(" L ")} Z` };
}

/** 🎛️ Closed host chrome for one port status. Reject and approve share the offered preview gate. */
export function gisMapInferencePortAffordancesV1(status: GisMapInferencePortStatusV1): GisMapInferencePortAffordancesV1 {
  const terminal = gisMapInferencePortTerminalV1(status.phase);
  const previewOk = status.preview !== undefined && status.preview.proposalHash === status.proposalHash && status.preview.jobId === status.jobId;
  return {
    request: status.phase === "idle",
    cancel: !terminal && status.phase !== "idle" && status.phase !== "offered" && !status.cancelRequested,
    reject: status.phase === "offered" && previewOk && !status.cancelRequested,
    approve: status.phase === "offered" && status.proposalHash !== null && previewOk && !status.cancelRequested,
    overlay: (status.phase === "offered" || status.phase === "approving") && previewOk,
  };
}

/** 🗺️ Projects one exact server page onto a rendered phase. `stale` outranks everything (the base
 * the job was accepted against is gone), then the job's own terminal states, then the proposal's. */
function gisMapInferenceServerPhaseV1(page: GisMapInferenceEventPageV1): GisMapInferencePortPhaseV1 {
  if (page.stale || page.proposalState === "stale") return "stale";
  if (page.state === "cancelled" || page.proposalState === "cancelled") return "cancelled";
  if (page.state === "failed") return "failed";
  if (page.proposalState === "approved") return "applied";
  if (page.proposalState === "offered" || page.state === "succeeded") return "offered";
  return "running";
}

function gisMapInferenceWithoutPreviewV1(status: GisMapInferencePortStatusV1): Omit<GisMapInferencePortStatusV1, "preview"> {
  const { preview: _preview, ...rest } = status;
  return rest;
}

/** 🧮️ Total, pure transition. It never fabricates a phase the server has not reported: `submitting`
 * is only left on an exact receipt, `cancelled` only on an exact server answer (a Cancel click is
 * recorded as `cancelRequested`, never as an optimistic terminal), `approving` is only reachable
 * from `offered`, and an answer for a different job id or after a terminal is ignored outright. */
export function reduceGisMapInferencePortV1(current: GisMapInferencePortStatusV1, event: GisMapInferencePortEventV1): GisMapInferencePortStatusV1 {
  if (event.kind === "clear") return idleGisMapInferencePortStatusV1();
  if (gisMapInferencePortTerminalV1(current.phase)) return current;
  switch (event.kind) {
    case "start":
      return current.phase === "idle" ? { ...idleGisMapInferencePortStatusV1(), phase: "submitting" } : current;
    case "lease-unverified":
      return current.phase === "idle" || current.phase === "submitting" ? { ...current, phase: "failed", code: "inference.lease-unverified" } : current;
    case "receipt": {
      if (current.phase !== "submitting" && !(current.phase === "indeterminate" && current.jobId === null)) return current;
      const page: GisMapInferenceEventPageV1 = {
        schema: "semio.hub.inference-job-events/v1",
        jobId: event.receipt.jobId,
        state: event.receipt.state,
        proposalState: event.receipt.proposalState,
        cancelRequested: false,
        stale: false,
        proposalHash: event.receipt.proposalHash,
        events: [],
        progress: [],
        nextCursor: event.receipt.cursor,
      };
      return { ...gisMapInferenceWithoutPreviewV1(current), phase: gisMapInferenceServerPhaseV1(page), jobId: event.receipt.jobId, cursor: event.receipt.cursor, proposalHash: event.receipt.proposalHash ?? null, code: null };
    }
    case "page": {
      if (current.jobId === null || current.jobId !== event.page.jobId) return current;
      const latest = event.page.progress.at(-1);
      const server = gisMapInferenceServerPhaseV1(event.page);
      const phase = current.phase === "approving" && !gisMapInferencePortTerminalV1(server) ? "approving" : server;
      return {
        ...gisMapInferenceWithoutPreviewV1(current),
        phase,
        cursor: Math.max(current.cursor, event.page.nextCursor),
        completed: latest?.completed ?? current.completed,
        total: latest?.total ?? current.total,
        proposalHash: event.page.proposalHash ?? null,
        ...((phase === "offered" || phase === "approving") && event.page.preview !== undefined ? { preview: event.page.preview } : {}),
        cancelRequested: current.cancelRequested || event.page.cancelRequested,
        code: phase === "failed" ? "inference.storage" : null,
      };
    }
    case "approve":
      return current.phase === "offered" && current.proposalHash !== null && current.preview?.proposalHash === current.proposalHash && current.preview.jobId === current.jobId && !current.cancelRequested ? { ...current, phase: "approving" } : current;
    case "approval": {
      if ((current.phase !== "approving" && current.phase !== "indeterminate") || current.jobId !== event.receipt.jobId || current.proposalHash !== event.receipt.proposalHash) return current;
      const withoutPreview = gisMapInferenceWithoutPreviewV1(current);
      return event.receipt.applied ? { ...withoutPreview, phase: "applied", code: null } : { ...withoutPreview, phase: "failed", code: "approval.commit-unavailable" };
    }
    case "cancel":
      return current.phase === "idle" ? current : { ...current, cancelRequested: true };
    case "indeterminate":
      return { ...gisMapInferenceWithoutPreviewV1(current), phase: "indeterminate", code: event.code };
    case "failed":
      return { ...gisMapInferenceWithoutPreviewV1(current), phase: event.code === "inference.cancelled" ? "cancelled" : "failed", code: event.code };
  }
}

/** 🌐️ Complete localized phase text. EN and DE are both explicit; there is no default language and
 * no fallback. No string carries an origin, path, receipt, digest or user identity. */
export const GIS_MAP_INFERENCE_PORT_TEXT_V1: Readonly<Record<GisMapInferencePortPhaseV1, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  idle: Object.freeze({ en: "No proposal requested.", de: "Kein Vorschlag angefordert." }),
  submitting: Object.freeze({ en: "Requesting a bounds proposal…", de: "Begrenzungsvorschlag wird angefordert…" }),
  running: Object.freeze({ en: "Computing the bounds proposal…", de: "Begrenzungsvorschlag wird berechnet…" }),
  offered: Object.freeze({ en: "A bounds proposal is ready for review.", de: "Ein Begrenzungsvorschlag liegt zur Prüfung bereit." }),
  approving: Object.freeze({ en: "Waiting for the server to commit the approved proposal…", de: "Warten auf die Freigabe des Vorschlags durch den Server…" }),
  indeterminate: Object.freeze({ en: "The outcome is unknown. The original request is retained while its server state is checked.", de: "Das Ergebnis ist unbekannt. Die ursprüngliche Anfrage bleibt erhalten, während ihr Serverstatus geprüft wird." }),
  applied: Object.freeze({ en: "The approved proposal was committed to the document.", de: "Der freigegebene Vorschlag wurde im Dokument übernommen." }),
  cancelled: Object.freeze({ en: "The proposal was cancelled.", de: "Der Vorschlag wurde abgebrochen." }),
  stale: Object.freeze({ en: "The document changed while the proposal ran. Request a new one.", de: "Das Dokument hat sich während des Vorschlags geändert. Fordern Sie einen neuen an." }),
  failed: Object.freeze({ en: "The proposal did not complete.", de: "Der Vorschlag wurde nicht abgeschlossen." }),
});

/** 🌐️ Complete localized failure text, one entry per published code, EN and DE both explicit. */
export const GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1: Readonly<Record<GisMapInferencePortCodeV1, Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  "inference.unavailable": Object.freeze({ en: "Proposals are unavailable for this document.", de: "Für dieses Dokument sind keine Vorschläge verfügbar." }),
  "inference.denied": Object.freeze({ en: "You may not request proposals for this document.", de: "Sie dürfen für dieses Dokument keine Vorschläge anfordern." }),
  "inference.not-found": Object.freeze({ en: "This proposal no longer exists.", de: "Dieser Vorschlag existiert nicht mehr." }),
  "inference.invalid": Object.freeze({ en: "The request was rejected as malformed.", de: "Die Anfrage wurde als fehlerhaft abgelehnt." }),
  "inference.bounds": Object.freeze({ en: "The request exceeded its accepted size.", de: "Die Anfrage hat die zulässige Größe überschritten." }),
  "inference.conflict": Object.freeze({ en: "The document changed; request a new proposal.", de: "Das Dokument hat sich geändert; fordern Sie einen neuen Vorschlag an." }),
  "inference.capacity": Object.freeze({ en: "Too many proposals are running. Try again shortly.", de: "Es laufen zu viele Vorschläge. Versuchen Sie es in Kürze erneut." }),
  "inference.expired": Object.freeze({ en: "This proposal expired before it was approved.", de: "Dieser Vorschlag ist vor der Freigabe abgelaufen." }),
  "inference.cancelled": Object.freeze({ en: "The proposal was cancelled.", de: "Der Vorschlag wurde abgebrochen." }),
  "approval.commit-unavailable": Object.freeze({ en: "The approved proposal could not be committed and was not applied.", de: "Der freigegebene Vorschlag konnte nicht übernommen werden und wurde nicht angewendet." }),
  "inference.storage": Object.freeze({ en: "The proposal service is temporarily unavailable.", de: "Der Vorschlagsdienst ist vorübergehend nicht verfügbar." }),
  "inference.transport": Object.freeze({ en: "The outcome is unknown. Close retries checking the original request without submitting another.", de: "Das Ergebnis ist unbekannt. Schließen prüft die ursprüngliche Anfrage erneut, ohne eine weitere zu senden." }),
  "inference.lease-unverified": Object.freeze({ en: "This document has no verified execution target, so no proposal can start.", de: "Dieses Dokument hat kein verifiziertes Ausführungsziel, daher kann kein Vorschlag starten." }),
});

/** 🌐️ Complete localized control and region labels, EN and DE both explicit. */
export const GIS_MAP_INFERENCE_PORT_CONTROL_TEXT_V1: Readonly<Record<"heading" | "request" | "cancel" | "reject" | "approve" | "close" | "progress" | "region" | "longitude" | "latitude" | "overlay", Readonly<Record<"en" | "de", string>>>> = Object.freeze({
  heading: Object.freeze({ en: "Bounds proposal", de: "Begrenzungsvorschlag" }),
  request: Object.freeze({ en: "Request bounds proposal", de: "Begrenzungsvorschlag anfordern" }),
  cancel: Object.freeze({ en: "Cancel proposal", de: "Vorschlag abbrechen" }),
  reject: Object.freeze({ en: "Reject proposal", de: "Vorschlag ablehnen" }),
  approve: Object.freeze({ en: "Approve proposal", de: "Vorschlag freigeben" }),
  close: Object.freeze({ en: "Close proposal", de: "Vorschlag schließen" }),
  progress: Object.freeze({ en: "Proposal progress", de: "Fortschritt des Vorschlags" }),
  region: Object.freeze({ en: "Region", de: "Gebiet" }),
  longitude: Object.freeze({ en: "Longitude extent", de: "Längengradbereich" }),
  latitude: Object.freeze({ en: "Latitude extent", de: "Breitengradbereich" }),
  overlay: Object.freeze({ en: "Proposed bounds on the map", de: "Vorgeschlagene Grenzen auf der Karte" }),
});

function gisMapInferenceHex(value: unknown, length: number): string {
  if (typeof value !== "string" || value.length !== length || !/^[0-9a-f]+$/u.test(value)) throw new Error("gis-map-inference.invalid-hex");
  return value;
}

function gisMapInferenceJobState(value: unknown): GisMapInferenceJobStateV1 {
  if (value !== "accepted" && value !== "running" && value !== "succeeded" && value !== "failed" && value !== "cancelled") throw new Error("gis-map-inference.invalid-state");
  return value;
}

function gisMapInferenceProposalState(value: unknown): GisMapInferenceProposalStateV1 {
  if (value !== "none" && value !== "offered" && value !== "approved" && value !== "stale" && value !== "cancelled") throw new Error("gis-map-inference.invalid-proposal-state");
  return value;
}

/** 🗺️ Strictly decodes one bounded rectangular preview and rejects any substituted shape. */
export function parseGisMapInferencePreviewV1(value: unknown): GisMapInferencePreviewV1 {
  const object = documentOpenObject(value, ["schema", "jobId", "proposalHash", "regionId", "ring"]);
  const jobId = gisMapInferenceHex(object.jobId, 32);
  const proposalHash = gisMapInferenceHex(object.proposalHash, 64);
  if (object.schema !== "semio.hub.gis-map-inference-preview/v1" || object.regionId !== `inference-${jobId}` || !Array.isArray(object.ring) || object.ring.length !== 5) throw new Error("gis-map-inference.invalid-preview");
  const ring = object.ring.map((point) => {
    if (!Array.isArray(point) || point.length !== 2 || point.some((coordinate) => typeof coordinate !== "number" || !Number.isFinite(coordinate))) throw new Error("gis-map-inference.invalid-preview");
    return [point[0] as number, point[1] as number] as const;
  }) as unknown as GisMapInferencePreviewV1["ring"];
  const [lonMin, latMin] = ring[0];
  const [lonMax, latMax] = ring[2];
  if (
    lonMin < -180 ||
    lonMax > 180 ||
    latMin < -90 ||
    latMax > 90 ||
    lonMin > lonMax ||
    latMin > latMax ||
    ring.some((point, index) => point[0] !== [lonMin, lonMax, lonMax, lonMin, lonMin][index] || point[1] !== [latMin, latMin, latMax, latMax, latMin][index])
  )
    throw new Error("gis-map-inference.invalid-preview");
  return { schema: object.schema, jobId, proposalHash, regionId: object.regionId, ring };
}

/** 💡️ Strictly decodes the private worker-to-host status projection. */
export function parseGisMapInferencePortStatusV1(value: unknown): GisMapInferencePortStatusV1 {
  const object = documentOpenObject(value, ["phase", "jobId", "cursor", "completed", "total", "proposalHash", "cancelRequested", "code"], ["preview"]);
  const phases: readonly GisMapInferencePortPhaseV1[] = ["idle", "submitting", "running", "offered", "approving", "indeterminate", "applied", "cancelled", "stale", "failed"];
  const codes: readonly GisMapInferencePortCodeV1[] = [
    "inference.unavailable",
    "inference.denied",
    "inference.not-found",
    "inference.invalid",
    "inference.bounds",
    "inference.conflict",
    "inference.capacity",
    "inference.expired",
    "inference.cancelled",
    "approval.commit-unavailable",
    "inference.storage",
    "inference.transport",
    "inference.lease-unverified",
  ];
  if (
    !phases.includes(object.phase as GisMapInferencePortPhaseV1) ||
    typeof object.cancelRequested !== "boolean" ||
    (object.jobId !== null && typeof object.jobId !== "string") ||
    (object.proposalHash !== null && typeof object.proposalHash !== "string") ||
    (object.preview !== undefined && (typeof object.preview !== "object" || object.preview === null)) ||
    (object.code !== null && !codes.includes(object.code as GisMapInferencePortCodeV1))
  )
    throw new Error("gis-map-inference.invalid-status");
  const status: GisMapInferencePortStatusV1 = {
    phase: object.phase as GisMapInferencePortPhaseV1,
    jobId: object.jobId === null ? null : gisMapInferenceHex(object.jobId, 32),
    cursor: documentOpenInteger(object.cursor),
    completed: documentOpenInteger(object.completed),
    total: documentOpenInteger(object.total, true),
    proposalHash: object.proposalHash === null ? null : gisMapInferenceHex(object.proposalHash, 64),
    ...(object.preview === undefined ? {} : { preview: parseGisMapInferencePreviewV1(object.preview) }),
    cancelRequested: object.cancelRequested,
    code: object.code as GisMapInferencePortCodeV1 | null,
  };
  if (
    status.cursor > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR ||
    status.completed > status.total ||
    (status.preview !== undefined && ((status.phase !== "offered" && status.phase !== "approving") || status.jobId !== status.preview.jobId || status.proposalHash !== status.preview.proposalHash))
  )
    throw new Error("gis-map-inference.invalid-status");
  return status;
}

/** 📤️ Seals one submit intent under the exact route bound; an oversized body never leaves. */
export function sealGisMapInferenceJobRequestV1(requestId: string, lifetimeMs: number): GisMapInferenceJobRequestV1 {
  const request: GisMapInferenceJobRequestV1 = {
    schema: "semio.hub.inference-request/v1",
    version: 1,
    requestId: gisMapInferenceHex(requestId, 32),
    serviceId: GIS_MAP_INFERENCE_SERVICE_ID,
    policyVersion: 1,
    lifetimeMs: documentOpenInteger(lifetimeMs, true),
  };
  if (request.lifetimeMs > GIS_MAP_INFERENCE_JOB_MAX_LIFETIME_MS) throw new Error("gis-map-inference.invalid-lifetime");
  if (new TextEncoder().encode(JSON.stringify(request)).length > GIS_MAP_INFERENCE_REQUEST_MAX_BYTES) throw new Error("gis-map-inference.oversized-request");
  return request;
}

/** ✅️ Seals one approval body; the hash is echoed, never computed here. */
export function sealGisMapInferenceApprovalRequestV1(jobId: string, proposalHash: string): GisMapInferenceApprovalRequestV1 {
  const request: GisMapInferenceApprovalRequestV1 = { schema: "semio.hub.inference-approval/v1", version: 1, jobId: gisMapInferenceHex(jobId, 32), proposalHash: gisMapInferenceHex(proposalHash, 64) };
  if (new TextEncoder().encode(JSON.stringify(request)).length > GIS_MAP_INFERENCE_REQUEST_MAX_BYTES) throw new Error("gis-map-inference.oversized-request");
  return request;
}

/** 🧾️ Strictly parses one accepted-submit receipt. */
export function parseGisMapInferenceJobReceiptV1(value: unknown): GisMapInferenceJobReceiptV1 {
  const object = documentOpenObject(value, ["schema", "jobId", "state", "proposalState", "proposalHash", "cursor", "expiresAtMs"]);
  if (object.schema !== "semio.hub.inference-job-receipt/v1") throw new Error("gis-map-inference.invalid-receipt");
  const cursor = documentOpenInteger(object.cursor);
  if (cursor > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR) throw new Error("gis-map-inference.invalid-cursor");
  return {
    schema: object.schema,
    jobId: gisMapInferenceHex(object.jobId, 32),
    state: gisMapInferenceJobState(object.state),
    proposalState: gisMapInferenceProposalState(object.proposalState),
    proposalHash: object.proposalHash === null ? null : gisMapInferenceHex(object.proposalHash, 64),
    cursor,
    expiresAtMs: documentOpenInteger(object.expiresAtMs, true),
  };
}

/** 📃️ Strictly parses one bounded owner-private page, enforcing every published item/cursor bound
 * and a monotonically non-decreasing progress fold. */
export function parseGisMapInferenceEventPageV1(value: unknown): GisMapInferenceEventPageV1 {
  const object = documentOpenObject(value, ["schema", "jobId", "state", "proposalState", "cancelRequested", "stale", "proposalHash", "events", "progress", "nextCursor"], ["preview"]);
  if (object.schema !== "semio.hub.inference-job-events/v1") throw new Error("gis-map-inference.invalid-page");
  if (typeof object.cancelRequested !== "boolean" || typeof object.stale !== "boolean") throw new Error("gis-map-inference.invalid-flags");
  if (!Array.isArray(object.events) || object.events.length > GIS_MAP_INFERENCE_EVENT_PAGE_MAX_ITEMS) throw new Error("gis-map-inference.invalid-events");
  if (!Array.isArray(object.progress) || object.progress.length > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR) throw new Error("gis-map-inference.invalid-progress");
  const nextCursor = documentOpenInteger(object.nextCursor);
  if (nextCursor > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR) throw new Error("gis-map-inference.invalid-cursor");
  let ordinal = 0;
  const events = object.events.map((row) => {
    const event = documentOpenObject(row, ["ordinal", "kind", "atMs"]);
    const next = documentOpenInteger(event.ordinal, true);
    if (next <= ordinal) throw new Error("gis-map-inference.unordered-events");
    ordinal = next;
    return { ordinal: next, kind: documentOpenText(event.kind), atMs: documentOpenInteger(event.atMs, true) };
  });
  let cursor = 0;
  let completed = 0;
  const progress = object.progress.map((row) => {
    const item = documentOpenObject(row, ["cursor", "runEpoch", "completed", "total", "atMs"]);
    const nextCursorValue = documentOpenInteger(item.cursor, true);
    const nextCompleted = documentOpenInteger(item.completed);
    const total = documentOpenInteger(item.total, true);
    if (nextCursorValue <= cursor || nextCursorValue > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR || nextCompleted < completed || nextCompleted > total) throw new Error("gis-map-inference.invalid-progress");
    cursor = nextCursorValue;
    completed = nextCompleted;
    return { cursor: nextCursorValue, runEpoch: documentOpenInteger(item.runEpoch), completed: nextCompleted, total, atMs: documentOpenInteger(item.atMs, true) };
  });
  const jobId = gisMapInferenceHex(object.jobId, 32);
  const state = gisMapInferenceJobState(object.state);
  const proposalState = gisMapInferenceProposalState(object.proposalState);
  const proposalHash = object.proposalHash === null ? null : gisMapInferenceHex(object.proposalHash, 64);
  const preview = object.preview === undefined ? undefined : parseGisMapInferencePreviewV1(object.preview);
  if (preview !== undefined && (state !== "succeeded" || proposalState !== "offered" || object.cancelRequested || object.stale || proposalHash === null || preview.jobId !== jobId || preview.proposalHash !== proposalHash))
    throw new Error("gis-map-inference.invalid-preview-owner");
  return {
    schema: object.schema,
    jobId,
    state,
    proposalState,
    cancelRequested: object.cancelRequested,
    stale: object.stale,
    proposalHash,
    ...(preview === undefined ? {} : { preview }),
    events,
    progress,
    nextCursor,
  };
}

/** ✅️ Strictly parses one approval receipt. */
export function parseGisMapInferenceApprovalReceiptV1(value: unknown): GisMapInferenceApprovalReceiptV1 {
  const object = documentOpenObject(value, ["schema", "jobId", "mutationId", "commandHash", "proposalHash", "applied", "undo"]);
  if (object.schema !== "semio.hub.inference-approval-receipt/v1") throw new Error("gis-map-inference.invalid-approval-receipt");
  if (typeof object.applied !== "boolean") throw new Error("gis-map-inference.invalid-applied");
  const undo = documentOpenObject(object.undo, ["targetId", "expectedCurrent"]);
  return {
    schema: object.schema,
    jobId: gisMapInferenceHex(object.jobId, 32),
    mutationId: gisMapInferenceHex(object.mutationId, 32),
    commandHash: gisMapInferenceHex(object.commandHash, 64),
    proposalHash: gisMapInferenceHex(object.proposalHash, 64),
    applied: object.applied,
    undo: { targetId: gisMapInferenceHex(undo.targetId, 32), expectedCurrent: editedArtifactFrontier(undo.expectedCurrent) },
  };
}

/** 📥️ Strictly parses the client-authored undo request; no inverse bytes are accepted. */
export function parseGisMapApprovalUndoRequestV1(value: unknown): GisMapApprovalUndoRequestV1 {
  const object = documentOpenObject(value, ["schema", "version", "targetId", "idempotencyKey", "expectedCurrent"]);
  if (object.schema !== "semio.hub.gis-map-approval-undo/v1" || object.version !== 1) throw new Error("gis-map-approval-undo.invalid-envelope");
  return {
    schema: object.schema,
    version: object.version,
    targetId: gisMapInferenceHex(object.targetId, 32),
    idempotencyKey: gisMapInferenceHex(object.idempotencyKey, 32),
    expectedCurrent: editedArtifactFrontier(object.expectedCurrent),
  };
}

/** ↩️ Seals one worker-private durable undo intent from the Hub-minted handle and a stable
 * idempotency key. The page never supplies or receives either value. */
export function sealGisMapApprovalUndoRequestV1(handle: GisMapApprovalUndoHandleV1, idempotencyKey: string): GisMapApprovalUndoRequestV1 {
  return parseGisMapApprovalUndoRequestV1({ schema: "semio.hub.gis-map-approval-undo/v1", version: 1, targetId: handle.targetId, idempotencyKey, expectedCurrent: handle.expectedCurrent });
}

/** 🧾️ Strictly parses the server-minted durable undo receipt. */
export function parseGisMapApprovalUndoReceiptV1(value: unknown): GisMapApprovalUndoReceiptV1 {
  const object = documentOpenObject(value, ["schema", "targetId", "originalJobId", "mutationId", "commandHash", "applied", "replayed", "frontier"]);
  if (object.schema !== "semio.hub.gis-map-approval-undo-receipt/v1" || typeof object.applied !== "boolean" || typeof object.replayed !== "boolean") throw new Error("gis-map-approval-undo.invalid-receipt");
  return {
    schema: object.schema,
    targetId: gisMapInferenceHex(object.targetId, 32),
    originalJobId: gisMapInferenceHex(object.originalJobId, 32),
    mutationId: gisMapInferenceHex(object.mutationId, 32),
    commandHash: gisMapInferenceHex(object.commandHash, 64),
    applied: object.applied,
    replayed: object.replayed,
    frontier: editedArtifactFrontier(object.frontier),
  };
}

/** 🚦️ Maps one exact HTTP status onto the published failure vocabulary; an unmapped status is
 * indeterminate, never silently successful. */
export function gisMapInferenceCodeFromStatusV1(status: number): GisMapInferencePortCodeV1 {
  switch (status) {
    case 400:
      return "inference.invalid";
    case 403:
      return "inference.denied";
    case 404:
      return "inference.not-found";
    case 409:
      return "inference.conflict";
    case 410:
      return "inference.expired";
    case 413:
      return "inference.bounds";
    case 429:
      return "inference.capacity";
    case 503:
      return "inference.unavailable";
    default:
      return "inference.transport";
  }
}
//#endregion 💡️InferencePort

export type GisMapApprovalHistoryPhaseV1 = "unavailable" | "available" | "submitting" | "applied" | "failed";
export type GisMapApprovalHistoryStatusV1 = Readonly<{ phase: GisMapApprovalHistoryPhaseV1; canUndo: boolean; code: GisMapInferencePortCodeV1 | null }>;

export function parseGisMapApprovalHistoryStatusV1(value: unknown): GisMapApprovalHistoryStatusV1 {
  if (typeof value !== "object" || value === null || Array.isArray(value)) throw new Error("backbone worker: invalid inference history status");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "canUndo,code,phase") throw new Error("backbone worker: invalid inference history fields");
  const phase = row.phase;
  if (phase !== "unavailable" && phase !== "available" && phase !== "submitting" && phase !== "applied" && phase !== "failed") throw new Error("backbone worker: invalid inference history phase");
  if (typeof row.canUndo !== "boolean" || (phase !== "available" && phase !== "failed" && row.canUndo)) throw new Error("backbone worker: invalid inference history availability");
  if (phase === "available" && !row.canUndo) throw new Error("backbone worker: invalid inference history availability");
  if (row.code !== null && (typeof row.code !== "string" || !(row.code in GIS_MAP_INFERENCE_PORT_CODE_TEXT_V1))) throw new Error("backbone worker: invalid inference history code");
  if ((phase === "failed") !== (row.code !== null)) throw new Error("backbone worker: invalid inference history failure");
  return { phase, canUndo: row.canUndo, code: row.code as GisMapInferencePortCodeV1 | null };
}

