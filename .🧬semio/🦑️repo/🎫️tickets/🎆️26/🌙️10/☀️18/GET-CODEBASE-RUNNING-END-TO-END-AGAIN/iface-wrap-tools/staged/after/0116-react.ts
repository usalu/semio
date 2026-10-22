/** 🪟️ The browser actor ui patch offer and mounted identity messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { browserDocumentMountIsCurrentV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { type BackboneWorkerResponse, encodeBackboneWorkerRequest, documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { applyBrowserActorUiPatchesV1, browserActorPanelKeysV1 } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { BrowserActorActionMailboxV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { reduceBootstrapUiState, reduceExecutionTargetUiState, type BootstrapUiState, type ExecutionTargetUiState } from "../../🪪️host-bootstrap/🟦️.tsx";
import { canonicalSurfaceId } from "../../🔐️identity-binding/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";
import type { Dispatch, RefObject, SetStateAction } from "../../../⚛️react-port/🟦️.ts";

/** 🩻️ Validates a browser actor ui patch offer against the open session and answers a verdict per surface. */
export function handleBrowserActorUiPatchMessage({ openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, setBrowserActorUiVersion, publishUiRefreshSectionsV1, uiRefreshCacheRef, message, worker, core }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef" | "browserActorUiByRuntimeKeyRef" | "setBrowserActorUiVersion" | "publishUiRefreshSectionsV1" | "uiRefreshCacheRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "browser-actor-ui-patch" }>>;
  readonly worker: Worker;
  readonly core: ShellCoreAfterIdentityAndDocumentPorts;
}) {
  const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  const expectedSurfaceId = entry?.scope !== undefined && entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null;
  if (entry === undefined || entry.clientInstanceId !== message.clientInstanceId) return;
  const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
  const refusal =
    entry.scope === undefined || entry.scope.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId
      ? "scope-mismatch"
      : expectedSurfaceId !== message.verifiedSurfaceId
        ? `verified-surface-mismatch: session ${expectedSurfaceId ?? "<none>"} offer ${message.verifiedSurfaceId}`
        : message.instanceId !== 0
            ? `instance-mismatch: ${message.instanceId}`
            : retained !== undefined && (retained.clientInstanceId !== message.clientInstanceId || retained.activationGeneration !== message.activationGeneration || retained.verifiedSurfaceId !== message.verifiedSurfaceId || retained.sessionInstanceId !== entry.session.instanceId)
              ? "retained-owner-mismatch"
              : null;
  const answer = (verdicts: readonly { readonly surface: string; readonly outcome: "acknowledged" | "rejected"; readonly revision: number; readonly reason?: string }[]) =>
    worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "browser-actor-ui-patch-result", clientInstanceId: entry.clientInstanceId, scope: message.scope, verifiedSurfaceId: message.verifiedSurfaceId, activationGeneration: message.activationGeneration, instanceId: message.instanceId, receipt: message.receipt, verdicts }) });
  if (refusal !== null) {
    answer(message.patches.map((patch) => ({ surface: patch.surface, outcome: "rejected" as const, revision: 0, reason: refusal })));
    return;
  }
  const applied = applyBrowserActorUiPatchesV1(message.patches, new Set(entry.session.app.windowKinds.map((kind) => kind.id)), browserActorPanelKeysV1(entry.session.app), retained === undefined ? null : { windows: retained.windows, panels: retained.panels, sections: retained.sections });
  if (applied.stores !== null && retained === undefined) {
    const actions = new BrowserActorActionMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest({ ...request, clientInstanceId: entry.clientInstanceId }) }));
    browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { clientInstanceId: message.clientInstanceId, activationGeneration: message.activationGeneration, verifiedSurfaceId: message.verifiedSurfaceId, scope: { ...message.scope }, sessionInstanceId: entry.session.instanceId, windows: applied.stores.windows, panels: applied.stores.panels, sections: applied.stores.sections, identity: null, actions });
  }
  if (applied.stores !== null && (retained === undefined || applied.surfacesAdded)) setBrowserActorUiVersion((current) => current + 1);
  const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey),
    shown = core.shellStateRef.current.pluginRuntime.session;
  if (applied.sectionsChanged && live !== undefined && shown?.pluginId === entry.session.pluginId && shown.instanceId === entry.session.instanceId) publishUiRefreshSectionsV1(uiRefreshCacheRef.current, live);
  answer(applied.verdicts);
  return;
}

/** 🏔️ Binds the mounted browser actor identity and resolves the document ready state. */
export function handleBrowserActorUiMountedMessage({ openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, failDocumentBackbone, dispatch, setBrowserActorUiVersion, message, rebootstrapDiscardedSessionsRef, setBootstrapUiByDocument, setExecutionTargetUiByDocument }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "openDocumentSessionsRef" | "browserActorUiByRuntimeKeyRef" | "failDocumentBackbone" | "dispatch" | "setBrowserActorUiVersion"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "browser-actor-ui-mounted" }>>;
  readonly rebootstrapDiscardedSessionsRef: RefObject<Map<string, ActiveSession>>;
  readonly setBootstrapUiByDocument: Dispatch<SetStateAction<BootstrapUiState>>;
  readonly setExecutionTargetUiByDocument: Dispatch<SetStateAction<ExecutionTargetUiState>>;
}) {
  const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
  const entry = openDocumentSessionsRef.current.get(runtimeKey);
  const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
  if (entry === undefined || retained === undefined || !browserDocumentMountIsCurrentV1(
    { clientInstanceId: entry.clientInstanceId, scope: entry.scope, instanceId: entry.session.instanceId },
    { clientInstanceId: retained.clientInstanceId, scope: retained.scope, instanceId: retained.sessionInstanceId, activationGeneration: retained.activationGeneration, verifiedSurfaceId: retained.verifiedSurfaceId, revision: retained.windows.get(message.windowKindId)?.getRevisionSnapshot() ?? 0 },
    { ...message, revision: message.uiRevision },
  )) return;
  if (entry.creationMount !== null && !entry.creationMount.accept(message.catalogGenerationId)) {
    failDocumentBackbone(runtimeKey, entry, new Error("artifact-creation.catalog-generation-mismatch"));
    return;
  }
  browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { ...retained, identity: message });
  entry.resolveReady();
  const discarded = rebootstrapDiscardedSessionsRef.current.get(runtimeKey);
  if (discarded !== undefined) {
    rebootstrapDiscardedSessionsRef.current.delete(runtimeKey);
    dispatch({ type: "SET_SESSION", value: (current) => current ?? discarded });
  }
  setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "snapshot-replaced", documentId: message.scope.documentId, scope: message.scope }));
  setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: message.scope.documentId, scope: message.scope }));
  setBrowserActorUiVersion((current) => current + 1);
  return;
}
