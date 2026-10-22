/** 🔶️ The document services phase of useSpawnedSession. */
import { useCallback, useEffect, useRef } from "../../../⚛️react-port/🟦️.ts";
import { panelTabKindId, pendingWindowUiNode, type BuiltNode, type UiDirtyScope, type WindowEngagement, type WindowMeasure } from "@semio-tech/framework";
import { focusedProgramKeyV1, programKeyedEntriesV1, withProgramEntriesV1, spawnedProgramViewStateV1, guestActiveUtilityByWindowIdV1, guestWindowIdV1, spawnedGuestWindowInstancesV1, spawnedWindowInstanceIdV1 } from "../../🪟️spawned-program/🟦️.ts";
import { type WindowLayoutNode } from "@semio-tech/ui-react";
import { sessionProgramsV1 } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import type { ViewModel } from "../../../🔌️PluginRuntime/🧷️handle-contract/🟦️.ts";
import { flattenPanelTabLeaves, resolveCanvasBodyKey, frameworkLayoutDeclaredInstances } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { applyUiRefreshResponseToCache, buildUiRefreshRequest, mergeRecordPreservingIdentity } from "../../../🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { type SpawnedAppEntry } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { isPluginInstanceRetiredV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { liveInstanceWindowFaultV1 } from "../../🩺️fault/🟦️.ts";
import { publishBuiltNodesV1, publishPanelBodiesV1, type SpawnedRefreshSlotV1 } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterContributionsAndRefresh } from "../../🔁️contributions-refresh/🟦️.ts";

