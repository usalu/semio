/** 🧵️ Plugin supervisor and install progress refs, the document service inventory and `ensureBackboneWorker`, which boots and wires the store worker.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useBackboneWorker` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { directorySessionAuthorityIsCurrentV1 } from "@semio-tech/framework/directory/session-refresh";
import { useCallback, useRef, useState } from "../../⚛️react-port/🟦️.ts";
// #region 🔌️Adapters
import {
  decodePackValue
} from "@semio-tech/framework/packs";
import {
  decodeDocumentArchiveBytes,
  type DocumentArchivePack
} from "@semio-tech/framework/wire-contracts";
import {
  DocumentArchiveFaultError
} from "@semio-tech/framework/app-channel";
import {
  type BackboneWorkerResponse,
  decodeBackboneWorkerResponse,
  encodeBackboneWorkerRequest,
  documentRuntimeKeyV1
} from "@semio-tech/framework/backbone";
import {
  parseDirectorySpaceAdministrationPageV1
} from "@semio-tech/framework/directory/io/text";
import { type ScopedPresencePeerV1 } from "../👥️presence-scope/🟦️.ts";
import { IDENTITY_CONFIG_SCHEMA } from "@semio-tech/framework/backbone/config-lane";
import { LOCAL_CATALOG_CONFIG_SCHEMA, type LocalCatalog } from "@semio-tech/framework/config/mutations";
import { type ActiveSession, type LoadedProgramState, type PluginSupervisorState } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import {
  applyBrowserActorUiPatchesV1,
  browserActorPanelKeysV1,
  commandRejectionNoticeV1,
  appFaultNoticeV1,
  historyLaneNoticeV1,
  historyRefusalCodeV1,
  historyRefusalNoticeV1
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
// #endregion 🔌️Adapters
import { isPluginInstanceRetiredV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { StreamMuxChannelV1 } from "@semio-tech/framework/io/stream-mux";
import { liveInstanceWindowFaultV1 } from "../🩺️fault/🟦️.ts";
import {
  inferencePortStatusRuntimeKeyV1,
  reduceBootstrapUiState,
  reduceExecutionTargetUiState,
  type BootstrapUiState,
  type ExecutionTargetUiState
} from "../🪪️host-bootstrap/🟦️.tsx";
import { announcedPageStreams, scopeRuntimeKey } from "../🔐️identity-binding/🟦️.ts";
import {
  captureSpaceArtifactCreationCatalogAuthorityV1,
  retainDirectoryCommandResult,
  SPACE_INDEX_DOCUMENT_ID,
  spaceArtifactCreationCatalogRefreshRequestV1,
  spaceArtifactCreationOwnerAcceptsStatus,
  type SpaceArtifactCreationOwnerV1,
  spaceArtifactCreationReadyOpening
} from "../🧾️space-artifact-creation/🟦️.ts";
import type { ShellCoreAfterIdentityAndDocumentPorts } from "../🔒️identity-document-ports/🟦️.ts";
import { handleBrowserActorUiMountedMessage, handleBrowserActorUiPatchMessage } from "./🪟️browser-actor-messages/🟦️.ts";
import { handleSocketActorFailedMessage, handleSocketActorMessage } from "./🔌️socket-actor-messages/🟦️.ts";
import { handleInferenceHistoryStatusMessage } from "./🧠️inference-history-messages/🟦️.ts";
import { handleDocumentCheckInStatusMessage, handleSpaceArtifactCreationCatalogRefreshRequiredMessage, handleSpaceArtifactCreationStatusMessage } from "./🌱️artifact-creation-messages/🟦️.ts";
import { handleDirectoryAdministrationCapabilityMessage, handleDirectoryAdministrationStateMessage, handleDirectoryCommandReceiptMessage, handleDirectoryScopeRevokedMessage } from "./📇️directory-messages/🟦️.ts";
import { handleArtifactBootstrapMessage } from "./🥾️bootstrap-messages/🟦️.ts";
import { handleDocumentEventMessage, handleIdentityEventMessage, handleLocalCatalogEventMessage } from "./📬️document-event-messages/🟦️.ts";
import { usePluginInstallState } from "./🔧️plugin-install-state/🟦️.ts";

export type ShellCoreAfterBackboneWorker = ShellCoreAfterIdentityAndDocumentPorts & ReturnType<typeof useBackboneWorker>;

//#region FrameworkOsShell
export function useBackboneWorker(core: ShellCoreAfterIdentityAndDocumentPorts) {
  const {
    backboneWorkerFactory, dispatch, historyStore, historyOrderRef, loadedPlugins, pluginSupervisorById,
    uiRefreshCacheRef, sessionRef, backboneWorkerRef, backboneWorkerStreamsDetachRef, setIdentity,
    verifiedSessionAuthorityRef, sessionRefreshRef, hubEnv, handleDirectoryEventPageRef, handlePreferenceLaneMessageRef,
    handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef, showTransientNoticeRef, closeDocumentRef,
    openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, setSpaceArtifactCreationUi,
    setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, spaceArtifactCreationCatalogRef,
    spaceArtifactCreationCatalogUiRef, cancelSpaceArtifactCreationsForRuntime, setDirectoryPendingCommands,
    directoryCommandResultsRef, setSpaceAdministration, spaceAdministrationRef, spaceAdministrationEpochRef,
    spaceAdministrationMessageRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef,
    retiredSessionDocumentOwnersRef, inferencePortOpeningRef, setInferenceHistoryByRuntimeKey,
    identitySnapshotResolverRef, identityClientInstanceIdRef, setLocalCatalog, localCatalogClientInstanceIdRef,
    localDocumentOwnersRef, openDocumentSessionsRef, failDocumentBackbone, receiveDocumentBackbone, captureDialogOrigin,
    documentArchivePorts, browserActorUiByRuntimeKeyRef, setBrowserActorUiVersion, publishUiRefreshSectionsV1,
    retireBrowserActorUi, directoryScopedOwnersRef, collectSpaceDirectoryEvents
  } = core;
  const socketActorReadyRef = useRef<Map<string, { readonly clientInstanceId: string; resolve(actorId: string): void; reject(error: Error): void }>>(new Map());
  const [bootstrapUiByDocument, setBootstrapUiByDocument] = useState<BootstrapUiState>({});
  const [executionTargetUiByDocument, setExecutionTargetUiByDocument] = useState<ExecutionTargetUiState>({});
  const [presencePeersByRuntimeKey, setPresencePeersByRuntimeKey] = useState<Readonly<Record<string, readonly ScopedPresencePeerV1[]>>>({});
  const rebootstrapDiscardedSessionsRef = useRef<Map<string, ActiveSession>>(new Map());
  /** 🐚️ Mirrors `loadedPlugins` for the unmount-cleanup effect below, which needs the latest value at
   * teardown time without depending on it (a dependency would tear down and re-run on every reload). */
  const loadedPluginsRef = useRef<readonly LoadedProgramState[]>([]);
  core.loadedPluginsRef = loadedPluginsRef;
  loadedPluginsRef.current = loadedPlugins;
  /** 🩺️ Read at catch time by the fault classifier so a crashed/quarantined supervisor outranks the
   * plugin's own fault code, without making every refresh effect depend on the supervisor roster. */
  const pluginSupervisorByIdRef = useRef<Readonly<Record<string, PluginSupervisorState>>>({});
  pluginSupervisorByIdRef.current = pluginSupervisorById;

  /** 🧯️ Where every FIRE-AND-FORGET refresh sends its rejection. A `void refreshUi(…)` without a
   * `catch` is an unhandled promise rejection, and an unhandled rejection out of a React subtree stops
   * that subtree applying updates — so one guest that declines to render takes the whole shell with it,
   * silently, with nothing on screen to say so.
   *
   * Measured 2026-09-20 (ticket 26/09/18, S2 §3.3): `🪐️space`'s Home answers
   * `s.home.session-identity-required` and `settlePluginTurn` throws "stopped without publishing
   * requested UI surfaces"; the rejection escaped, the tree froze, and the human's subsequent
   * successful hub sign-in could no longer re-establish anything — every recovery path in this file was
   * intact and simply never re-entered. A guest refusal is a WINDOW FAULT: it is shown, and the shell
   * keeps running so the next identity/session change can recover from it.
   *
   * A refresh that lands on a RETIRED instance ({@link isPluginInstanceRetiredV1}) is not a fault: the
   * window it would have refreshed is already gone, exactly the drop `dropForRetiredInstance` makes.
   * Reporting it raised the shell's boot error beacon for ~1 s on every `de` boot of the `s` host (a
   * locale-driven re-open retires the Home instance while its refresh is in flight; measured
   * `wp-g10/generated/boot-probe-de-1.txt`). */
  const reportRefreshFault = useCallback((pluginId: string, error: unknown): void => {
    const fault = liveInstanceWindowFaultV1(error, pluginSupervisorByIdRef.current[pluginId], isPluginInstanceRetiredV1);
    if (fault !== null) dispatch({ type: "SET_ERROR", value: fault.message, fault });
  }, []);
  const { pluginModuleUrlByIdRef, pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef, pluginInstallAbortsRef, pluginInstallProgressById, notePluginInstallProgress } = usePluginInstallState();

  const documentServiceInventoryRef = useRef<Array<{owner:string;serviceId:string}>>([]);
  const ensureBackboneWorker = useCallback((): Worker => {
    if (backboneWorkerRef.current) return backboneWorkerRef.current;
    const worker = backboneWorkerFactory?.() ?? new Worker(new URL("../../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url), { type: "module" });
    worker.postMessage({wire:encodeBackboneWorkerRequest({kind:"service-contributions",services:documentServiceInventoryRef.current})});
    const streams = announcedPageStreams();
    if (streams instanceof StreamMuxChannelV1) {
      const streamPipe = new MessageChannel();
      backboneWorkerStreamsDetachRef.current = streams.attachPort(streamPipe.port1);
      worker.postMessage({ kind: "semio-stream-mux-port", port: streamPipe.port2 }, [streamPipe.port2]);
    }
    worker.onmessage = (messageEvent: MessageEvent<BackboneWorkerResponse | { readonly wire: Uint8Array }>) => {
      const message = "wire" in messageEvent.data ? decodeBackboneWorkerResponse(messageEvent.data.wire) : messageEvent.data;
      if (message.kind === "browser-actor-action-result") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", ...message.scope });
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (retained === undefined || entry === undefined || retained.clientInstanceId !== message.clientInstanceId || entry.clientInstanceId !== message.clientInstanceId || retained.sessionInstanceId !== entry.session.instanceId) return;
        const { clientInstanceId: _clientInstanceId, ...result } = message;
        retained.actions.settle(result);
        return;
      }
      if (message.kind === "browser-actor-ui-patch") {
        return handleBrowserActorUiPatchMessage({ openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, setBrowserActorUiVersion, publishUiRefreshSectionsV1, uiRefreshCacheRef, message, worker, core });
      }
      if (message.kind === "browser-actor-ui-mounted") {
        return handleBrowserActorUiMountedMessage({ openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, failDocumentBackbone, dispatch, setBrowserActorUiVersion, message, rebootstrapDiscardedSessionsRef, setBootstrapUiByDocument, setExecutionTargetUiByDocument });
      }
      if (message.kind === "socket-actor") {
        return handleSocketActorMessage({ openDocumentSessionsRef, message, socketActorReadyRef });
      }
      if (message.kind === "inference-port-opened") {
        inferencePortOpeningRef.current?.mailbox.settle(message);
        return;
      }
      if (message.kind === "inference-port-closed") {
        const owner = inferencePortOwnerRef.current;
        if (owner === null || owner.owner !== message.owner || owner.serviceId !== message.serviceId || owner.operationEpoch !== message.operationEpoch || owner.scope.spaceId !== message.scope.spaceId || owner.scope.documentId !== message.scope.documentId) return;
        inferencePortOwnerRef.current = null;
        inferencePortAuthorityRef.current = null;
        dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
        return;
      }
      if (message.kind === "inference-port-status") {
        if (!directorySessionAuthorityIsCurrentV1(inferencePortAuthorityRef.current, verifiedSessionAuthorityRef.current)) return;
        const runtimeKey = inferencePortStatusRuntimeKeyV1(inferencePortOwnerRef.current, inferencePortEpochRef.current, message);
        if (runtimeKey === null) return;
        dispatch({ type: "SET_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey, status: message.status });
        return;
      }
      if (message.kind === "inference-history-status") {
        return handleInferenceHistoryStatusMessage({ openDocumentSessionsRef, verifiedSessionAuthorityRef, retiredSessionDocumentOwnersRef, setInferenceHistoryByRuntimeKey, historyOrderRef, message });
      }
      if (message.kind === "execution-target-status") {
        const runtimeKey = scopeRuntimeKey(message);
        const entry = runtimeKey === null ? undefined : openDocumentSessionsRef.current.get(runtimeKey);
        if (runtimeKey === null || message.scope?.spaceId !== message.spaceId || entry?.clientInstanceId !== message.clientInstanceId) return;
        setExecutionTargetUiByDocument((current) => reduceExecutionTargetUiState(current, message));
        return;
      }
      if (message.kind === "socket-actor-failed") {
        return handleSocketActorFailedMessage({ openDocumentSessionsRef, message, socketActorReadyRef });
      }
      if (message.kind === "space-artifact-creation-catalog-refresh-required") {
        return handleSpaceArtifactCreationCatalogRefreshRequiredMessage({ spaceArtifactCreationOwnersRef, captureDialogOrigin, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, message, core, worker });
      }
      if (message.kind === "document-check-in-status") {
        return handleDocumentCheckInStatusMessage({ openDocumentSessionsRef, core, message });
      }
      if (message.kind === "space-artifact-creation-status") {
        return handleSpaceArtifactCreationStatusMessage({ spaceArtifactCreationOwnersRef, openDocumentSessionsRef, setSpaceArtifactCreationUi, openReadySpaceArtifactCreationRef, message, worker });
      }
      if (message.kind === "space-artifact-creation-catalog") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.spaceId, documentId: SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return;
        setSpaceArtifactCreationCatalog(message);
        return;
      }
      if (message.kind === "space-artifact-creation-catalog-status") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.spaceId, documentId: SPACE_INDEX_DOCUMENT_ID });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        if (entry?.clientInstanceId !== message.clientInstanceId || entry.scope?.spaceId !== message.spaceId || entry.scope.documentId !== SPACE_INDEX_DOCUMENT_ID) return;
        if (message.phase !== "ready") setSpaceArtifactCreationCatalog(null);
        setSpaceArtifactCreationCatalogUi(message);
        return;
      }
      if (message.kind === "preference-lane-page" || message.kind === "preference-lane-recorded" || message.kind === "preference-lane-failed") {
        handlePreferenceLaneMessageRef.current(message);
        return;
      }
      if (message.kind === "directory-event-page") {
        handleDirectoryEventPageRef.current(message);
        return;
      }
      if (message.kind === "directory-bootstrap-failed") {
        handleDirectoryBootstrapFailureRef.current(message);
        return;
      }
      if (message.kind === "directory-message") {
        if (message.message.kind === "event") collectSpaceDirectoryEvents([message.message.event]);
        return;
      }
      if (message.kind === "directory-space-events") {
        collectSpaceDirectoryEvents(message.events);
        return;
      }
      if (message.kind === "directory-scope-revoked") {
        return handleDirectoryScopeRevokedMessage({ directoryScopedOwnersRef, cancelSpaceArtifactCreationsForRuntime, setSpaceArtifactCreationCatalog, setSpaceArtifactCreationCatalogUi, openDocumentSessionsRef, closeDocumentRef, retireBrowserActorUi, setInferenceHistoryByRuntimeKey, message, worker, rebootstrapDiscardedSessionsRef, setPresencePeersByRuntimeKey, setBootstrapUiByDocument, setExecutionTargetUiByDocument });
      }
      if (message.kind === "directory-status") {
        setDirectoryPendingCommands(message.pendingCommands);
        return;
      }
      if (message.kind === "directory-administration-state") {
        return handleDirectoryAdministrationStateMessage({ spaceAdministrationEpochRef, backboneWorkerRef, spaceAdministrationRef, spaceAdministrationMessageRef, setSpaceAdministration, message, worker });
      }
      if (message.kind === "directory-administration-capability") {
        return handleDirectoryAdministrationCapabilityMessage({ spaceAdministrationRef, spaceAdministrationEpochRef, backboneWorkerRef, message });
      }
      if (message.kind === "directory-administration-capability-rejected") {
        return;
      }
      if (message.kind === "directory-command-failed") {
        retainDirectoryCommandResult(directoryCommandResultsRef.current, message.requestId, { kind: "failed", code: message.code });
        console.error("[os-shell] directory command failed", message.requestId, message.code);
        return;
      }
      if (message.kind === "directory-command-receipt") {
        return handleDirectoryCommandReceiptMessage({ directoryCommandResultsRef, collectSpaceDirectoryEvents, message });
      }
      if (message.kind === "artifact-bootstrap-progress" || message.kind === "artifact-bootstrap-failed" || message.kind === "artifact-rebootstrap-required") {
        return handleArtifactBootstrapMessage({ openDocumentSessionsRef, failDocumentBackbone, retireBrowserActorUi, cancelSpaceArtifactCreationsForRuntime, sessionRef, message, setBootstrapUiByDocument, worker, rebootstrapDiscardedSessionsRef });
      }
      if (message.kind === "event" && message.documentId === LOCAL_CATALOG_CONFIG_SCHEMA) {
        return handleLocalCatalogEventMessage({ localCatalogClientInstanceIdRef, setLocalCatalog, message });
      }
      if (message.kind === "event" && localDocumentOwnersRef.current.has(message.clientInstanceId)) {
        const owner = localDocumentOwnersRef.current.get(message.clientInstanceId);
        if (message.event.kind === "status") owner?.status?.(message.event.persisted);
        else if (message.event.kind === "documentArchiveReplaced") owner?.archive?.(new Uint8Array(message.event.archive));
        return;
      }
      if (message.kind === "event" && message.documentId === IDENTITY_CONFIG_SCHEMA) {
        return handleIdentityEventMessage({ identityClientInstanceIdRef, verifiedSessionAuthorityRef, setIdentity, identitySnapshotResolverRef, sessionRefreshRef, message });
      }
      return handleDocumentEventMessage({ openDocumentSessionsRef, dispatch, receiveDocumentBackbone, uiTerminologyRef, uiLocaleRef, showTransientNoticeRef, closeDocumentRef, documentArchivePorts, failDocumentBackbone, historyStore, sessionRef, message, setPresencePeersByRuntimeKey, core, rebootstrapDiscardedSessionsRef, setBootstrapUiByDocument });
    };
    const failBrowserActorActions = () => {
      for (const retained of browserActorUiByRuntimeKeyRef.current.values()) retained.actions.close("browser-actor-action: worker completion unconfirmed");
    };
    worker.addEventListener("error", failBrowserActorActions);
    worker.addEventListener("messageerror", failBrowserActorActions);
    backboneWorkerRef.current = worker;
    return worker;
  }, [cancelSpaceArtifactCreationsForRuntime, captureDialogOrigin, collectSpaceDirectoryEvents, failDocumentBackbone, hubEnv, documentArchivePorts, receiveDocumentBackbone, retireBrowserActorUi]);

  return {
    socketActorReadyRef, bootstrapUiByDocument, setBootstrapUiByDocument, executionTargetUiByDocument,
    setExecutionTargetUiByDocument, presencePeersByRuntimeKey, setPresencePeersByRuntimeKey,
    rebootstrapDiscardedSessionsRef, pluginSupervisorByIdRef, reportRefreshFault, pluginModuleUrlByIdRef,
    pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef, pluginInstallAbortsRef,
    pluginInstallProgressById, notePluginInstallProgress, documentServiceInventoryRef, ensureBackboneWorker
  };
}
//#endregion FrameworkOsShell
