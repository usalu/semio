/** 📒️ The hub connection book state and the hub command callbacks of the hub connection hook. */
import { useCallback, useEffect, useReducer, useRef, useState } from "../../⚛️react-port/🟦️.ts";
import type { DirectoryCommand, DirectoryCommandReceiptV1 } from "@semio-tech/framework/directory/schema";
import { hubSessionInitialStateV1, readHubConnectionBookV1, reduceHubSessionV1, removeHubConnectionV1, runHubSessionAuthorityV1, runHubSignInV1, selectHubConnectionV1, selectedHubConnectionV1, upsertHubConnectionV1, writeHubConnectionBookV1, hubConnectionIdForOriginV1, parseHubOriginV1, type HubConnectionBookV1, type HubSessionAuthorityV1, type HubSignInCredentialV1, type HubSignInErrorCodeV1 } from "@semio-tech/framework/directory/sign-in";
import { spaceRowsAfterEventsV1, spaceRowsV1, type SpaceBrowserPhaseV1, type SpaceRowV1 } from "@semio-tech/framework/directory/spaces";
import { type AgentDelegationErrorCodeV1, type AgentDelegationPhaseV1, type AgentDelegationSummaryV1 } from "@semio-tech/framework/directory/delegations";
import { AgentDelegationRefusalV1, HUB_SESSION_AUTHORITY_RECHECK_MAX_MS, type HubAgentCredentialV1, hubConnectionOperationOwnerCurrentV1, type HubConnectionOperationOwnerV1, type HubConnectionPortV1, type HubInviteCapabilityV1, type HubRedemptionStateV1, type HubSpaceMemberRowV1 } from "../🔌️connection-contract/🟦️.ts";
import { useLiveRef } from "../🧰️hub-connection-helpers/🟦️.tsx";

