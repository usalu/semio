/** 🪪️ The identity, agent bridge and directory state of the identity and document ports hook. */
import { BackgroundDocumentSessionsV1, DocumentAttachmentLaneV1, type DocumentOpeningReceiptV1 } from "../../🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts";
import { type DirectorySessionRefreshV1 } from "@semio-tech/framework/directory/session-refresh";
import { HubSessionPortClientV1 } from "@semio-tech/framework/directory/session-refresh/session-port";
import { LOCAL_HUB_SESSION_ENDPOINT_V1, parseLocalHubSessionAnswerV1 } from "@semio-tech/framework/directory/local-session";
import { readHubSessionCapabilityV1, writeHubSessionCapabilityV1, type HubSessionCapabilityV1 } from "@semio-tech/framework/directory/sign-in";
import { useCallback, useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type AppRef, type AppRole, type ArtifactDialect, type Conflict, type MergeReport, type Severity } from "@semio-tech/framework";
import { type BackboneWorkerResponse, encodeBackboneWorkerRequest, type PersistenceBinding, type ResolvedArtifactOpeningRelay } from "@semio-tech/framework/backbone";
import { type DirectorySessionAuthorityV1 } from "@semio-tech/framework/directory/schema/session-authority-v1";
import { type DirectoryEvent } from "@semio-tech/framework/directory/schema";
import { shellIdentityResolutionV1 } from "../../🔀️surface-switch/🟦️.ts";
import { type Identity } from "@semio-tech/framework/config/mutations";
import { builtinUiThemes, type ElementsSurfaceDevice, resolveUiDriver, semioTheme, type UiDriver, type UiTheme } from "@semio-tech/ui-react";
import { type DocumentOpeningReference } from "../../🧭️opening/🟦️.ts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { type ActorDocumentMessagePortV1 } from "@semio-tech/framework/backbone/binding";
import { reduceArtifactCreationProgressUiV1, type ArtifactCreationProgressUiStateV1 } from "../../🌱️artifact-creation/🟦️.tsx";
import { closeDirectoryHomeOwnerV1, type DirectoryBootstrapUiState, type DirectoryHomeOwnerV1 } from "../../📇️directory-bootstrap/🟦️.tsx";
import { useAgentBridge, type AgentAppCommandHandler, type AgentBridgeOfferScopeV1 } from "../../../🔗️AgentBridge/🟦️.tsx";
import { shellAppFault } from "@semio-tech/framework/mcp/channel";
import type { BridgeInstanceRef } from "@semio-tech/framework/mcp/bridge";
import type { ReduceResult, ShellCommand } from "@semio-tech/framework/shell";
import { LOCAL_SESSION_RECLAIM_INTERVAL_MS } from "../../🎚️shell-model/🟦️.tsx";
import { hubBootstrapOriginV1, hubSessionStorageV1, readViteSEnv } from "../../🔐️identity-binding/🟦️.ts";
import { type DirectoryCommandResultSlotV1, type SpaceArtifactCreationOwnerV1 } from "../../🧾️space-artifact-creation/🟦️.ts";
import { type ShellSpaceAdministrationStateV1 } from "../../🏠️space-administration/🟦️.ts";
import { logUnlessRetiredV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterShellCoreState } from "../../🧩️shell-core-state/🟦️.ts";
import type { BackgroundSpaceIndexSession, OpenDocumentSessionTarget, PreparedArtifactOpeningTarget } from "../🧰️identity-port-types/🟦️.ts";

