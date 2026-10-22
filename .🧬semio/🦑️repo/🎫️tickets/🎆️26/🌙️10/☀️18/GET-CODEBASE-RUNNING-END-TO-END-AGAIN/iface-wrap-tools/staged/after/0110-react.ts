/** 📇️ The directory scope revocation, space administration and command receipt messages of the backbone worker.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useBackboneWorker}: each function is one message arm of the backbone worker `onmessage` ladder, moved verbatim. */
import { type BackboneWorkerResponse, encodeBackboneWorkerRequest, documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { parseDirectorySpaceAdministrationPageV1 } from "@semio-tech/framework/directory/io/text";
import { type ScopedPresencePeerV1 } from "../../👥️presence-scope/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { reduceBootstrapUiState, reduceExecutionTargetUiState, type BootstrapUiState, type ExecutionTargetUiState } from "../../🪪️host-bootstrap/🟦️.tsx";
import { retainDirectoryCommandResult } from "../../🧾️space-artifact-creation/🟦️.ts";
import { copyDirectoryInviteCapabilityV1, reduceShellSpaceAdministrationState, shellSpaceAdministrationCapabilityAllowed } from "../../🏠️space-administration/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../../🔒️identity-document-ports/🟦️.ts";
import type { Dispatch, RefObject, SetStateAction } from "../../../⚛️react-port/🟦️.ts";

/** 🚫️ Retires every document, creation and ui state of a revoked directory scope. */
export function handleDirectoryScopeRevokedMessage({ directoryScopedOwnersRef, cancelSpaceArtifactCreationsForRuntime, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, openDocumentSessionsRef, closeDocumentRef, retireBrowserActorUi, setInferenceHistoryByRuntimeKey, message, worker, rebootstrapDiscardedSessionsRef, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "directoryScopedOwnersRef" | "cancelSpaceArtifactCreationsForRuntime" | "setSpaceArtifactCreationCatalog" | "setSpaceArtifactCreationCatalogUi" | "openDocumentSessionsRef" | "closeDocumentRef" | "retireBrowserActorUi" | "setInferenceHistoryByRuntimeKey"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "directory-scope-revoked" }>>;
  readonly worker: Worker;
  readonly rebootstrapDiscardedSessionsRef: RefObject<Map<string, ActiveSession>>;
  readonly setPresencePeersByRuntimeKey: Dispatch<SetStateAction<Readonly<Record<string, readonly ScopedPresencePeerV1[]>>>>;
  readonly setBootstrapUiByDocument: Dispatch<SetStateAction<BootstrapUiState>>;
  readonly setExecutionTargetUiByDocument: Dispatch<SetStateAction<ExecutionTargetUiState>>;
}) {
  const key = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
  if (!directoryScopedOwnersRef.current.delete(key)) return;
  cancelSpaceArtifactCreationsForRuntime(key, worker);
  setSpaceArtifactCreationCatalog((catalog) => catalog?.spaceId === message.scope.spaceId ? null : catalog);
  setSpaceArtifactCreationCatalogUi((status) => status?.spaceId === message.scope.spaceId ? null : status);
  const entry = openDocumentSessionsRef.current.get(key);
  if (entry !== undefined) closeDocumentRef.current(key, entry.clientInstanceId);
  retireBrowserActorUi(key, "browser-actor-action: scope revoked");
  rebootstrapDiscardedSessionsRef.current.delete(key);
  setInferenceHistoryByRuntimeKey((current) => {
    if (!(key in current)) return current;
    const next = { ...current };
    delete next[key];
    return next;
  });
  setPresencePeersByRuntimeKey((current) => {
    if (!(key in current)) return current;
    const next = { ...current };
    delete next[key];
    return next;
  });
  setBootstrapUiByDocument((current) => reduceBootstrapUiState(current, { kind: "detached", documentId: message.scope.documentId, scope: message.scope }));
  setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, { kind: "execution-target-cleared", documentId: message.scope.documentId, scope: message.scope }));
  return;
}

/** 🏠️ Verifies and applies a space administration page for the current epoch. */
export function handleDirectoryAdministrationStateMessage({ spaceAdministrationEpochRef, backboneWorkerRef, spaceAdministrationRef, spaceAdministrationMessageRef, setSpaceAdministration, message, worker }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "spaceAdministrationEpochRef" | "backboneWorkerRef" | "spaceAdministrationRef" | "spaceAdministrationMessageRef" | "setSpaceAdministration"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "directory-administration-state" }>>;
  readonly worker: Worker;
}) {
  const epoch = spaceAdministrationEpochRef.current;
  const administrationMessage = message;
  if (administrationMessage.operationEpoch !== epoch || worker !== backboneWorkerRef.current || administrationMessage.spaceId !== spaceAdministrationRef.current?.spaceId) return;
  const revision = ++spaceAdministrationMessageRef.current;
  const verificationIsCurrent = (): boolean => epoch === spaceAdministrationEpochRef.current && revision === spaceAdministrationMessageRef.current && worker === backboneWorkerRef.current;
  if (administrationMessage.canonicalJson === undefined) {
    setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, current?.page ?? null) : current);
    return;
  }
  void parseDirectorySpaceAdministrationPageV1(administrationMessage.canonicalJson)
    .then((page) => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, administrationMessage, epoch, page) : current))
    .catch(() => setSpaceAdministration((current) => verificationIsCurrent() ? reduceShellSpaceAdministrationState(current, { ...administrationMessage, phase: "failed", code: "invalid" }, epoch, null) : current));
  return;
}

/** 🎟️ Copies an invite capability for the current administration transfer. */
export function handleDirectoryAdministrationCapabilityMessage({ spaceAdministrationRef, spaceAdministrationEpochRef, backboneWorkerRef, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "spaceAdministrationRef" | "spaceAdministrationEpochRef" | "backboneWorkerRef"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "directory-administration-capability" }>>;
}) {
  if (!shellSpaceAdministrationCapabilityAllowed(spaceAdministrationRef.current, message.operationEpoch) || message.operationEpoch !== spaceAdministrationEpochRef.current) return;
  const { operationEpoch, transferEpoch, inviteToken } = message;
  void copyDirectoryInviteCapabilityV1(inviteToken).then((copied) => {
    const currentWorker = backboneWorkerRef.current;
    if (currentWorker === null || operationEpoch !== spaceAdministrationEpochRef.current) return;
    currentWorker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "directory-administration-capability-result", operationEpoch, transferEpoch, copied }) });
  });
  return;
}

/** 🧾️ Retains a command receipt and folds its accepted events into the directory. */
export function handleDirectoryCommandReceiptMessage({ directoryCommandResultsRef, collectSpaceDirectoryEvents, message }: Pick<ShellCoreAfterIdentityAndDocumentPorts, "directoryCommandResultsRef" | "collectSpaceDirectoryEvents"> & {
  readonly message: Extract<BackboneWorkerResponse, Readonly<{ kind: "directory-command-receipt" }>>;
}) {
  retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "receipt", receipt: message.receipt });
  if (message.receipt.outcome === "accepted" && message.receipt.events.length > 0) collectSpaceDirectoryEvents(message.receipt.events);
  return;
}
