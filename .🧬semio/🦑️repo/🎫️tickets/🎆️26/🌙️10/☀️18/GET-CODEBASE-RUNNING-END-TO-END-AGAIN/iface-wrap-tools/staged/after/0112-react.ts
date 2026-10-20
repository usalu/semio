/** 🔸️ The retire authenticated inference phase of useDirectoryAndRoute. */
import { startDirectorySessionRefreshV1 } from "@semio-tech/framework/directory/session-refresh";
import { HubSessionPortClientV1 } from "@semio-tech/framework/directory/session-refresh/session-port";
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { waitForEvent } from "@semio-tech/framework";
import { encodeBackboneWorkerRequest } from "@semio-tech/framework/backbone";
import { IDENTITY_CONFIG_SCHEMA, identityActorConfig } from "@semio-tech/framework/backbone/config-lane";
import { type Identity, signIn } from "@semio-tech/framework/config/mutations";
import { setPluginRuntimeActor } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { identityMutationEnvelope, shellActorId } from "../../🔐️identity-binding/🟦️.ts";
import type { useCatalogState } from "../🔶️catalog/🟦️.ts";

/** 🔹 Builds retireAuthenticatedInferencePresentation, cancelSessionAuthorityBootstrap of useDirectoryAndRoute. */
export function useRetireAuthenticatedInferenceState({ inferencePortOpeningRef, backboneWorkerRef, inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, dispatch, openDocumentSessionsRef, retiredSessionDocumentOwnersRef, inferenceHistoryByRuntimeKeyRef, setInferenceHistoryByRuntimeKey, shellActorIdRef, identityBootstrapAbortRef, hubEnv, sessionRefreshRef, hubSessionPortRef, verifiedSessionAuthorityRef, setVerifiedSessionAuthority, setIdentityOffline, setHubSessionRefused, shellSessionIdRef, identityClientInstanceIdRef, hubSessionCapability, ensureBackboneWorker, identitySnapshotResolverRef, setIdentity }: Pick<ReturnType<typeof useCatalogState>, "inferencePortOpeningRef" | "backboneWorkerRef" | "inferencePortEpochRef" | "inferencePortOwnerRef" | "inferencePortAuthorityRef" | "dispatch" | "openDocumentSessionsRef" | "retiredSessionDocumentOwnersRef" | "inferenceHistoryByRuntimeKeyRef" | "setInferenceHistoryByRuntimeKey" | "shellActorIdRef" | "identityBootstrapAbortRef" | "hubEnv" | "sessionRefreshRef" | "hubSessionPortRef" | "verifiedSessionAuthorityRef" | "setVerifiedSessionAuthority" | "setIdentityOffline" | "setHubSessionRefused" | "shellSessionIdRef" | "identityClientInstanceIdRef" | "hubSessionCapability" | "ensureBackboneWorker" | "identitySnapshotResolverRef" | "setIdentity">) {
  /** 🧯️ Retires session-owned presentation immediately while the worker retains uncertain jobs. */
  const retireAuthenticatedInferencePresentation = useCallback(() => {
    inferencePortOpeningRef.current?.mailbox.close("inference-opening: identity retired");
    inferencePortOpeningRef.current = null;
    const worker = backboneWorkerRef.current;
    const operationEpoch = inferencePortEpochRef.current;
    if (worker && inferencePortOwnerRef.current) worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "service-operation", owner: inferencePortOwnerRef.current.owner, serviceId: inferencePortOwnerRef.current.serviceId, action: "close", operationEpoch, payload: {} }) });
    const owner = inferencePortOwnerRef.current;
    inferencePortOwnerRef.current = null;
    inferencePortAuthorityRef.current = null;
    inferencePortEpochRef.current = operationEpoch + 1;
    if (owner !== null) dispatch({ type: "CLEAR_INFERENCE_PORT_FOR_DOCUMENT", runtimeKey: owner.runtimeKey });
    for (const entry of openDocumentSessionsRef.current.values()) if (entry.scope !== undefined) retiredSessionDocumentOwnersRef.current.add(entry);
    inferenceHistoryByRuntimeKeyRef.current = {};
    setInferenceHistoryByRuntimeKey({});
  }, []);
  useEffect(() => retireAuthenticatedInferencePresentation, [retireAuthenticatedInferencePresentation]);

  useEffect(() => {
    setPluginRuntimeActor(shellActorIdRef.current);
  }, []);

  const cancelSessionAuthorityBootstrap = useCallback(() => {
    const abort = identityBootstrapAbortRef.current;
    if (!hubEnv || abort === null || abort.signal.aborted) return;
    identityBootstrapAbortRef.current = null;
    abort.abort("directory.session-authority.cancelled");
    sessionRefreshRef.current?.close();
    sessionRefreshRef.current = null;
    hubSessionPortRef.current?.close();
    hubSessionPortRef.current = null;
    retireAuthenticatedInferencePresentation();
    verifiedSessionAuthorityRef.current = null;
    setVerifiedSessionAuthority(null);
    setIdentityOffline(true);
    setHubSessionRefused(true);
    shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
    setPluginRuntimeActor(shellActorIdRef.current);
    const clientInstanceId = identityClientInstanceIdRef.current;
    if (clientInstanceId !== null) {
      backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
      if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
    }
  }, [hubEnv, retireAuthenticatedInferencePresentation]);

  useEffect(() => {
    const capability = hubSessionCapability;
    if (!hubEnv || capability === null) return;
    setHubSessionRefused(false);
    let cancelled = false;
    const identityWaitAbort = new AbortController();
    identityBootstrapAbortRef.current = identityWaitAbort;
    (async () => {
      const worker = ensureBackboneWorker();
      const identityConfig = identityActorConfig(shellActorIdRef.current, {maximumItems:1024,maximumCopyBytes:65536,maximumCapacityBytes:65536,maximumReleaseBytes:65536,maximumDepth:64}, hubEnv.dataDir);
      const identityAttempt = { clientInstanceId: crypto.randomUUID() };
      const { clientInstanceId } = identityAttempt;
      identityClientInstanceIdRef.current = clientInstanceId;
      worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "open", clientInstanceId, ...identityConfig }) });
      const storedIdentityRead = waitForEvent<Identity | null>(
        (handler) => {
          identitySnapshotResolverRef.current = handler;
          return () => {
            if (identitySnapshotResolverRef.current === handler) identitySnapshotResolverRef.current = null;
          };
        },
        { signal: AbortSignal.any([identityWaitAbort.signal, AbortSignal.timeout(2000)]) },
      ).catch(() => null);
      const sameIdentity = (left: Identity | null, right: Identity): boolean => left !== null && left.userId === right.userId && left.email === right.email && left.displayName === right.displayName && left.hubBaseUrl === right.hubBaseUrl;
      let shownIdentity: Identity | null = null;
      let writtenIdentity: Identity | null = null;
      const sessionChannel = new MessageChannel();
      worker.postMessage({ kind: "semio-hub-session-port", port: sessionChannel.port2 }, [sessionChannel.port2]);
      const sessionPort = new HubSessionPortClientV1(sessionChannel.port1, capability.token);
      hubSessionPortRef.current = sessionPort;
      const ownerIsCurrent = (): boolean => !cancelled && identityClientInstanceIdRef.current === clientInstanceId;
      sessionRefreshRef.current = startDirectorySessionRefreshV1({
        signal: identityWaitAbort.signal,
        read: (signal) => sessionPort.me(signal),
        onAuthority: (authority) => {
          if (!ownerIsCurrent()) return;
          const previous = verifiedSessionAuthorityRef.current;
          const authorityChanged = previous === null || previous.sessionBindingSha256 !== authority.sessionBindingSha256 || previous.authorizationGeneration !== authority.authorizationGeneration;
          if (authorityChanged) retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = authority;
          if (authorityChanged) setVerifiedSessionAuthority(authority);
          setIdentityOffline(false);
          setHubSessionRefused(false);
          const resolved: Identity = { userId: authority.userId, email: authority.email, displayName: authority.displayName, hubBaseUrl: hubEnv.hubBaseUrl, issuedAtMs: shownIdentity?.userId === authority.userId ? shownIdentity.issuedAtMs : Date.now() };
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, resolved);
          setPluginRuntimeActor(shellActorIdRef.current);
          if (sameIdentity(shownIdentity, resolved) && !authorityChanged) return;
          shownIdentity = resolved;
          setIdentity(resolved);
          void storedIdentityRead.then((stored) => {
            if (!ownerIsCurrent()) return;
            const previous = writtenIdentity ?? stored;
            if (sameIdentity(previous, resolved) && !authorityChanged) return;
            const record: Identity = previous?.userId === resolved.userId ? { ...resolved, issuedAtMs: previous.issuedAtMs } : resolved;
            const envelope = identityMutationEnvelope(shellActorIdRef.current, signIn(record), previous);
            writtenIdentity = record;
            worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "send", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId, message: { kind: "localMutations", envelopes: [envelope] } }) });
          });
        },
        onUnavailable: () => {
          if (!ownerIsCurrent()) return;
          retireAuthenticatedInferencePresentation();
          verifiedSessionAuthorityRef.current = null;
          setVerifiedSessionAuthority(null);
          setIdentityOffline(true);
          setHubSessionRefused(true);
          shellActorIdRef.current = shellActorId(shellSessionIdRef.current, null);
          setPluginRuntimeActor(shellActorIdRef.current);
        },
        onDegraded: (offline) => {
          if (!ownerIsCurrent()) return;
          setIdentityOffline(offline);
        },
      });
    })().catch((error) => console.error("[os-shell] identity bootstrap failed unexpectedly", error));
    return () => {
      cancelled = true;
      identityWaitAbort.abort();
      if (identityBootstrapAbortRef.current === identityWaitAbort) identityBootstrapAbortRef.current = null;
      sessionRefreshRef.current?.close();
      sessionRefreshRef.current = null;
      hubSessionPortRef.current?.close();
      hubSessionPortRef.current = null;
      verifiedSessionAuthorityRef.current = null;
      setVerifiedSessionAuthority(null);
      const clientInstanceId = identityClientInstanceIdRef.current;
      if (clientInstanceId !== null) {
        backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "close", documentId: IDENTITY_CONFIG_SCHEMA, clientInstanceId }) });
        if (identityClientInstanceIdRef.current === clientInstanceId) identityClientInstanceIdRef.current = null;
      }
    };
  }, [hubEnv, hubSessionCapability]);
  return { cancelSessionAuthorityBootstrap };
}
