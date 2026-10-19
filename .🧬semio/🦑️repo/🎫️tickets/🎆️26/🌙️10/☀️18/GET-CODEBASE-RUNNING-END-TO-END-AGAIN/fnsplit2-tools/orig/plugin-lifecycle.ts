/** 🚀️ Primary session establishment, plugin install, reload and uninstall, hub program installs and the extension ledger with its install, uninstall and enable operations.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `usePluginLifecycle` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

// #region 🔌️Adapters
import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  useSyncExternalStore
} from "react";
import {
  createBundledPluginSource,
  createDevPluginSource,
  createExtensionSource,
  extensionRegistryFromCatalog,
  multiplexPluginSources,
  type PluginModuleAcquired,
  type PluginSource,
  type ProgramHotSwapEvent,
  retireContributorInstances,
  resolvePlaygroundDefaultAppId,
  resolvePluginRegistryId,
  panelViewContext
} from "@semio-tech/framework";
import { DEV_STREAM_ROUTES } from "@semio-tech/framework/backbone";
import { resolveBootPrimaryAppV1 } from "../🔀️surface-switch/🟦️.ts";
import { type UiStatus } from "@semio-tech/ui-react";
import { type LoadedProgramState, resolvePluginCanvasStatus } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { upsertLoadedProgramV1 } from "../../🐚️Shell/♻️slice-reducers/🟦️.ts";
import type { ViewModel } from "../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { EMPTY_APP_LABELS_OVERLAY, shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import {
  applyFrameworkLayoutSeed,
  loadPluginModuleResilient,
  pluginInstallProgressTotalV1,
  pluginModuleStoreNoticeTextV1,
  pluginShouldEstablishSession,
  type PluginInstallOutcome
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { buildActiveUtilityByWindowId, sessionWindowInstances } from "../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { buildSpacePanelState, panelJsonFromState, parsePanelState } from "../../🛠️ShellHelpers/📌️panel/🟦️.ts";
// #endregion 🔌️Adapters
import { onPluginInstancesLost, PluginBootShardLostError, type PluginWasmHandle } from "../../🔌️PluginRuntime/🟦️.tsx";
import { isShardLostError } from "@semio-tech/framework/actor/shard-client/heartbeat";
import { streamMuxWatchV1 } from "@semio-tech/framework/io/stream-mux";
import type { HubPluginSourceV1 } from "@semio-tech/framework/plugin-module/hub-source/port";
import { hubCatalogClosureV1, hubCatalogOwnerOfDialectV1, hubProgramIdV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import type { TrustedPluginModuleIndexEntryV1 } from "@semio-tech/framework/plugin-module";
import { resolveRequiredHostApps } from "../🪪️host-bootstrap/🟦️.tsx";
import { commandStallWatch, registerProgramExtensionV1 } from "../../🔌️PluginRuntime/🟦️.tsx";
import { seededActiveWindowId } from "../🎚️shell-model/🟦️.tsx";
import { encodeWindowActionInvocation } from "../📨️window-invocation/🟦️.ts";
import { announcedPageStreams, reloadRetainsActiveApp } from "../🔐️identity-binding/🟦️.ts";
import { requiredHostPanelLeafId, withLocalizedWindowKindLabels } from "../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterDirectoryAndRoute } from "../🛂️directory-route/🟦️.ts";

//#region FrameworkOsShell
/** 🧩️ Durable-in-session extension ledger mirror — space document ops are dispatched
 * best-effort when a space/studio session can accept them; this state keeps Settings + contribution
 * filtering coherent even when the space app is not the active session. */
type ExtensionLedgerEntry = {
  readonly extensionId: string;
  readonly version: string;
  readonly sourceUri: string;
  readonly packageHash: string;
  readonly enabled: boolean;
  readonly extendsHost: string;
};

export type ShellCoreAfterPluginLifecycle = ShellCoreAfterDirectoryAndRoute & ReturnType<typeof usePluginLifecycle>;