/** 📖 Binds the hub connection book state, its readers and the space member watchers. */
export function useHubConnectionBook({ port }: { readonly port: HubConnectionPortV1 }) {
  const [book, setBook] = useState<HubConnectionBookV1>(() => readHubConnectionBookV1(port.storage, port.bootstrapOrigin));
  const connection = selectedHubConnectionV1(book);
  const [session, dispatchSession] = useReducer(reduceHubSessionV1, connection.id, hubSessionInitialStateV1);
  const [rows, setRows] = useState<readonly SpaceRowV1[]>([]);
  const [spacesPhase, setSpacesPhase] = useState<SpaceBrowserPhaseV1>("loading");
  const [search, setSearch] = useState("");
  const [invite, setInvite] = useState<HubInviteCapabilityV1 | null>(null);
  const [redemption, setRedemption] = useState<HubRedemptionStateV1>({ phase: "idle", error: null });
  const [authority, setAuthority] = useState<HubSessionAuthorityV1 | null>(null);
  const [memberRows, setMemberRows] = useState<readonly HubSpaceMemberRowV1[]>([]);
  const [watchedSpaceId, setWatchedSpaceId] = useState<string | null>(null);
  const [delegationSummaries, setDelegationSummaries] = useState<readonly AgentDelegationSummaryV1[]>([]);
  const [delegationPhase, setDelegationPhase] = useState<AgentDelegationPhaseV1>("idle");
  const [delegationError, setDelegationError] = useState<AgentDelegationErrorCodeV1 | null>(null);
  const [agentCredential, setAgentCredential] = useState<HubAgentCredentialV1 | null>(null);
  const delegationAbort = useRef<AbortController | null>(null);
  const rebootstrapped = useRef(false);
  const signInAbort = useRef<AbortController | null>(null);
  const authorityAbort = useRef<AbortController | null>(null);
  const membersAbort = useRef<AbortController | null>(null);
  const spacesAbort = useRef<AbortController | null>(null);
  const commandAbort = useRef<AbortController | null>(null);
  const signOutAbort = useRef<AbortController | null>(null);
  const redemptionAbort = useRef<AbortController | null>(null);
  const operationGeneration = useRef(1);
  const operationConnectionId = useRef(connection.id);
  const portRef = useLiveRef(port);
  const connectionRef = useLiveRef(connection);
  const watchedSpaceIdRef = useLiveRef(watchedSpaceId);

  if (operationConnectionId.current !== connection.id) {
    operationGeneration.current++;
    operationConnectionId.current = connection.id;
  }

  const captureOperationOwner = useCallback((): HubConnectionOperationOwnerV1 => ({ generation: operationGeneration.current, connectionId: connectionRef.current.id, origin: connectionRef.current.origin }), [connectionRef]);
  const operationOwnerCurrent = useCallback((owner: HubConnectionOperationOwnerV1): boolean => hubConnectionOperationOwnerCurrentV1(owner, operationGeneration.current, operationConnectionId.current), []);

  const retireOperationsForConnection = useCallback((connectionId: string) => {
    if (operationConnectionId.current === connectionId) return;
    operationGeneration.current++;
    operationConnectionId.current = connectionId;
    signInAbort.current?.abort();
    signOutAbort.current?.abort();
    commandAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    spacesAbort.current?.abort();
    redemptionAbort.current?.abort();
    delegationAbort.current?.abort();
    signInAbort.current = null;
    signOutAbort.current = null;
    commandAbort.current = null;
    authorityAbort.current = null;
    membersAbort.current = null;
    spacesAbort.current = null;
    redemptionAbort.current = null;
    delegationAbort.current = null;
  }, []);

  const persist = useCallback((next: HubConnectionBookV1) => {
    writeHubConnectionBookV1(portRef.current.storage, next);
    setBook(next);
  }, [portRef]);

  const loadSpaces = useCallback((owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    spacesAbort.current?.abort();
    const abort = new AbortController();
    spacesAbort.current = abort;
    setSpacesPhase((current) => (current === "ready" || current === "stale" ? current : "loading"));
    void portRef.current
      .listSpaces(owner.origin, abort.signal)
      .then((entries) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setRows(spaceRowsV1(entries));
        setSpacesPhase("ready");
        dispatchSession({ kind: "connectivity", offline: false });
      })
      .catch(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setSpacesPhase((current) => (current === "loading" ? "failed" : "stale"));
        dispatchSession({ kind: "connectivity", offline: true });
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  /** 🪪️ Reads `GET /auth/sessions/me`. This is the only source of `expiresAtMs` — AU1 §1.1 keeps the
   * expiry out of the mint response precisely so the two can never disagree — and a `401` here is
   * the hub's own statement that the capability is gone, which is what drives re-authentication. */
  const readAuthority = useCallback((owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    authorityAbort.current?.abort();
    const abort = new AbortController();
    authorityAbort.current = abort;
    void runHubSessionAuthorityV1(portRef.current.signIn, owner.origin, abort.signal).then((outcome) => {
      if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
      if (outcome.kind === "authority") {
        setAuthority(outcome.authority);
        dispatchSession({ kind: "minted", userId: outcome.authority.userId, expiresAtMs: outcome.authority.expiresAtMs });
        dispatchSession({ kind: "connectivity", offline: false });
        return;
      }
      if (outcome.kind === "expired") {
        setAuthority(null);
        dispatchSession({ kind: "expired" });
        return;
      }
      if (outcome.code === "unreachable") dispatchSession({ kind: "connectivity", offline: true });
    });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  const loadMembers = useCallback((spaceId: string | null, owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    membersAbort.current?.abort();
    if (spaceId === null) {
      setMemberRows([]);
      return;
    }
    const abort = new AbortController();
    membersAbort.current = abort;
    void portRef.current
      .readSpaceMembers(owner.origin, spaceId, abort.signal)
      .then((rows) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setMemberRows(rows);
      })
      .catch(() => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setMemberRows([]);
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  /** 🤖️ Reads `GET /auth/agent-delegations?space=<id>`. `null` clears the lane without a round trip:
   * a delegation belongs to exactly one space, so there is nothing to show until one is open. */
  const loadDelegations = useCallback((spaceId: string | null, owner = captureOperationOwner()) => {
    if (!operationOwnerCurrent(owner)) return;
    delegationAbort.current?.abort();
    if (spaceId === null) {
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
      return;
    }
    const abort = new AbortController();
    delegationAbort.current = abort;
    setDelegationPhase((current) => (current === "ready" ? current : "loading"));
    void portRef.current
      .listAgentDelegations(owner.origin, spaceId, abort.signal)
      .then((summaries) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationSummaries(summaries);
        setDelegationPhase("ready");
        setDelegationError(null);
      })
      .catch((error: unknown) => {
        if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
        setDelegationPhase("failed");
        setDelegationError(error instanceof AgentDelegationRefusalV1 ? error.code : "unreachable");
      });
  }, [captureOperationOwner, operationOwnerCurrent, portRef]);

  /** 👥️ Follows the space the shell has open. The one-time credential belongs to its delegation's space and is dropped
   * only when a DIFFERENT space opens: a session re-established under the pane passes through "no space" and back, and
   * clearing it there threw away a credential the human had not saved yet, mid "Set up MCP client" (G10 S4 relay). */
  const watchSpaceMembers = useCallback((spaceId: string | null) => {
    setWatchedSpaceId(spaceId);
    loadMembers(spaceId);
    setAgentCredential((value) => (value === null || spaceId === null || value.receipt.spaceId === spaceId ? value : null));
    loadDelegations(spaceId);
  }, [loadDelegations, loadMembers]);
  return { captureOperationOwner, commandAbort, setSpacesPhase, portRef, operationOwnerCurrent, session, setRows, loadSpaces, loadMembers, watchedSpaceIdRef, spacesAbort, connection, rebootstrapped, dispatchSession, readAuthority, operationGeneration, signInAbort, signOutAbort, authorityAbort, membersAbort, redemptionAbort, delegationAbort, book, operationConnectionId, retireOperationsForConnection, setAuthority, setMemberRows, setInvite, setRedemption, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, persist, setSearch, connectionRef, invite, loadDelegations, agentCredential, rows, search, memberRows, delegationSummaries, authority, spacesPhase, redemption, delegationPhase, delegationError, watchSpaceMembers };
}

/**
 * ⌨ Builds the hub command runner and the sign in cancellation callbacks.
 *
 * ♻️ Re-bootstrap on mount: the browsing context already holds a session (a reload, or this surface reopened after a
 * sign-in), so it is presumed live only until the hub's own `me` answer confirms or refuses it. Mount-only — a later
 * sign-out must not resurrect the session it just ended.
 *
 * ♻️ Re-authentication without a round trip per render: once the locally known deadline has
 * passed, one `me` read confirms it with the hub (which may have extended or revoked it) instead
 * of the shell deciding from its own clock.
 */
export function useHubCommandCallbacks({ captureOperationOwner, commandAbort, setSpacesPhase, portRef, operationOwnerCurrent, session, setRows, loadSpaces, loadMembers, watchedSpaceIdRef, spacesAbort, connection, rebootstrapped, dispatchSession, readAuthority, operationGeneration, signInAbort, signOutAbort, authorityAbort, membersAbort, redemptionAbort, delegationAbort, book, operationConnectionId, retireOperationsForConnection, setAuthority, setMemberRows, setInvite, setRedemption, setAgentCredential, setDelegationSummaries, setDelegationPhase, setDelegationError, persist }: Pick<ReturnType<typeof useHubConnectionBook>, "captureOperationOwner" | "commandAbort" | "setSpacesPhase" | "portRef" | "operationOwnerCurrent" | "session" | "setRows" | "loadSpaces" | "loadMembers" | "watchedSpaceIdRef" | "spacesAbort" | "connection" | "rebootstrapped" | "dispatchSession" | "readAuthority" | "operationGeneration" | "signInAbort" | "signOutAbort" | "authorityAbort" | "membersAbort" | "redemptionAbort" | "delegationAbort" | "book" | "operationConnectionId" | "retireOperationsForConnection" | "setAuthority" | "setMemberRows" | "setInvite" | "setRedemption" | "setAgentCredential" | "setDelegationSummaries" | "setDelegationPhase" | "setDelegationError" | "persist">) {
  const runCommand = useCallback(
    (command: DirectoryCommand, onReceipt: (receipt: DirectoryCommandReceiptV1) => void) => {
      const owner = captureOperationOwner();
      commandAbort.current?.abort();
      const abort = new AbortController();
      commandAbort.current = abort;
      setSpacesPhase("submitting");
      void portRef.current
        .submitCommand(owner.origin, command, abort.signal)
        .then((receipt) => {
          if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
          onReceipt(receipt);
          const userId = session.userId;
          if (userId !== null) setRows((current) => spaceRowsAfterEventsV1(current, receipt.events, userId));
          setSpacesPhase("ready");
          loadSpaces(owner);
          loadMembers(watchedSpaceIdRef.current, owner);
        })
        .catch(() => {
          if (abort.signal.aborted || !operationOwnerCurrent(owner)) return;
          setSpacesPhase("failed");
        });
    },
    [captureOperationOwner, loadMembers, loadSpaces, operationOwnerCurrent, portRef, session.userId, watchedSpaceIdRef],
  );

  useEffect(() => {
    loadSpaces();
    return () => spacesAbort.current?.abort();
  }, [connection.id, loadSpaces]);

  useEffect(() => {
    const held = portRef.current.heldCapability?.() ?? null;
    if (held === null || rebootstrapped.current) return;
    rebootstrapped.current = true;
    dispatchSession({ kind: "minted", userId: held.userId, expiresAtMs: null });
    readAuthority();
  }, [portRef, readAuthority, rebootstrapped]);

  useEffect(() => {
    const online = (): void => dispatchSession({ kind: "connectivity", offline: false });
    const offline = (): void => dispatchSession({ kind: "connectivity", offline: true });
    const target = typeof globalThis.addEventListener === "function" ? globalThis : null;
    target?.addEventListener("online", online);
    target?.addEventListener("offline", offline);
    return () => {
      target?.removeEventListener("online", online);
      target?.removeEventListener("offline", offline);
    };
  }, []);

  useEffect(() => () => {
    operationGeneration.current++;
    signInAbort.current?.abort();
    signOutAbort.current?.abort();
    commandAbort.current?.abort();
    authorityAbort.current?.abort();
    membersAbort.current?.abort();
    spacesAbort.current?.abort();
    redemptionAbort.current?.abort();
    delegationAbort.current?.abort();
  }, []);

  useEffect(() => {
    if (session.phase !== "signed-in" || session.expiresAtMs === null) return undefined;
    const remaining = session.expiresAtMs - Date.now();
    if (remaining <= 0) {
      readAuthority();
      return undefined;
    }
    const timer = setTimeout(readAuthority, Math.min(remaining, HUB_SESSION_AUTHORITY_RECHECK_MAX_MS));
    return () => clearTimeout(timer);
  }, [readAuthority, session.expiresAtMs, session.phase]);

  const selectConnection = useCallback((id: string) => {
    const next = selectHubConnectionV1(book, id);
    if (next.selectedId === operationConnectionId.current) return;
    retireOperationsForConnection(next.selectedId);
    setAuthority(null);
    setMemberRows([]);
    setInvite(null);
    setRedemption({ phase: "idle", error: null });
    setRows([]);
    setSpacesPhase("loading");
    setAgentCredential(null);
    setDelegationSummaries([]);
    setDelegationPhase("idle");
    setDelegationError(null);
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
  }, [book, persist, retireOperationsForConnection]);

  const addRemoteHub = useCallback((typed: string, label: string): HubSignInErrorCodeV1 | "invalid-origin" | null => {
    let origin: string;
    try {
      origin = parseHubOriginV1(typed);
    } catch {
      return "invalid-origin";
    }
    const trimmed = label.trim();
    const next = upsertHubConnectionV1(book, { id: hubConnectionIdForOriginV1(origin), label: trimmed.length > 0 ? trimmed.slice(0, 128) : origin, origin, kind: "remote", lastUserId: null });
    const changed = next.selectedId !== operationConnectionId.current;
    retireOperationsForConnection(next.selectedId);
    if (changed) {
      setAuthority(null);
      setMemberRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setRows([]);
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    }
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
    return null;
  }, [book, persist, retireOperationsForConnection]);

  const forgetHub = useCallback((id: string) => {
    const next = removeHubConnectionV1(book, id);
    const changed = next.selectedId !== operationConnectionId.current;
    retireOperationsForConnection(next.selectedId);
    if (changed) {
      setAuthority(null);
      setMemberRows([]);
      setInvite(null);
      setRedemption({ phase: "idle", error: null });
      setRows([]);
      setSpacesPhase("loading");
      setAgentCredential(null);
      setDelegationSummaries([]);
      setDelegationPhase("idle");
      setDelegationError(null);
    }
    dispatchSession({ kind: "select-connection", connectionId: next.selectedId });
    persist(next);
  }, [book, persist, retireOperationsForConnection]);

  const signIn = useCallback((input: Readonly<{ email: string; password: string }>) => {
    const owner = captureOperationOwner();
    signInAbort.current?.abort();
    const abort = new AbortController();
    signInAbort.current = abort;
    dispatchSession({ kind: "submit" });
    const active = portRef.current;
    const credential: HubSignInCredentialV1 = { email: input.email, password: input.password, deviceInstanceId: active.deviceInstanceId, clientClass: active.clientClass };
    void runHubSignInV1(active.signIn, owner.origin, credential, abort.signal).then((outcome) => {
      if (!operationOwnerCurrent(owner)) return;
      if (outcome.kind === "failed") {
        dispatchSession({ kind: "failed", code: outcome.code, retryAfterSeconds: outcome.retryAfterSeconds });
        if (outcome.code === "unreachable") dispatchSession({ kind: "connectivity", offline: true });
        return;
      }
      dispatchSession({ kind: "minted", userId: outcome.result.userId, expiresAtMs: null });
      dispatchSession({ kind: "connectivity", offline: false });
      const target = book.connections.find((entry) => entry.id === owner.connectionId);
      if (target) persist(upsertHubConnectionV1(book, { ...target, lastUserId: outcome.result.userId }));
      readAuthority(owner);
      loadSpaces(owner);
      loadMembers(watchedSpaceIdRef.current, owner);
    });
  }, [book, captureOperationOwner, loadMembers, loadSpaces, operationOwnerCurrent, persist, portRef, readAuthority, watchedSpaceIdRef]);

  const cancelSignIn = useCallback(() => {
    signInAbort.current?.abort();
    signInAbort.current = null;
    dispatchSession({ kind: "failed", code: "cancelled", retryAfterSeconds: null });
  }, []);
  return { runCommand, selectConnection, addRemoteHub, forgetHub, signIn, cancelSignIn };
}