/** 🆔 Binds the document access, the agent bridge, the identity, session authority and directory refs and state. */
export function useIdentityAgentBridgeAndDirectoryState({ core }: { readonly core: ShellCoreAfterShellCoreState }) {
  const {
    measuredDevice, dispatch, historyStore, uiLayout, uiDriverId, uiCustomDrivers, uiDriverDraft, uiLocale,
    uiTerminology, uiThemeId, uiCustomThemes, uiThemeDraft, refreshHistorySnapshot, dialogOpeningRef, liveDialogRef,
    openSpaceId, sessionRef, hostPorts
  } = core;
  const documentAccess = hostPorts?.documentAccess;
  const uiDevice: ElementsSurfaceDevice = measuredDevice === "desktop" ? uiLayout : measuredDevice;
  const uiTheme: UiTheme = useMemo(() => {
    if (uiThemeDraft) return uiThemeDraft;
    const found = builtinUiThemes().find((t) => t.id === uiThemeId) ?? uiCustomThemes[uiThemeId];
    return found ?? semioTheme();
  }, [uiThemeId, uiCustomThemes, uiThemeDraft]);
  const uiDriver: UiDriver = useMemo(() => uiDriverDraft ?? resolveUiDriver(uiDriverId, uiCustomDrivers), [uiDriverId, uiCustomDrivers, uiDriverDraft]);
  /** 🧵️ Lazily-created worker running `🟦️backbone-🟦️worker.ts` — one per shell instance, reused across `openDocument` calls. */
  const backboneWorkerRef = useRef<Worker | null>(null);
  const backboneWorkerStreamsDetachRef = useRef<(() => void) | null>(null);
  /** 🪪️ Per-tab session id component of the actor (contract §C0 `user:{userId}#{shellSessionId}`) —
   * stable for this tab's whole lifetime, shared by both the pre-identity `client-{id}` actor and the
   * post-sign-in `user:{userId}#{id}` one, so a tab's actor id only ever changes its PREFIX on sign-in,
   * never re-mints the suffix mid-session. */
  const shellSessionIdRef = useRef<string>(Math.random().toString(36).slice(2));
  /** 🖋️ Stable per-tab actor id for hub `Hello`/presence frames and operation-origin filtering. */
  const shellActorIdRef = useRef<string>(`client-${shellSessionIdRef.current}`);
  /** 🎛️ Applies one inbound agent `ShellCommand` to THIS host's real chrome. `useAgentBridge` runs
   * before `dispatch`/`dock` exist, so the seam is a ref the effect below fills — without it the
   * bridge reduces `ui_reveal`/`ui_focus` against its private mirror, answers the gateway `ok: true`
   * and nothing on screen moves, which is exactly what slice M7's live transcript measured. */
  const applyAgentShellCommandRef = useRef<(command: ShellCommand) => void>(() => {});
  /** 🗿️ Executes one agent artifact command against THIS host's live plugin instance. Same ref
   * rationale as {@link applyAgentShellCommandRef} — `useAgentBridge` runs thousands of lines before
   * `onAction`/`historyProjection` exist. Until the effect below fills it, the honest answer is a
   * typed refusal, never silence (a silent route costs the agent its whole wall budget). */
  const agentArtifactRouteRef = useRef<AgentAppCommandHandler>(() => [shellAppFault("plugin.unavailable", "this shell's artifact route is not mounted yet — no plugin session has been established")]);
  /** 🧾️ Prepared-but-uncommitted agent plans, by transaction id. The shell has no "compute the ops
   * without applying them" seam, so `action_prepare`'s ops are a DEFERRED plan parked here and
   * `transaction_commit` is the leg that dispatches — which is what makes a rollback a real no-op. */
  const agentDeferredPlansRef = useRef<Map<string, string>>(new Map());
  /** 📇️🪟️ The agent census, filled by the effect beside the spawned roster far below (the roster,
   * the focused program and the plugin manifests are all declared after this line, and
   * `useAgentBridge` takes a value rather than a ref). */
  const [agentBridgeCensus, setAgentBridgeCensus] = useState<readonly BridgeInstanceRef[]>([]);
  /** 📇️ What an agent may address in this shell. The gateway's `ShellArtifactChannel` resolves an
   * `AppCommand`'s target from exactly this census, so a shell with nothing open is a shell whose
   * artifact verbs answer `plugin.unavailable` by name.
   *
   * 🪟️ It is the live session PLUS every spawned program. Before this it was the session alone, which
   * made a spawned foreign editor invisible to an MCP client even while the human was working in it:
   * measured verbatim on the real `s` host with `note` spawned from the palette and its window on the
   * canvas (ticket 26/09/18, S6, `🗑️generated/s6-agent-gate-s.txt`) —
   *
   * ```
   * PLUGIN_UNAVAILABLE: capability `note.s.note.note@1/*#editor.addBlock` is owned by plugin `note`,
   * which has no open instance in the attached shell — open one there first
   * (the shell reports 1 open instance(s))
   * ```
   *
   * — that ONE instance being `s`'s own Home. It is the same session-only shape S5 found at five other
   * decision sites, on the last lane that still had it: outcome 4's agent could drive only the landing
   * app of a shell whose whole point is hosting every other plugin's artifacts.
   *
   * 🎯️ ORDER carries the focus: entry 0 is the program that owns the canvas right now. A
   * capability-less agent command has no other rule for choosing among several open instances, and
   * the shell's own answer to "which document does the user mean" is this one — which is also the
   * identity published on `data-semio-artifact-id`, so what a browser can be asked about and what
   * the agent route hands out stay the same string once more than one program is open. */
  const agentBridgeInstances = agentBridgeCensus;
  /** 🎯️ Who this shell is for the agent-bridge offer — assigned once the hub connection port exists below. */
  const agentBridgeOfferScopeRef = useRef<(signal: AbortSignal) => Promise<AgentBridgeOfferScopeV1>>(async () => null);
  const agentBridge = useAgentBridge({
    shellSessionId: shellSessionIdRef.current,
    offerScope: useCallback((signal: AbortSignal) => agentBridgeOfferScopeRef.current(signal), []),
    instances: agentBridgeInstances,
    onCommandApplied: useCallback((command: ShellCommand | null, result: ReduceResult | null) => {
      if (command && result?.ok) applyAgentShellCommandRef.current(command);
    }, []),
    onAppCommand: useCallback<AgentAppCommandHandler>((request) => agentArtifactRouteRef.current(request), []),
  });
  /** 🪪️ §C3 identity bootstrap — `null` until `DirectoryClient.me()`/`mintSession` resolves (or forever,
   * with no hub env). Mirrored into {@link shellActorIdRef}/`setPluginRuntimeActor` by the effect below,
   * never read directly for the actor id (that's always `shellActorIdRef.current`). */
  const [identity, setIdentity] = useState<Identity | null>(null);
  const identityRef = useRef<Identity | null>(null);
  identityRef.current = identity;
  /** 🪪️ True once a hub env is configured but the hub could not be reached (§C3 "keep the last
   * persisted identity, show an offline chip... never blocks the UI thread"). */
  const [identityOffline, setIdentityOffline] = useState(false);
  /** 🚫️ The hub pronounced the remembered session over (refused or expired), or the human cancelled its
   * verification: the badge then offers sign-in instead of reading a shortage it would never recover from. */
  const [hubSessionRefused, setHubSessionRefused] = useState(false);
  const [verifiedSessionAuthority, setVerifiedSessionAuthority] = useState<DirectorySessionAuthorityV1 | null>(null);
  const verifiedSessionAuthorityRef = useRef<DirectorySessionAuthorityV1 | null>(null);
  const sessionRefreshRef = useRef<DirectorySessionRefreshV1 | null>(null);
  /** 🪪️ REST-only client for the identity boot handshake (`me`/`mintSession`) — distinct from the
   * directory-lane's persistent `/directory/socket/v1` subscription, which the shell never opens itself (§C6:
   * `🏪️store/👷️worker/🟦️.ts`'s `🔖️Directory` region is the only socket owner). Re-created only if the
   * hub base url or token actually changes. */
  const hubSessionPortRef = useRef<HubSessionPortClientV1 | null>(null);
  const hubEnv = useMemo(() => {
    const hubBaseUrl = readViteSEnv("VITE_S_HUB_URL");
    const dataDir = readViteSEnv("VITE_S_DATA_DIR");
    return hubBaseUrl ? { hubBaseUrl, dataDir } : null;
  }, []);
  /** 🎫️ The one hub session this shell acts with. Seeded from the browsing context's own store, so a
   * reload continues the session the human signed into; replaced by every mint and cleared by a
   * sign-out or by the hub refusing the capability. It is the ONLY identity input below: there is no
   * second, launcher-issued credential path any more. */
  const [hubSessionCapability, setHubSessionCapability] = useState<HubSessionCapabilityV1 | null>(() =>
    hubEnv === null ? null : readHubSessionCapabilityV1(hubSessionStorageV1(), hubBootstrapOriginV1()),
  );
  /** 🎫️ The capability as the hub pane's port reads it on every request — written here, together with the state, so a
   * request sent right after a mint already carries it. */
  const hubSessionCapabilityRef = useRef(hubSessionCapability);
  const rememberHubSessionCapability = useCallback((minted: Readonly<{ token: string; userId: string }> | null) => {
    const origin = hubBootstrapOriginV1();
    const next = minted === null ? null : { origin, token: minted.token, userId: minted.userId };
    writeHubSessionCapabilityV1(hubSessionStorageV1(), next);
    hubSessionCapabilityRef.current = next;
    setHubSessionCapability(next);
  }, []);
  const localSessionClaimedAtRef = useRef(0);
  useEffect(() => {
    if (!hubEnv || (hubSessionCapability !== null && !hubSessionRefused)) return;
    if (Date.now() - localSessionClaimedAtRef.current < LOCAL_SESSION_RECLAIM_INTERVAL_MS) return;
    localSessionClaimedAtRef.current = Date.now();
    let cancelled = false;
    void (async () => {
      try {
        const response = await fetch(LOCAL_HUB_SESSION_ENDPOINT_V1, { method: "GET", credentials: "same-origin", signal: AbortSignal.timeout(5_000) });
        if (!response.ok || cancelled) return;
        const session = parseLocalHubSessionAnswerV1(await response.json());
        if (cancelled || session === null) return;
        rememberHubSessionCapability(session);
      } catch {}
    })();
    return () => {
      cancelled = true;
    };
  }, [hubEnv, hubSessionCapability, hubSessionRefused, rememberHubSessionCapability]);

  /** 🪪️ Where the hub identity stands, for the route admission ({@link shellRouteAdmissionV1}): a space route waits for
   * the human it is opened for, and a hub that does not answer opens it offline. */
  const identityResolution = shellIdentityResolutionV1({ hubConfigured: hubEnv !== null, sessionHeld: hubSessionCapability !== null, refused: hubSessionRefused, confirmed: verifiedSessionAuthority !== null && identity?.userId === verifiedSessionAuthority.userId, offline: identityOffline });
  const identityResolutionRef = useRef(identityResolution);
  identityResolutionRef.current = identityResolution;
  /** 📇️ Retained directory owner bound to the same visible Home landing instance. */
  const directoryHomeOwnerRef = useRef<DirectoryHomeOwnerV1 | null>(null);
  const directoryHomeRetirementRef = useRef<{ readonly owner: DirectoryHomeOwnerV1; readonly promise: Promise<void> } | null>(null);
  const directoryHomeOpeningRef = useRef<Promise<void>>(Promise.resolve());
  const directoryBootstrapEpochRef = useRef(0);
  const [directoryBootstrapUi, setDirectoryBootstrapUi] = useState<DirectoryBootstrapUiState>({ kind: "idle" });
  const refreshDirectoryHomeRef = useRef<(session: ActiveSession) => Promise<void>>(async () => {});
  const handleDirectoryEventPageRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-event-page" }>) => void>(() => {});
  /** 🌐️ The worker's preference-lane answers (a page, a recorded change, a failure), read through a ref because the worker
   * handler is built once. */
  const handlePreferenceLaneMessageRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "preference-lane-page" | "preference-lane-recorded" | "preference-lane-failed" }>) => void>(() => {});
  const handleDirectoryBootstrapFailureRef = useRef<(message: Extract<BackboneWorkerResponse, { readonly kind: "directory-bootstrap-failed" }>) => void>(() => {});
  const uiLocaleRef = useRef(uiLocale);
  const uiTerminologyRef = useRef(uiTerminology);
  uiLocaleRef.current = uiLocale;
  uiTerminologyRef.current = uiTerminology;
  const retireDirectoryHomeOwner = useCallback((owner: DirectoryHomeOwnerV1, worker: Worker | null): Promise<void> => {
    const current = directoryHomeRetirementRef.current;
    if (current?.owner === owner) return current.promise;
    const promise = closeDirectoryHomeOwnerV1(owner, (request) => worker?.postMessage({ wire: encodeBackboneWorkerRequest(request) }));
    directoryHomeRetirementRef.current = { owner, promise };
    void promise.finally(() => {
      if (directoryHomeRetirementRef.current?.owner === owner) directoryHomeRetirementRef.current = null;
    });
    return promise;
  }, []);
  /** 📇️ Set by the `DirectoryLane` region below once it's defined — `ensureBackboneWorker`'s
   * `onmessage` (created once, `useCallback([])`) reads this indirection rather than the callback
   * itself so a fold that depends on `session`/`hostConfig`/`onActionRef` never goes stale. */
  const dispatchDirectoryEventsRef = useRef<(events: readonly DirectoryEvent[]) => void>(() => {});
  /** ⚖️ Same ref-forwarding idiom as {@link dispatchDirectoryEventsRef} — `ensureBackboneWorker`'s
   * `remoteMutations` handling (below) needs `showTransientNotice`/`shellLabel`, both declared LATER
   * in this component (contract freeze `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
   * CONFLICTS` §C6/§C9: a peer's `MergeReport`/`Conflicts` reply to `ApplyEnvelopes` must reach the
   * Conflicts panel and, for a `"degraded"` outcome, a transient notice). Set once `applyRemoteMerge`
   * itself is defined, read only from inside the worker's `onmessage` closure body. */
  const applyRemoteMergeRef = useRef<(conflicts: readonly Conflict[] | null, mergeReport: MergeReport | null) => void>(() => {});
  /** 🧯️ Same ref-forwarding idiom as {@link applyRemoteMergeRef} — `applyHostEffects`'s `notify`
   * branch (declared ABOVE `showTransientNotice`) is the shell end of `kernel::Effect::Notify`, the
   * one channel a plugin has for telling the user its work was refused (e.g. puzzle3d's
   * `addBrushObject`/`acceptSuggestion` rejecting a colliding placement). Without this branch the
   * effect reached `wireEffectToFriendly` and was dropped, so a refusal looked exactly like nothing
   * happening. Assigned right after `showTransientNotice` itself is defined. */
  const showTransientNoticeRef = useRef<(message: string, kind?: Severity, code?: string) => void>(() => {});
  /** 📌️ Same idiom for the explicit check-in: `onAction` answers the history body's `framework.checkin` `submit` with
   * `dispatchCheckpoint`, which is declared below it in the 🔖️CheckIn region. */
  const dispatchCheckpointRef = useRef<(message: string) => void>(() => {});
  /** 🐚️ Same ref-forwarding idiom — `applyHostEffects`'s `replayShellCommand` branch replays a
   * SHELL-owned command id against shell-owned state (`dispatchOsCommand`, declared far below), never
   * into the guest, which has no window kind for chrome and refuses it `undeclared-action`. Answers
   * whether the id was actually routed; an unroutable one is a warning, never a guest dispatch. */
  const replayShellOwnedCommandRef = useRef<(commandId: string, args?: Record<string, unknown>) => boolean>(() => false);
  
  
  
  
  const backgroundSpaceIndexSessionsRef = useRef(new BackgroundDocumentSessionsV1<BackgroundSpaceIndexSession>());
  const openDocumentRef = useRef<(ref: DocumentOpeningReference, bindings?: readonly PersistenceBinding[], target?: OpenDocumentSessionTarget) => Promise<DocumentOpeningReceiptV1 | null>>(async () => null);
  const closeDocumentRef = useRef<(runtimeKey: string, clientInstanceId?: string) => void>(() => {});
  const documentAttachmentLanesRef = useRef(new WeakMap<PluginWasmHandle, Map<number, DocumentAttachmentLaneV1>>());
  const documentPortsRef = useRef(new WeakMap<PluginWasmHandle, Map<number, ActorDocumentMessagePortV1>>());
  const documentAttachmentLane = useCallback((plugin: PluginWasmHandle, instanceId: number): DocumentAttachmentLaneV1 => {
    let lanes = documentAttachmentLanesRef.current.get(plugin);
    if (lanes === undefined) { lanes = new Map(); documentAttachmentLanesRef.current.set(plugin, lanes); }
    let lane = lanes.get(instanceId);
    if (lane === undefined) {
      lane = new DocumentAttachmentLaneV1(async () => {
        const ports = documentPortsRef.current.get(plugin);
        const port = ports?.get(instanceId);
        await port?.retire().catch(logUnlessRetiredV1("[os-shell] a document port could not be retired"));
        if (port !== undefined && ports?.get(instanceId) === port) ports.delete(instanceId);
      });
      lanes.set(instanceId, lane);
    }
    return lane;
  }, []);
  const retireDocumentAttachment = useCallback(async (plugin: PluginWasmHandle, instanceId: number, clientInstanceId: string): Promise<void> => {
    const lanes = documentAttachmentLanesRef.current.get(plugin);
    const lane = lanes?.get(instanceId);
    if (lane === undefined) return;
    await lane.close(clientInstanceId);
    if (lane.idle && lanes?.get(instanceId) === lane) lanes.delete(instanceId);
  }, []);
  const openArtifactWithAppRefRef = useRef<(target: AppRef, dialect: ArtifactDialect, role: AppRole, admit?: () => boolean, publish?: boolean) => Promise<PreparedArtifactOpeningTarget | null>>(async () => null);
  const resolveArtifactOpeningRelayRef = useRef<(actionId: string, args: unknown) => ResolvedArtifactOpeningRelay | null>(() => null);
  /** 🎬️ The hub's "open an artifact whose plugin has never been loaded" resolution — see
   * {@link installActivationOwnerAndResolve}. Sync resolution stays on
   * {@link resolveArtifactOpeningRelayRef}; only this one may install. */
  const resolveArtifactOpeningWithActivationRef = useRef<(actionId: string, args: unknown) => Promise<ResolvedArtifactOpeningRelay | null>>(async () => null);
  const openReadySpaceArtifactCreationRef = useRef<(requestId: string) => void>(() => {});
  const spaceArtifactCreationOwnersRef = useRef(new Map<string, SpaceArtifactCreationOwnerV1>());
  const [spaceArtifactCreationUi, setSpaceArtifactCreationUi] = useState<ArtifactCreationProgressUiStateV1>({});
  const [spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog" }> | null>(null);
  const [spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi] = useState<Extract<BackboneWorkerResponse, { readonly kind: "space-artifact-creation-catalog-status" }> | null>(null);
  const spaceArtifactCreationCatalogRef = useRef(spaceArtifactCreationCatalog);
  const spaceArtifactCreationCatalogUiRef = useRef(spaceArtifactCreationCatalogUi);
  spaceArtifactCreationCatalogRef.current = spaceArtifactCreationCatalog;
  spaceArtifactCreationCatalogUiRef.current = spaceArtifactCreationCatalogUi;
  const cancelSpaceArtifactCreationsForRuntime = useCallback((runtimeKey: string, worker: Worker | null) => {
    const cleared: string[] = [];
    for (const [requestId, owner] of spaceArtifactCreationOwnersRef.current) {
      if (owner.runtimeKey !== runtimeKey) continue;
      spaceArtifactCreationOwnersRef.current.delete(requestId);
      cleared.push(requestId);
      if (owner.ready === null) worker?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId: owner.spaceId }) });
    }
    if (cleared.length > 0) setSpaceArtifactCreationUi((current) => cleared.reduce((next, requestId) => reduceArtifactCreationProgressUiV1(next, { kind: "cleared", requestId }), current));
  }, []);
  const cancelSpaceArtifactCreation = useCallback((requestId: string, spaceId: string) => {
    const owner = spaceArtifactCreationOwnersRef.current.get(requestId);
    if (owner === undefined || owner.spaceId !== spaceId || owner.cancelRequested || owner.opening || owner.ready !== null) return;
    spaceArtifactCreationOwnersRef.current.set(requestId, { ...owner, cancelRequested: true });
    setSpaceArtifactCreationUi((current) => reduceArtifactCreationProgressUiV1(current, { kind: "cancel-requested", requestId, spaceId }));
    backboneWorkerRef.current?.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "space-artifact-create-cancel", requestId, spaceId }) });
  }, []);
  /** 📇️ Offline-queue depth surfaced by the worker's `directory-status` — not yet rendered by any
   * chrome this lane owns (2-F/3-A's "row shows 'pending'" territory); kept as local state so a
   * consumer can read it once that chrome lands, with zero further plumbing here. */
  const [, setDirectoryPendingCommands] = useState(0);
  /** 🧾️ Retained, request-id-keyed command result slot. A receipt lands here BEFORE any accepted
   * event is folded, so a completion always has an owner; a one-shot invite capability lives only
   * in this slot until an explicit administration copy handler consumes it, and is never logged,
   * folded into Home, or put into React state telemetry. */
  const directoryCommandResultsRef = useRef(new Map<string, DirectoryCommandResultSlotV1>());
  /** 🏛️ The one Shell-owned administration pane state. It is filled ONLY from the worker's retained
   * `DirectoryAdministrationOperation`: the exact canonical page bytes the hub sealed, the phase,
   * and whether a one-shot invite capability is still held by the worker. No role, session identity,
   * bearer, cursor key, or invite token is ever stored here — a terminal phase arrives with the page
   * already erased, so an unmount, identity change, 401/403 or scoped 4401 clears the pane. */
  const [spaceAdministration, setSpaceAdministration] = useState<ShellSpaceAdministrationStateV1 | null>(null);
  const [hubWorkspaceOpen, setHubWorkspaceOpen] = useState(false);
  return { hubSessionCapabilityRef, rememberHubSessionCapability, agentBridgeOfferScopeRef, hubEnv, openSpaceId, spaceAdministration, closeDocumentRef, backboneWorkerRef, applyRemoteMergeRef, documentPortsRef, documentAttachmentLane, spaceArtifactCreationCatalogRef, spaceArtifactCreationCatalogUiRef, sessionRef, historyStore, refreshHistorySnapshot, liveDialogRef, dispatch, dialogOpeningRef, identityRef, documentAccess, uiDevice, uiTheme, uiDriver, backboneWorkerStreamsDetachRef, shellSessionIdRef, shellActorIdRef, applyAgentShellCommandRef, agentArtifactRouteRef, agentDeferredPlansRef, setAgentBridgeCensus, agentBridgeInstances, agentBridge, identity, setIdentity, identityOffline, setIdentityOffline, hubSessionRefused, setHubSessionRefused, verifiedSessionAuthority, setVerifiedSessionAuthority, verifiedSessionAuthorityRef, sessionRefreshRef, hubSessionPortRef, hubSessionCapability, identityResolution, identityResolutionRef, directoryHomeOwnerRef, directoryHomeRetirementRef, directoryHomeOpeningRef, directoryBootstrapEpochRef, directoryBootstrapUi, setDirectoryBootstrapUi, refreshDirectoryHomeRef, handleDirectoryEventPageRef, handlePreferenceLaneMessageRef, handleDirectoryBootstrapFailureRef, uiLocaleRef, uiTerminologyRef, retireDirectoryHomeOwner, dispatchDirectoryEventsRef, showTransientNoticeRef, dispatchCheckpointRef, replayShellOwnedCommandRef, backgroundSpaceIndexSessionsRef, openDocumentRef, documentAttachmentLanesRef, retireDocumentAttachment, openArtifactWithAppRefRef, resolveArtifactOpeningRelayRef, resolveArtifactOpeningWithActivationRef, openReadySpaceArtifactCreationRef, spaceArtifactCreationOwnersRef, spaceArtifactCreationUi, setSpaceArtifactCreationUi, spaceArtifactCreationCatalog, setSpaceArtifactCreationCatalog, spaceArtifactCreationCatalogUi, setSpaceArtifactCreationCatalogUi, cancelSpaceArtifactCreationsForRuntime, cancelSpaceArtifactCreation, setDirectoryPendingCommands, directoryCommandResultsRef, setSpaceAdministration, hubWorkspaceOpen, setHubWorkspaceOpen };
}
