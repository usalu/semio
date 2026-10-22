/** 🟠️ The footer items phase of useCanvasAndFooter. */
import { shellFooterNavbarItem } from "../../../🏛️ShellFooter/🟦️.tsx";
import { useCallback, useEffect, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { FRAMEWORK_PANEL_TAB_ARTIFACT_ID } from "@semio-tech/framework";
import { documentRuntimeKeyV1 } from "@semio-tech/framework/backbone";
import { type Anchor, navbarFillItem, type NavbarItem, PanelChromeTabBar, PresenceBar } from "@semio-tech/ui-react";
import { PANEL_TAB_BAR_HOSTS } from "../../../🛠️ShellHelpers/🔩️plumbing/🟦️.ts";
import { toolPanelTreeContentRevision } from "../../../🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { HubConnectionIndicator } from "../../../🔄️ShellSync/🟦️.tsx";
import { AgentApprovalsNotice } from "../../../🤖️AgentApprovals/🟦️.tsx";
import { AgentPresence } from "../../../🚦️AgentPresence/🟦️.tsx";
import { type ShellCatalogProbe, shellCatalogProbePlugins } from "../../🎚️shell-model/🟦️.tsx";
import type { buildPluginFilterState } from "../🔶️plugin-filter/🟦️.tsx";

/** 🟡 Builds footerItems, buildPanelProps, dispatchShellMenuAction of useCanvasAndFooter. */
export function useFooterItemsState({ uiDevice, mobile, buildPanelSelectionProps, brand, uiLocale, presencePeers, timeTravelPeers, agentBridge, revealAgentApproval, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, panels, dispatch, treeOpenStates, activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey, scope, pluginFilter, hostMode, notFoundPath, error, session, initialExampleReady, registry, pluginStatusById, appRouter, spacePrograms, panel, contributionsPublisherRef, documentServices, browserActorUiByRuntimeKeyRef, routerFaultByPluginId, activeWindowId, onAction }: Pick<ReturnType<typeof buildPluginFilterState>, "uiDevice" | "mobile" | "buildPanelSelectionProps" | "brand" | "uiLocale" | "presencePeers" | "timeTravelPeers" | "agentBridge" | "revealAgentApproval" | "hubConnectionStatuses" | "hubSessionPresence" | "hubLink" | "openHubWorkspace" | "panels" | "dispatch" | "treeOpenStates" | "activeToolId" | "focusedToolMeasuresByToolId" | "actionPaneStagedArgsByKey" | "scope" | "pluginFilter" | "hostMode" | "notFoundPath" | "error" | "session" | "initialExampleReady" | "registry" | "pluginStatusById" | "appRouter" | "spacePrograms" | "panel" | "contributionsPublisherRef" | "documentServices" | "browserActorUiByRuntimeKeyRef" | "routerFaultByPluginId" | "activeWindowId" | "onAction">) {
  const footerItems = useMemo((): NavbarItem[] => {
    const compactFooter = uiDevice === "tablet";
    const items: NavbarItem[] = mobile
      ? []
      : [
          { key: "bottomLeftPanelTabs", content: <PanelChromeTabBar anchor="bottom-left" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-left")} /> },
          { key: "bottomMiddlePanelTabs", centered: true, content: <PanelChromeTabBar anchor="bottom-middle" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-middle")} /> },
        ];
    const credits = brand?.footerItems ?? [];
    if (credits.some((item) => item.placement === "leading")) items.push({ key: "footerLeadingGap", className: "w-huge", content: null });
    for (const item of credits.filter((item) => item.placement === "leading")) items.push(shellFooterNavbarItem(item, uiLocale, mobile));
    items.push(navbarFillItem("footerLeadingFill"));
    for (const item of credits.filter((item) => item.placement === "trailing")) items.push(shellFooterNavbarItem(item, uiLocale, mobile));
    if (credits.some((item) => item.placement === "trailing")) items.push({ key: "footerTrailingGap", className: "w-huge", content: null });
    if (!mobile && !(compactFooter && presencePeers.length === 0)) items.push({ key: "presenceBar", content: <PresenceBar id="s-presence-peers" peers={timeTravelPeers.peers} /> });
    if (agentBridge.pendingApprovals.length > 0) items.push({ key: "agentApprovals", content: <AgentApprovalsNotice approvals={agentBridge.pendingApprovals} onReview={revealAgentApproval} /> });
    if (agentBridge.status === "unavailable" || agentBridge.status === "incompatible") items.push({ key: "agentBridgeNotice", content: <AgentPresence status={agentBridge.status} presence={agentBridge.presence} versionMismatch={agentBridge.versionMismatch} /> });
    items.push({ key: "hubConnection", content: <HubConnectionIndicator statuses={hubConnectionStatuses} session={hubSessionPresence} link={hubLink} onSignIn={openHubWorkspace} /> });
    if (!mobile) items.push({ key: "bottomRightPanelTabs", content: <PanelChromeTabBar anchor="bottom-right" compactLabels={compactFooter} {...buildPanelSelectionProps("bottom-right")} /> });
    return items;
  }, [agentBridge.pendingApprovals, agentBridge.presence, agentBridge.status, agentBridge.versionMismatch, brand?.footerItems, buildPanelSelectionProps, hubConnectionStatuses, hubLink, hubSessionPresence, mobile, openHubWorkspace, presencePeers, revealAgentApproval, timeTravelPeers.peers, uiDevice, uiLocale]);

  const buildPanelProps = useCallback(
    (anchor: Anchor) => ({
      ...buildPanelSelectionProps(anchor),
      size: panels[anchor].size,
      onSizeChange: (value: number) => dispatch({ type: "SET_PANEL_SIZE", anchor, value }),
      tabBarHost: (PANEL_TAB_BAR_HOSTS[anchor] ? "chrome" : "panel") as "panel" | "chrome",
      treeOpenStates,
      onTreeOpenStateChange: (id: string, open: boolean) => dispatch({ type: "SET_TREE_OPEN_STATE", id, open }),
      treeContentRevision: toolPanelTreeContentRevision(activeToolId, focusedToolMeasuresByToolId, actionPaneStagedArgsByKey),
    }),
    [actionPaneStagedArgsByKey, activeToolId, buildPanelSelectionProps, panels, focusedToolMeasuresByToolId, treeOpenStates],
  );

  // #region 🔖️ReadinessBeacon
  useEffect(() => {
    const root = document.documentElement;
    const shellRoot = scope.rootRef.current;
    const beaconId = pluginFilter ?? "unknown";
    const notFound = hostMode && notFoundPath !== null;
    if (notFound) {
      root.dataset.semioOsNotFound = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      if (shellRoot) {
        shellRoot.dataset.shellNotFound = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
      }
    } else if (error) {
      root.dataset.semioOsError = beaconId;
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellError = beaconId;
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellNotFound;
      }
    } else if ((session && initialExampleReady) || (!pluginFilter && registry.length === 0)) {
      root.dataset.semioOsReady = beaconId;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        shellRoot.dataset.shellReady = beaconId;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    }
    return () => {
      delete root.dataset.semioOsReady;
      delete root.dataset.semioOsError;
      delete root.dataset.semioOsNotFound;
      if (shellRoot) {
        delete shellRoot.dataset.shellReady;
        delete shellRoot.dataset.shellError;
        delete shellRoot.dataset.shellNotFound;
      }
    };
  }, [session, initialExampleReady, error, pluginFilter, registry.length, notFoundPath, hostMode, scope.rootRef]);
  // #endregion 🔖️ReadinessBeacon

  // #region 🔖️CatalogSmokeProbe
  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const host = window as unknown as { __semioOsCatalogProbe?: ShellCatalogProbe };
    host.__semioOsCatalogProbe = {
      shellPluginId: pluginFilter ?? "unknown",
      ready: !!session && !error,
      plugins: shellCatalogProbePlugins(registry, pluginStatusById, appRouter),
      programs: spacePrograms.map((program) => ({ pluginId: program.pluginId, appId: program.appId, label: program.label })),
      spawned: (panel?.spawnedApps ?? []).map((entry) => ({ id: entry.id, pluginId: entry.pluginId, appId: entry.appId })),
    };
    const installed = window as unknown as { __semioOsInstalledContributions?: (instanceId: number) => string | null };
    installed.__semioOsInstalledContributions = (instanceId) => contributionsPublisherRef.current?.installedFor(instanceId) ?? null;
    return () => {
      delete host.__semioOsCatalogProbe;
      delete installed.__semioOsInstalledContributions;
    };
  }, [session, error, pluginFilter, registry, pluginStatusById, panel, spacePrograms, appRouter]);

  useEffect(() => {
    let dev = false;
    try {
      dev = Boolean((import.meta as unknown as { readonly env?: { readonly DEV?: boolean } }).env?.DEV);
    } catch {
      dev = false;
    }
    if (!dev || typeof window === "undefined") return;
    const read = (owner: string, serviceId: string, spaceId: string, documentId: string): unknown => {
      const presentation = documentServices.find((entry) => entry.owner === owner && entry.serviceId === serviceId);
      if (!presentation?.probe || typeof spaceId !== "string" || typeof documentId !== "string" || spaceId.length === 0 || documentId.length === 0) return null;
      const retained = browserActorUiByRuntimeKeyRef.current.get(documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId, documentId }));
      if (retained?.identity === null || retained?.identity === undefined) return null;
      const store = retained.windows.get(retained.identity.windowKindId);
      return store === undefined ? null : presentation.probe({ ...retained.identity, sessionInstanceId: retained.sessionInstanceId, store });
    };
    const host = window as unknown as { __semioMountedServiceProbe?: typeof read };
    host.__semioMountedServiceProbe = read;
    return () => { if (host.__semioMountedServiceProbe === read) delete host.__semioMountedServiceProbe; };
  }, [documentServices]);

  useEffect(() => {
    for (const fault of routerFaultByPluginId.values()) console.error(`AppRouter excluded plugin ${JSON.stringify(fault.scope.pluginId ?? "")}: ${fault.code}: ${fault.message}`);
  }, [routerFaultByPluginId]);
  // #endregion 🔖️CatalogSmokeProbe

  //#region 🖱️ShellContextMenu
  /** 🖱️ Dispatch sink for the shell fallback menu's `ContextMenuItemSpec`s (see
   * `buildShellContextMenuItems`) — intercepts the two reserved ids the builder emits in place of a
   * real dispatch (`"shell.openActionPane"`/`"shell.openPalette"`) and forwards everything else to
   * `onAction`, mirroring the command palette's own arg-carrying redirect. */
  const dispatchShellMenuAction = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      if (!session) return;
      if (action === "shell.openActionPane") {
        const windowKind = session.app.windowKinds.find((kind) => kind.id === activeWindowId) ?? session.app.windowKinds[0];
        const actionId = typeof args?.actionId === "string" ? args.actionId : undefined;
        if (!windowKind || !actionId) return;
        dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: windowKind.id });
        dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: windowKind.id, value: false });
        dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: windowKind.id, value: actionId });
        return;
      }
      if (action === "shell.openPalette") {
        dispatch({ type: "SET_SEARCH_OPEN", value: true });
        return;
      }
      if (action === "shell.openArtifactWith") {
        dispatch({ type: "SET_OPEN_WITH_FOCUS_ROLE", value: null });
        dispatch({ type: "SET_PANEL_PATH", anchor: "top-left", value: [FRAMEWORK_PANEL_TAB_ARTIFACT_ID] });
        dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
        return;
      }
      onAction({ controllerId: session.app.controllerId, action });
    },
    [session, activeWindowId, onAction, dispatch],
  );
  return { dispatchShellMenuAction, footerItems, buildPanelProps };
}