/** 🔷 Builds documentServices, scope, hostMode, dispatch of useSpawnedSession. */
export function useDocumentServicesState({ core }: { readonly core: ShellCoreAfterContributionsAndRefresh }) {
  const {
    documentServices, scope, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology, applyHistoryPatch,
    leftoverReplaceRefreshBodiesV1, applyLeftoverInteractionView, hostApp, landingAppId, appLabelsOverlay, shellLayout,
    spawnedRefreshSlotsRef, extraWindowCounterRef, extraWindowInstancesRef, sessionWorkRef, sealedInstancesRef,
    sessionSwitchGateRef, setSurfaceSwitchBusy, spawnedUiRefreshCacheRef, spawnedLayoutSeedRef, rememberOpenSpaceId,
    openInstanceIdRef, sessionRef, shellSessionLaneRef, builtNodeStoreCacheRef, backboneWorkerRef, identity, identityRef,
    verifiedSessionAuthorityRef, showTransientNoticeRef, openDocumentRef, closeDocumentRef, documentAttachmentLanesRef,
    inferencePortEpochRef, inferencePortOwnerRef, inferencePortAuthorityRef, retiredSessionDocumentOwnersRef,
    inferencePortOpeningRef, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, loadedPluginsRef,
    pluginSupervisorByIdRef, reportRefreshFault, documentServiceInventoryRef, ensureBackboneWorker,
    establishPrimaryWithShardRetry, panel, spawnedIdsRef, focusedProgram, focusedApp, focusedSpawnedId,
    activeUtilityByWindowIdRef, activeToolIdRef, writeToolRegister, activeWindowIdRef, shellStateRef,
    injectActiveUtility, publishContributions, refreshUi
  } = core;
  const refreshSpawnedUiPass = useCallback(
    async (spawned: SpawnedAppEntry, viewState: ViewModel) => {
      const loadedPlugins = loadedPluginsRef.current;
      const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === spawned.pluginId);
      const plugin = pluginEntry?.handle;
      const app = pluginEntry?.manifest.apps.find((candidate) => candidate.id === spawned.appId);
      if (!plugin || !app) {
        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        const unavailableWindowId = spawnedWindowInstanceIdV1(spawned.id, "main");
        publishBuiltNodesV1(builtNodeStoreCacheRef.current, [[`spawned:${unavailableWindowId}`, { ...pendingWindowUiNode(), activity: "idle", component: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}`, emphasize: null, dataAttributes: null } } satisfies BuiltNode]]);
        dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { [unavailableWindowId]: "idle" } });
        dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
        dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
        return;
      }
      const spawnedSeed = `${spawned.pluginId}:${spawned.appId}:${spawned.instanceId}`;
      if (spawnedLayoutSeedRef.current !== spawnedSeed) {
        spawnedLayoutSeedRef.current = spawnedSeed;
        spawnedUiRefreshCacheRef.current = new Map();
      }
      const cache = spawnedUiRefreshCacheRef.current;
      const windowKinds = spawnedGuestWindowInstancesV1(app.windowKinds, frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds));
      const guestActiveWindowId = guestWindowIdV1(activeWindowIdRef.current, spawnedIdsRef.current) ?? resolveCanvasBodyKey(app);
      const fullViewState: ViewModel = injectActiveUtility(
        {
          ...spawnedProgramViewStateV1(viewState, app),
          locale: uiLocale,
          terminology: uiTerminology,
          windowId: guestActiveWindowId,
          windowInstances: windowKinds.map((instance) => ({ id: instance.id, windowKindId: instance.windowKindId })),
          activeUtilityByWindowId: guestActiveUtilityByWindowIdV1(activeUtilityByWindowIdRef.current, spawned.id, spawnedIdsRef.current),
        },
        spawnedWindowInstanceIdV1(spawned.id, guestActiveWindowId),
      );
      const panelTabLeaves = flattenPanelTabLeaves(app.panelTabs);
      const contributions = await publishContributions({ pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(viewState, app) }, sessionProgramsV1(loadedPlugins, spawned.pluginId), true);
      if (contributions.status === "failed") console.error(`[os-shell] contributions push into spawned "${spawned.pluginId}" #${spawned.instanceId} failed: ${contributions.reason}`);
      if (spawnedLayoutSeedRef.current !== spawnedSeed) return;
      const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, fullViewState, cache);
      if (request) {
        const response = await plugin.refreshUi(spawned.instanceId, request);
        if (spawnedLayoutSeedRef.current !== spawnedSeed) return;
        applyUiRefreshResponseToCache(cache, response);
      }
      const uiByWindowId = Object.fromEntries(windowKinds.map((kind) => [spawnedWindowInstanceIdV1(spawned.id, kind.id), (cache.get(`window:${kind.id}`)?.value as BuiltNode | undefined) ?? pendingWindowUiNode()] as const));
      const spawnedKeyed = <T,>(byGuestWindowId: Readonly<Record<string, T>>): Readonly<Record<string, T>> =>
        Object.fromEntries(Object.entries(byGuestWindowId).map(([windowId, value]) => [spawnedWindowInstanceIdV1(spawned.id, windowId), value] as const));
      const dynamicEngagements = spawnedKeyed((cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {});
      const dynamicMeasures = spawnedKeyed((cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {});
      publishBuiltNodesV1(builtNodeStoreCacheRef.current, Object.entries(uiByWindowId).map(([windowId, node]) => [`spawned:${windowId}`, node] as const));
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(uiByWindowId).map(([windowId, node]) => [windowId, node.activity] as const)) });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)) });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)) });
      const panelBodies = programKeyedEntriesV1(
        spawned.id,
        Object.fromEntries(panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined] as const)),
      );
      const panelStores = publishPanelBodiesV1(builtNodeStoreCacheRef.current, Object.entries(panelBodies));
      dispatch({ type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, panelStores)) });
      const toolMeasures = programKeyedEntriesV1(spawned.id, (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {});
      dispatch({ type: "SET_TOOL_MEASURES_BY_TOOL_ID", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, Object.entries(toolMeasures))) });
    },
    [injectActiveUtility, publishContributions, uiLocale, uiTerminology],
  );
  const refreshSpawnedUiPassRef = useRef(refreshSpawnedUiPass);
  refreshSpawnedUiPassRef.current = refreshSpawnedUiPass;
  const refreshSpawnedUi = useCallback(
    (spawned: SpawnedAppEntry, viewState: ViewModel, scopeArg: UiDirtyScope = { kind: "full" }): Promise<void> => {
      if (scopeArg.kind === "none") return Promise.resolve();
      const slots = spawnedRefreshSlotsRef.current;
      const running = slots.get(spawned.id);
      if (running) {
        running.viewState = viewState;
        running.again = true;
        return running.done;
      }
      const slot: SpawnedRefreshSlotV1 = { viewState, again: false, done: Promise.resolve() };
      slot.done = (async () => {
        try {
          do {
            slot.again = false;
            await refreshSpawnedUiPassRef.current(spawned, slot.viewState);
          } while (slot.again);
        } finally {
          slots.delete(spawned.id);
        }
      })();
      slots.set(spawned.id, slot);
      return slot.done;
    },
    [],
  );

  const sessionTripleKey = session ? `${session.pluginId}:${session.app.id}:${session.instanceId}` : null;
  const sessionHumanKey = `${identity?.userId ?? ""}:${identity?.displayName ?? ""}`;
  useEffect(() => {
    const current = sessionRef.current;
    if (!current) return;
    void refreshUi(current).catch((renderError) => reportRefreshFault(current.pluginId, renderError));
  }, [refreshUi, reportRefreshFault, sessionTripleKey]);

  useEffect(() => {
    if (!hostMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    const activeSpawned = panel?.spawnedApps.find((entry) => entry.id === panel.activeSpawnedId);
    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_ENGAGEMENTS", value: {} });
      dispatch({ type: "SET_SPAWNED_WINDOW_MEASURES", value: {} });
      return;
    }
    void refreshSpawnedUi(activeSpawned, session.viewState).catch((renderError) => {
      const fault = liveInstanceWindowFaultV1(renderError, pluginSupervisorByIdRef.current[activeSpawned.pluginId], isPluginInstanceRetiredV1);
      if (fault !== null) dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {}, fault });
    });
  }, [panel, refreshSpawnedUi, refreshSpawnedUiPass, session, hostMode]);

  /** 🪟️ The session layout the shell was showing when a spawned program took the canvas — restored
   * verbatim when focus comes back, so opening a foreign editor and closing it again does not cost
   * the user the arrangement they built. */
  const sessionLayoutBeforeSpawnRef = useRef<{ readonly layout: WindowLayoutNode | null; readonly activeWindowId: string | null } | null>(null);
  const focusedProgramKey = focusedProgramKeyV1(focusedProgram);
  const seededFocusedProgramKeyRef = useRef<string>("");
  return { session, seededFocusedProgramKeyRef, focusedProgramKey, activeToolIdRef, writeToolRegister, focusedSpawnedId, focusedApp, sessionLayoutBeforeSpawnRef, shellLayout, activeWindowIdRef, appLabelsOverlay, uiTerminology, uiLocale, dispatch, scope, loadedPluginsRef, openDocumentSessionsRef, closeDocumentRef, documentAttachmentLanesRef, identity, sessionRef, shellSessionLaneRef, refreshUi, reportRefreshFault, establishPrimaryWithShardRetry, pluginSupervisorByIdRef, sessionHumanKey, hostMode, hostApp, setSurfaceSwitchBusy, sessionSwitchGateRef, loadedPlugins, sessionWorkRef, sealedInstancesRef, openDocumentRef, extraWindowInstancesRef, extraWindowCounterRef, landingAppId, rememberOpenSpaceId, openInstanceIdRef, showTransientNoticeRef, documentServices, documentServiceInventoryRef, backboneWorkerRef, inferencePortOwnerRef, inferencePortOpeningRef, verifiedSessionAuthorityRef, identityRef, retiredSessionDocumentOwnersRef, inferencePortEpochRef, ensureBackboneWorker, inferencePortAuthorityRef, browserActorUiByRuntimeKeyRef, shellStateRef, applyHistoryPatch, applyLeftoverInteractionView, leftoverReplaceRefreshBodiesV1, refreshSpawnedUi };
}