export function usePluginLifecycle(core: ShellCoreAfterDirectoryAndRoute) {
  const {
    catalog, pluginFilter, appId, appRole, hostConfig, hostMode, dispatch, pluginStatusById, pluginSupervisorById,
    session, error, contributorInstancesRef, extraWindowCounterRef, extraWindowInstancesRef, rememberOpenSpaceId,
    openInstanceIdRef, sessionRef, shellRouteLedgerRef, backboneWorkerRef, hubEnv, directoryHomeOwnerRef, uiLocaleRef,
    uiTerminologyRef, retireDirectoryHomeOwner, showTransientNoticeRef, extensionFetchAbortRef, loadedPluginsRef,
    pluginModuleUrlByIdRef, pluginArtifactRebuiltAtRef, recordPluginArtifactRebuiltAt, pluginOpInFlightRef,
    pluginInstallAbortsRef, pluginInstallProgressById, notePluginInstallProgress, registry, hostPorts
  } = core;
  const extensionRoute = hostPorts?.pluginDistribution?.extensionRoute;
  const extensionStoreUrl = (suffix: string): string => {
    if (extensionRoute === undefined) throw new Error("extension-route-absent: the composition root supplied no plugin distribution port");
    return `${extensionRoute}${suffix}`;
  };
  //#region 🔌️PluginRuntime
  /** 🔌️ The one registry entry the shell must have loaded before it can create a session — the studio
   * host plugin (`hostConfig.pluginId`) in studio mode, otherwise the resolved single-app variant.
   * Every other registry entry streams in independently and is never fatal to boot. */
  const primaryPluginId = useMemo(() => hostConfig?.pluginId ?? (pluginFilter ? resolvePluginRegistryId(catalog, pluginFilter) : undefined) ?? registry[0]?.pluginId, [catalog, hostConfig, pluginFilter, registry]);
  const shellPluginCanvasStatus = useMemo((): UiStatus | undefined => {
    return resolvePluginCanvasStatus(!!session, error, primaryPluginId ? pluginStatusById[primaryPluginId] : undefined, primaryPluginId ? pluginSupervisorById[primaryPluginId] : undefined);
  }, [session, error, primaryPluginId, pluginStatusById, pluginSupervisorById]);
  /** 🌎️ A shell with a hub installs a module this device does not serve from the hub's trusted catalog into its
   * persisted plugin module store (the plugin distribution port's hub source); `null` without a hub. Its notices reach the person
   * through the shell's own transient notice, in their language. */
  const hubPluginSource: HubPluginSourceV1 | null = useMemo(
    () =>
      hubEnv
        ? hostPorts?.pluginDistribution?.hubPluginSource({
            onNotice: (notice) => showTransientNoticeRef.current(pluginModuleStoreNoticeTextV1(notice, uiLocaleRef.current), notice.kind === "quota-exceeded" ? "error" : "info", `plugin-module-store.${notice.kind}`),
          }) ?? null
        : null,
    [hubEnv, hostPorts?.pluginDistribution],
  );
  /** 🔌️ Dev hosts stream availability on the page's stream channel (`DEV_STREAM_ROUTES`, one WebSocket for every stream of
   * the page and its workers, never an HTTP/1.1 connection held per stream); shipped static bundles replay one immediate
   * `snapshot` per tree ({@link createBundledPluginSource}) so dependency contributors install without
   * Vite middleware. The modules staged beside the shell answer first; the hub source answers what this
   * device does not serve — local-first, and only when something opens it: the hub source announces nothing. */
  const extensionRegistry = useMemo(() => extensionRegistryFromCatalog(catalog), [catalog]);
  const pluginSource: PluginSource = useMemo(() => {
    const local = import.meta.env.PROD
      ? [createBundledPluginSource(registry), createBundledPluginSource(extensionRegistry)]
      : [
          createDevPluginSource(registry, streamMuxWatchV1(announcedPageStreams(), DEV_STREAM_ROUTES.pluginModules)),
          createExtensionSource(catalog, streamMuxWatchV1(announcedPageStreams(), DEV_STREAM_ROUTES.extensionModules)),
        ];
    return multiplexPluginSources(...local, ...(hubPluginSource ? [hubPluginSource] : []));
  }, [catalog, extensionRegistry, registry, hubPluginSource]);

  /** 🔌️ Recreates the primary session instance for `handle` — the exact `hostConfig`/non-studio
   * app-resolution logic the boot effect used to run once inline, now shared with `reloadPlugin` so a
   * hot-swap of the session-owning plugin re-establishes the session the same way boot does.
   *
   * 🗣️ Locale and terminology are read through their refs, and are deliberately NOT dependencies. They
   * only label the layout SEED, i.e. they matter at the instant a session is established — which is
   * exactly what a ref answers — while an already-established session's baked-in titles are kept in the
   * current language by the retitle effect below (`syncShellLabelLocale`'s neighbour). Depending on them
   * reactively made this callback's identity change on every language switch, which propagated through
   * `installPlugin`/`reloadPlugin` into the `PluginSource` subscription effect's dep array; that effect's
   * re-run opens a FRESH `EventSource`, whose connect-time `snapshot` replays every already-built plugin
   * as a hot-swap, and a hot-swap of the session-owning plugin destroys the live instance —
   * `actor-activation.revoked` followed by `no channel for instance N` for everything still addressed to
   * it (ticket 26/09/02 wave B38 §1.7, fixed in wave B40). A UI-language change is a view-context
   * update; it must never reach the plugin install graph. */
  const establishPrimarySession = useCallback(
    async (handle: PluginWasmHandle) => {
      const manifest = handle.manifest;
      if (hostConfig) {
        // 🏛️ BOTH apps come from the manifest in hand, never from the `hostApp` render closure: this
        // callback runs from `installPlugin` immediately after `UPSERT_LOADED_PLUGIN` is dispatched, so
        // `loadedPlugins` — and therefore `hostPlugin`/`hostApp`, which derive from it — is still the
        // pre-commit array and `hostApp` is `undefined`. Reading it here threw "configured host app has
        // no panel leaf" and killed the whole boot, which is why a host-mode shell (`dev s`) never
        // reached a ready beacon while every single-plugin playground, whose `hostConfig` is null and
        // which therefore takes the branch below, booted fine. `handle` IS the host plugin here — the
        // `.landing` resolution on the next line already depends on that — so one resolution answers both.
        const hostApps = resolveRequiredHostApps(manifest.apps, hostConfig);
        const sApp = hostApps.landing;
        const panelState = buildSpacePanelState([], requiredHostPanelLeafId(hostApps.host));
        const instanceId = await handle.createApp(sApp.id);
        rememberOpenSpaceId(null);
        openInstanceIdRef.current = null;
        shellRouteLedgerRef.current.invalidate();
        const viewState: ViewModel = { activeModeId: sApp.defaultModeId ?? sApp.modes[0]?.id, panelJson: panelJsonFromState(panelState) };
        // 🪟️ Seed default-layout panes (Top/Perspective) before any effect can fire actions — otherwise
        // boot `setActiveExample` races the session-switch refresh and wipes pane bodies.
        const seeded = applyFrameworkLayoutSeed(sApp.defaultLayout, withLocalizedWindowKindLabels(sApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
        extraWindowInstancesRef.current = seeded.extraInstances;
        extraWindowCounterRef.current = seeded.extraInstances.length;
        dispatch({ type: "SET_SESSION", value: { pluginId: handle.pluginId, instanceId, app: sApp, viewState } });
        dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
        dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
        dispatch({ type: "SET_ERROR", value: null });
        return;
      }
      // 👁️✏️ Boot-time role (contract freeze §5) resolved against the app-id axis by
      // `resolveBootPrimaryAppV1` — the role of an OPEN session always comes from `session.app.role`,
      // never `appRole` itself. `appId`/`defaultAppId` name the artifact surface; `appRole` picks
      // which of that dialect's surfaces opens.
      const defaultAppId = pluginFilter ? resolvePlaygroundDefaultAppId(catalog, pluginFilter) : undefined;
      const primaryApp = resolveBootPrimaryAppV1(manifest.apps, appId, defaultAppId, appRole);
      if (appId !== undefined && primaryApp === undefined) {
        // 🔐️ A pinned app belongs to the selected primary aggregate, never a racing dependency.
        throw new Error(`primary plugin ${handle.pluginId} does not declare pinned app ${appId}`);
      }
      if (!primaryApp) return;
      const instanceId = await handle.createApp(primaryApp.id);
      const seeded = applyFrameworkLayoutSeed(primaryApp.defaultLayout, withLocalizedWindowKindLabels(primaryApp.windowKinds), EMPTY_APP_LABELS_OVERLAY, uiTerminologyRef.current, uiLocaleRef.current);
      extraWindowInstancesRef.current = seeded.extraInstances;
      extraWindowCounterRef.current = seeded.extraInstances.length;
      dispatch({
        type: "SET_SESSION",
        value: { pluginId: handle.pluginId, instanceId, app: primaryApp, viewState: { activeModeId: primaryApp.defaultModeId ?? primaryApp.modes[0]?.id } },
      });
      dispatch({ type: "SET_EXTRA_WINDOW_INSTANCES", value: seeded.extraInstances });
      dispatch({ type: "SET_SHELL_LAYOUT", value: seeded.modeLayout });
      dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: seededActiveWindowId(seeded.modeLayout) });
      dispatch({ type: "SET_ERROR", value: null });
    },
    [catalog, hostConfig, appId, appRole, pluginFilter, rememberOpenSpaceId],
  );

  /** 🚑️ ONE watchdog kill must not be a fatal boot. Losing the shard that was running the primary
   * plugin's first turn is a recoverable event by construction — `ShardClient.rebuild` has already
   * replaced the worker and `ActivationRegistry.handleShardLost` has already bumped and resumed every
   * actor that was on it — so the session is established once more against the rebuilt shard before
   * anything is reported. Only a SECOND loss is real, and it fails as a typed
   * {@link PluginBootShardLostError} (`plugin.boot.shard-lost`) that names the cause instead of
   * surfacing a bare `shard 0 terminated`. Every other boot error propagates untouched and unretried. */
  const establishPrimaryWithShardRetry = useCallback(
    async (handle: PluginWasmHandle): Promise<void> => {
      try {
        await establishPrimarySession(handle);
      } catch (bootError) {
        if (!isShardLostError(bootError)) throw bootError;
        try {
          await establishPrimarySession(handle);
        } catch (retryError) {
          if (!isShardLostError(retryError)) throw retryError;
          throw new PluginBootShardLostError(handle.pluginId, retryError);
        }
      }
    },
    [establishPrimarySession],
  );

  /** 🚑️ A LIVE session whose worker was taken down is recoverable exactly the way a killed boot is:
   * `ShardClient.rebuild` has already replaced the worker and `PluginRuntime` has already retired the
   * dead instance's bookkeeping (see `onPluginInstancesLost`), so a fresh `createApp` on the rebuilt
   * shard re-opens the document and the session continues.
   *
   * Without this the shard restore brought the ACTOR back and nothing else: the guest returned empty,
   * every later command page failed its liveness check with `plugin.command-page-invalid`, and the
   * session was dead until the user reloaded — measured on 2026-09-12 after a 16 s `brep.bool.cut`
   * tripped the watchdog (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
   * `📓️extension-evaluate-budget-2026-09-12.md`).
   *
   * If the rebuild ALSO fails there is nothing left to try in-page, so the shell says so in the
   * user's own language rather than leaving a session that answers nothing. */
  useEffect(() => {
    return onPluginInstancesLost((_shardIndex, lost) => {
      const session = sessionRef.current;
      if (!session) return;
      const ours = lost.find((entry) => entry.pluginId === session.pluginId && entry.instanceId === session.instanceId);
      if (!ours) return;
      const handle = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === ours.pluginId)?.handle;
      if (!handle) {
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
        return;
      }
      void establishPrimaryWithShardRetry(handle).catch((error: unknown) => {
        console.error(`ShellHost: could not re-establish ${ours.pluginId} after a worker loss`, error);
        dispatch({ type: "SET_ERROR", value: shellLabel("ui.common.workerLost") });
      });
    });
  }, [establishPrimaryWithShardRetry]);

  /** 🔌️ Installs a registry entry that isn't loaded yet: acquires its module (worker-backed, refcounted
   * — see `acquirePluginModule`), upserts it into `loadedPlugins`, and — if this is the primary plugin
   * and no session exists yet — establishes the session. Shared by the boot effect (primary plugin
   * only) and the `PluginSource` subscription effect (every other plugin, as its build lands).
   * `loadedPluginsRef` advances with the upsert, not a render later, so a caller that awaited this install
   * (an opening that first installs its kind's owner) routes over the set that already holds it. */
  const installPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number): Promise<PluginInstallOutcome> => {
      if (pluginOpInFlightRef.current.has(pluginId)) return "in-flight";
      if (loadedPluginsRef.current.some((entry) => entry.handle.pluginId === pluginId)) return "already-loaded";
      pluginOpInFlightRef.current.add(pluginId);
      // 👷️ The store's service worker must control the page before the first plugin worker exists, or no plugin worker
      // could ever load a stored module. A hub whose store cannot start leaves every local install untouched.
      await hubPluginSource?.ready().catch((error: unknown) => console.warn("plugin module store unavailable", error));
      const entry = registry.find((candidate) => candidate.pluginId === pluginId) ?? (await pluginSource.list().catch(() => [])).find((candidate) => candidate.pluginId === pluginId);
      if (!entry) {
        pluginOpInFlightRef.current.delete(pluginId);
        return "missing-registry";
      }
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "installing" });
      // 🛑️ One abort per install, registered while it runs so the install band's cancel control has
      // something to fire. `"installing"` is already the phase the band reads; this is the half that
      // was missing — a long-running phase nobody could stop.
      const abort = new AbortController();
      pluginInstallAbortsRef.current.set(pluginId, abort);
      try {
        let acquired: PluginModuleAcquired;
        try {
          acquired = await pluginSource.acquireModule(pluginId, rebuiltAt, { signal: abort.signal, onProgress: (progress) => notePluginInstallProgress(pluginId, progress) });
        } catch (error) {
          if (abort.signal.aborted) console.debug(`plugin install cancelled ${pluginId}`);
          else console.error("program module unavailable", pluginId, error);
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        const handle = await loadPluginModuleResilient(pluginId, acquired.moduleUrl, abort.signal);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "failed" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
          return "failed";
        }
        pluginModuleUrlByIdRef.current.set(pluginId, acquired.moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, acquired.rebuiltAt);
        loadedPluginsRef.current = upsertLoadedProgramV1(loadedPluginsRef.current, { handle, manifest: handle.manifest });
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "loaded" });
        // 🔐️ Only the configured primary may own the session; dependencies can expose the same
        // app ids and arrive first because registry installation is intentionally concurrent.
        const shouldEstablish = pluginShouldEstablishSession(pluginId, primaryPluginId, sessionRef.current !== null);
        if (shouldEstablish) {
          try {
            await establishPrimaryWithShardRetry(handle);
            dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "running" });
          } catch (bootError) {
            console.error("Framework OS boot failed", bootError);
            dispatch({ type: "SET_ERROR", value: bootError instanceof Error ? bootError.message : String(bootError) });
            return "failed";
          }
        }
        return "loaded";
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
        notePluginInstallProgress(pluginId, null);
        if (pluginInstallAbortsRef.current.get(pluginId) === abort) pluginInstallAbortsRef.current.delete(pluginId);
      }
    },
    [registry, pluginSource, hubPluginSource, primaryPluginId, establishPrimaryWithShardRetry, appId, recordPluginArtifactRebuiltAt, notePluginInstallProgress],
  );

  /** 🎬️ Every plugin whose install is running right now, from the SAME `"installing"` status the
   * install path already dispatches — no second notion of "an install is happening" to keep in sync. */
  const installingPluginIds = useMemo(() => Object.keys(pluginStatusById).filter((pluginId) => pluginStatusById[pluginId] === "installing").sort(), [pluginStatusById]);
  const pluginInstallProgress = useMemo(() => pluginInstallProgressTotalV1(pluginInstallProgressById, installingPluginIds), [pluginInstallProgressById, installingPluginIds]);

  /** 🛑️ Stops every install currently in flight — what the install band's one cancel control does.
   * Each aborted load settles as a refusal through `loadPluginModuleResilient`'s own `null`, so the
   * plugin lands in `"failed"` exactly as any other refused install would, with no second path. */
  const cancelPluginInstalls = useCallback(() => {
    for (const abort of pluginInstallAbortsRef.current.values()) abort.abort();
  }, []);

  /** ⏱️ Every command turn holding its program's lane past the stall bound (`⏱️command-stall`), with a once-a-second clock
   * while any is held so the band's seconds count up; its cancel rejects that turn and releases the lane. */
  const commandStalls = useSyncExternalStore(commandStallWatch.subscribe, commandStallWatch.stalls);
  const [commandStallClockMs, setCommandStallClockMs] = useState(() => performance.now());
  useEffect(() => {
    if (commandStalls.length === 0) return;
    setCommandStallClockMs(performance.now());
    const timer = setInterval(() => setCommandStallClockMs(performance.now()), 1_000);
    return () => clearInterval(timer);
  }, [commandStalls.length]);

  /** 🌎️ Installs one module of a catalog generation as a hub program (`🌎️hub-source/🔍️resolution`) — from the store, this
   * device's byte-identical staged module, or the hub — with the same install band, byte progress and cancel as every
   * other install, and loads it beside the device's own program of that plugin. Concurrent openings join one install. */
  const hubProgramInstallsRef = useRef(new Map<string, Promise<LoadedProgramState | null>>());
  const installHubProgram = useCallback(
    (generationId: string, entry: TrustedPluginModuleIndexEntryV1, peers: Readonly<Record<string, string>>): Promise<LoadedProgramState | null> => {
      const programId = hubProgramIdV1(entry.pluginId, entry.bundleSha256);
      const loaded = loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === programId);
      if (loaded !== undefined) return Promise.resolve(loaded);
      const running = hubProgramInstallsRef.current.get(programId);
      if (running !== undefined) return running;
      const install = (async (): Promise<LoadedProgramState | null> => {
        if (hubPluginSource === null) return null;
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "installing" });
        const abort = new AbortController();
        pluginInstallAbortsRef.current.set(programId, abort);
        try {
          const staged = registry.find((candidate) => candidate.pluginId === entry.pluginId) ?? extensionRegistry.find((candidate) => candidate.pluginId === entry.pluginId);
          const program = await hubPluginSource.installProgram(generationId, entry, staged?.moduleUrl ?? null, { signal: abort.signal, onProgress: (progress) => notePluginInstallProgress(programId, progress) });
          const handle = await loadPluginModuleResilient(programId, program.moduleUrl, abort.signal, entry.pluginId);
          if (!handle) {
            dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "failed" });
            return null;
          }
          const state: LoadedProgramState = { handle, manifest: handle.manifest, catalogModule: { pluginId: entry.pluginId, generationId, bundleSha256: entry.bundleSha256, source: program.source, peers } };
          console.info(`[os-shell] hub program ${programId} of generation ${generationId} loaded from ${program.source}`);
          loadedPluginsRef.current = upsertLoadedProgramV1(loadedPluginsRef.current, state);
          dispatch({ type: "UPSERT_LOADED_PLUGIN", value: state });
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "loaded" });
          return state;
        } catch (error) {
          if (abort.signal.aborted) console.debug(`hub program install cancelled ${programId}`);
          else console.error("hub program unavailable", programId, error);
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: programId, value: "failed" });
          return null;
        } finally {
          notePluginInstallProgress(programId, null);
          if (pluginInstallAbortsRef.current.get(programId) === abort) pluginInstallAbortsRef.current.delete(programId);
        }
      })();
      hubProgramInstallsRef.current.set(programId, install);
      void install.finally(() => {
        if (hubProgramInstallsRef.current.get(programId) === install) hubProgramInstallsRef.current.delete(programId);
      });
      return install;
    },
    [hubPluginSource, registry, extensionRegistry, notePluginInstallProgress],
  );

  /** 🌎️ The program a hub document of `artifactKind` runs on: the hub catalog's current generation names the package
   * that opens the kind; that package and its same-generation closure (dependencies, extensions) are installed as hub
   * programs, dependencies first, and every extension is indexed under its parent program before anything activates.
   * `null` while the hub is unreachable, when no single package opens the kind, or when an install fails. */
  const installHubDocumentProgram = useCallback(
    async (artifactKind: string): Promise<LoadedProgramState | null> => {
      if (hubPluginSource === null) return null;
      const index = await hubPluginSource.catalog(new AbortController().signal);
      if (index === null) return null;
      const owner = hubCatalogOwnerOfDialectV1(index, artifactKind);
      const closure = owner === null ? null : hubCatalogClosureV1(index, owner.pluginId);
      if (owner === null || closure === null) return null;
      const peers: Readonly<Record<string, string>> = Object.freeze(Object.fromEntries(closure.map((entry) => [entry.pluginId, hubProgramIdV1(entry.pluginId, entry.bundleSha256)])));
      for (const entry of closure) if ((await installHubProgram(index.generationId, entry, peers)) === null) return null;
      for (const entry of closure) if (entry.extendsPluginId !== null) registerProgramExtensionV1(peers[entry.extendsPluginId]!, peers[entry.pluginId]!);
      return loadedPluginsRef.current.find((candidate) => candidate.handle.pluginId === peers[owner.pluginId]) ?? null;
    },
    [hubPluginSource, installHubProgram],
  );

  /** 🔌️ Hot-swaps an already-loaded plugin to a newly built module — mirrors the os-core kernel's
   * `PluginHost::hot_swap_plugin` contract (validate → destroy affected instances → swap → recreate the
   * session if it was this plugin's → release the old module) without inventing a separate one:
   * acquires the new module BEFORE tearing anything down (the old handle keeps serving concurrent
   * traffic during the swap), validates that a session-owning plugin still declares the session's app
   * id, then only commits. Extension-only programs intentionally declare no apps. A validation failure
   * disposes the new lease and leaves the old plugin exactly as it was — nothing destroyed, status back
   * to `"loaded"`. */
  const reloadPlugin = useCallback(
    async (pluginId: string, rebuiltAt?: number) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return installPlugin(pluginId, rebuiltAt);
      pluginOpInFlightRef.current.add(pluginId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "reloading" });
      dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "restarting" });
      let newHandle: PluginWasmHandle | null = null;
      let committed = false;
      let ownsSession = false;
      try {
        const acquired = await pluginSource.acquireModule(pluginId, rebuiltAt, { signal: new AbortController().signal });
        newHandle = await loadPluginModuleResilient(pluginId, acquired.moduleUrl);
        if (!newHandle) throw new Error(`program ${pluginId} failed to reload`);
        const activeSession = sessionRef.current;
        ownsSession = activeSession?.pluginId === pluginId;
        const activeAppId = ownsSession ? activeSession?.app.id : undefined;
        if (!reloadRetainsActiveApp(newHandle.manifest.apps, activeAppId)) {
          throw new Error(`program ${pluginId} reload dropped the active session's app "${activeAppId}"`);
        }

        const oldAppIds = new Set(current.manifest.apps.map((app) => app.id));
        const newAppIds = new Set(newHandle.manifest.apps.map((app) => app.id));
        const hotSwapEvent: ProgramHotSwapEvent = {
          pluginId,
          version: newHandle.manifest.version,
          addedApps: [...newAppIds].filter((id) => !oldAppIds.has(id)),
          removedApps: [...oldAppIds].filter((id) => !newAppIds.has(id)),
        };

        const directoryHomeOwner = directoryHomeOwnerRef.current;
        if (directoryHomeOwner?.plugin.pluginId === pluginId) {
          directoryHomeOwnerRef.current = null;
          const worker = backboneWorkerRef.current;
          await retireDirectoryHomeOwner(directoryHomeOwner, worker);
        }

        // 🪦️ Destroy this plugin's live instances under the OLD handle before swapping — the primary
        // session instance (if owned), every studio-spawned instance, and any external-slot contributor
        // instance. Mirrors the shell-unmount teardown effect, scoped to one pluginId instead of every
        // loaded plugin.
        if (ownsSession && activeSession) {
          // 🪦️ The session is retired BEFORE its instance is destroyed, and the plugin stays
          // `reloading` until the successor session exists (below). Between those two points the shell
          // used to keep rendering the windows of a session whose instance was already gone: the
          // node-graph host remounted against the RETIRED instance, its boot `interactionSelect`,
          // `readLocalInteraction` and opening-camera dispatch all answered
          // `no channel for instance N`, `attachSurface` rejected, and the Flow window stayed blank for
          // the rest of the page because nothing re-attaches a surface whose effect is keyed on a
          // `surfaceId` that did not change (measured on :6023,
          // `📓️hot-swap-board-remount-2026-09-15.md` §1).
          dispatch({ type: "SET_SESSION", value: null });
          await current.handle.destroyApp(activeSession.instanceId).catch(() => {});
        }
        for (const spawned of core.spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === pluginId);
        if (hostMode && activeSession) {
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }

        pluginModuleUrlByIdRef.current.set(pluginId, acquired.moduleUrl);
        recordPluginArtifactRebuiltAt(pluginId, acquired.rebuiltAt);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
        committed = true;

        // 🪟️ The successor session FIRST, the live canvas after it. `SET_PLUGIN_STATUS "loaded"` is what
        // `resolvePluginCanvasStatus` reads to stop showing the reload state, so announcing it before
        // `establishPrimarySession` re-opened the whole window layout on the predecessor's retired
        // instance — one wasted mount per hot swap, every dispatch it makes refused.
        if (ownsSession) await establishPrimarySession(newHandle);
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });

        // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (H1-react) — `evictPluginModule`'s refcounted
        // module-URL lease pool is gone (packet H2's "must not exist" list); disposing the OLD handle
        // already tears down every actor it ever activated via `ShardClient.dispose`, which is the
        // real replacement — there is no separate shared-module resource left to evict.
        await current.handle.dispose();
      } catch (error) {
        // 🚧️ Past the commit point there is nothing to roll BACK to: the store already names the new
        // handle and the session already runs on it, so disposing it here revoked the activation the
        // shell had just adopted and every later call answered `plugin-handle.closed`. A predecessor
        // that refuses to retire (`plugin-handle.retirement-failed`, raised by a close ladder that
        // faulted) is a leak to report, never a reason to destroy the successor
        // (ticket 26/09/02 wave B17).
        if (committed) {
          console.warn(`hot-swap ${pluginId} retained its committed handle; predecessor retirement failed`, error);
          // 🚦️ The status announcement moved AFTER `establishPrimarySession`, so a failure between the
          // two would otherwise leave the plugin reading `reloading` for the rest of the page.
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
        } else {
          await newHandle?.dispose();
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });
          dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: "crashed" });
        }
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [installPlugin, establishPrimarySession, hostMode, pluginSource, retireDirectoryHomeOwner, recordPluginArtifactRebuiltAt],
  );

  /** 🔌️ Removes an already-loaded plugin: refuses the host/primary plugin and whichever plugin owns the
   * active session (there is nothing to fall back to), otherwise destroys its live instances the same
   * way `reloadPlugin` does, drops it from `loadedPlugins`, and evicts its module lease immediately
   * (rather than the pool's normal 30s linger — freeing it right away is the point of an explicit
   * uninstall). */
  const uninstallPlugin = useCallback(
    async (pluginId: string) => {
      if (pluginOpInFlightRef.current.has(pluginId)) return;
      const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === pluginId);
      if (!current) return;
      if (pluginId === primaryPluginId) {
        return;
      }
      if (sessionRef.current?.pluginId === pluginId) {
        return;
      }
      pluginOpInFlightRef.current.add(pluginId);
      try {
        for (const spawned of core.spawnedAppsRef.current.filter((entry) => entry.pluginId === pluginId)) {
          await current.handle.destroyApp(spawned.instanceId).catch(() => {});
        }
        await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === pluginId);
        if (hostMode && sessionRef.current) {
          const activeSession = sessionRef.current;
          const currentPanel = parsePanelState(activeSession.viewState);
          const dropped = currentPanel?.spawnedApps.filter((entry) => entry.pluginId === pluginId) ?? [];
          if (currentPanel && dropped.length > 0) {
            const survivingSpawned = currentPanel.spawnedApps.filter((entry) => entry.pluginId !== pluginId);
            const activeSpawnedId = currentPanel.activeSpawnedId && dropped.some((entry) => entry.id === currentPanel.activeSpawnedId) ? undefined : currentPanel.activeSpawnedId;
            const nextPanel = { ...currentPanel, spawnedApps: survivingSpawned, activeSpawnedId };
            dispatch({
              type: "SET_SESSION",
              value: (nextSession) => (nextSession ? { ...nextSession, viewState: { ...nextSession.viewState, panelJson: panelJsonFromState(nextPanel) } } : nextSession),
            });
          }
        }
        dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "available" });
        await current.handle.dispose();
        pluginModuleUrlByIdRef.current.delete(pluginId);
        pluginArtifactRebuiltAtRef.current.delete(pluginId);
      } finally {
        pluginOpInFlightRef.current.delete(pluginId);
      }
    },
    [primaryPluginId, hostMode],
  );
  
  const [extensionLedger, setExtensionLedger] = useState<readonly ExtensionLedgerEntry[]>([]);
  const extensionLedgerRef = useRef(extensionLedger);
  extensionLedgerRef.current = extensionLedger;
  const extensionTargetById = useMemo(() => new Map(catalog.extensions.map((target) => [target.pluginId, target] as const)), [catalog]);
  const extensionIdSet = useMemo(() => new Set(catalog.extensions.map((target) => target.pluginId)), [catalog]);

  const dispatchSpaceExtensionOp = useCallback(async (action: string, args: Record<string, unknown>) => {
    const active = sessionRef.current;
    if (!active) return;
    const pluginEntry = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === active.pluginId);
    if (!pluginEntry) return;
    try {
      const viewState = panelViewContext({
        ...active.viewState,
        locale: uiLocaleRef.current,
        terminology: uiTerminologyRef.current,
        windowInstances: sessionWindowInstances(active.app, extraWindowInstancesRef.current).map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
        activeUtilityByWindowId: buildActiveUtilityByWindowId(core.activeUtilityByWindowIdRef.current),
      });
      const wire = encodeWindowActionInvocation({ ...active, viewState }, { controllerId: active.app.controllerId, action, args }, extraWindowInstancesRef.current);
      await pluginEntry.handle.handleAction(active.instanceId, wire, viewState);
    } catch (error) {
    }
  }, []);

  /** 🧩️ Installs an extension package from a URL via the extension-store HTTP endpoint when
   * available, loads its module into `loadedPlugins`, and records it on the durable ledger. */
  const installExtension = useCallback(
    async (sourceUri: string) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const response = await fetch(extensionStoreUrl("/install"), {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({ url: sourceUri }),
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
      } catch (error) {
        const guessedId = sourceUri.split("/").filter(Boolean).pop()?.replace(/\.sxt$/i, "") ?? "";
        if (!guessedId) return;
        extensionId = guessedId;
        try {
          moduleUrl = (await pluginSource.acquireModule(extensionId, undefined, { signal: extensionFetchAbortRef.current.signal })).moduleUrl;
        } catch (resolveError) {
          return;
        }
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById, pluginSource, extensionRoute],
  );

  /** 🧩️ Installs an extension package from a local `.sxt` / `.semio` file via the extension store. */
  const installExtensionFromFile = useCallback(
    async (file: File) => {
      let extensionId = "";
      let version = "0.0.0";
      let moduleUrl = "";
      let packageHash = "";
      try {
        const bytes = await file.arrayBuffer();
        const response = await fetch(extensionStoreUrl("/install"), {
          method: "POST",
          headers: { "content-type": "application/octet-stream" },
          body: bytes,
          signal: extensionFetchAbortRef.current.signal,
        });
        if (!response.ok) {
          const body = await response.text();
          throw new Error(`extension store install failed (${response.status}): ${body}`);
        }
        const result = (await response.json()) as { extensionId: string; version: string; moduleUrl: string; packageHash?: string };
        extensionId = result.extensionId;
        version = result.version;
        moduleUrl = result.moduleUrl;
        packageHash = result.packageHash ?? "";
      } catch (error) {
        return;
      }
      if (!extensionId || !moduleUrl) return;
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "installing" });
      const sourceUri = `file:${file.name}`;
      try {
        const handle = await loadPluginModuleResilient(extensionId, moduleUrl);
        if (!handle) {
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "failed" });
          return;
        }
        pluginModuleUrlByIdRef.current.set(extensionId, moduleUrl);
        dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle, manifest: handle.manifest } });
        dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "loaded" });
        dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId: extensionId, value: "loaded" });
        const extendsHost = extensionTargetById.get(extensionId)?.extends ?? "unscoped";
        const entry: ExtensionLedgerEntry = {
          extensionId,
          version: handle.manifest.version || version,
          sourceUri,
          packageHash,
          enabled: true,
          extendsHost,
        };
        setExtensionLedger((prev) => {
          const next = prev.filter((existing) => existing.extensionId !== extensionId);
          next.push(entry);
          return next;
        });
        void dispatchSpaceExtensionOp("installExtension", {
          extensionId: entry.extensionId,
          version: entry.version,
          sourceUri: entry.sourceUri,
          packageHash: entry.packageHash,
          enabled: entry.enabled,
        });
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionTargetById, extensionRoute],
  );

  /** 🧩️ Unloads an extension, drops it from the ledger, and best-effort asks the store / space
   * document to forget it. */
  const uninstallExtension = useCallback(
    async (extensionId: string) => {
      if (pluginOpInFlightRef.current.has(extensionId)) return;
      pluginOpInFlightRef.current.add(extensionId);
      try {
        const current = loadedPluginsRef.current.find((entry) => entry.handle.pluginId === extensionId);
        if (current) {
          await retireContributorInstances(contributorInstancesRef.current, (entry) => entry.pluginId === extensionId);
          dispatch({ type: "REMOVE_LOADED_PLUGIN", pluginId: extensionId });
          dispatch({ type: "SET_PLUGIN_STATUS", pluginId: extensionId, value: "available" });
          await current.handle.dispose();
          pluginModuleUrlByIdRef.current.delete(extensionId);
          pluginArtifactRebuiltAtRef.current.delete(extensionId);
        }
        setExtensionLedger((prev) => prev.filter((entry) => entry.extensionId !== extensionId));
        void dispatchSpaceExtensionOp("uninstallExtension", { extensionId });
        try {
          await fetch(extensionStoreUrl(`/install?extensionId=${encodeURIComponent(extensionId)}`), { method: "DELETE", signal: extensionFetchAbortRef.current.signal });
        } catch {
          /* store may not expose DELETE yet */
        }
      } finally {
        pluginOpInFlightRef.current.delete(extensionId);
      }
    },
    [dispatchSpaceExtensionOp, extensionRoute],
  );

  /** 🧩️ Toggles whether an installed extension's contributions are pushed to host plugins. */
  const setExtensionEnabled = useCallback(
    async (extensionId: string, enabled: boolean) => {
      setExtensionLedger((prev) => {
        const existing = prev.find((entry) => entry.extensionId === extensionId);
        if (existing) {
          return prev.map((entry) => (entry.extensionId === extensionId ? { ...entry, enabled } : entry));
        }
        const target = extensionTargetById.get(extensionId);
        return [
          ...prev,
          {
            extensionId,
            version: "0.0.0",
            sourceUri: "",
            packageHash: "",
            enabled,
            extendsHost: target?.extends ?? "unscoped",
          },
        ];
      });
      void dispatchSpaceExtensionOp("setExtensionEnabled", { extensionId, enabled });
    },
    [dispatchSpaceExtensionOp, extensionTargetById],
  );

  //#endregion 🔌️PluginRuntime

  return {
    primaryPluginId, shellPluginCanvasStatus, hubPluginSource, extensionRegistry, pluginSource,
    establishPrimaryWithShardRetry, installPlugin, installingPluginIds, pluginInstallProgress, cancelPluginInstalls,
    commandStalls, commandStallClockMs, installHubDocumentProgram, reloadPlugin, uninstallPlugin, extensionLedger,
    extensionLedgerRef, extensionIdSet, installExtension, installExtensionFromFile, uninstallExtension,
    setExtensionEnabled
  };
}
//#endregion FrameworkOsShell
