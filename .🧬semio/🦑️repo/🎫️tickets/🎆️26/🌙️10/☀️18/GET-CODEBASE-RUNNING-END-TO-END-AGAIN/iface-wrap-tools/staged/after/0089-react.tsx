/** 🟠️ The workbench left tabs phase of useDocumentTransferAndTabs. */
import { useCallback, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { type AppRole, FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_ID } from "@semio-tech/framework";
import { type PanelTabNode, singleTreeLeaf, staticTreePanelDefinition } from "@semio-tech/ui-react";
import { type OpenWithEntry } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { openArtifactWithText, setAsDefaultText, shellLabel, shellTabIcon, surfaceRoleChipText } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { panelAnchorForGroup, panelTabDefinitionToNode, integrateAppSettingsPanelTabsIntoFrameworkBranch, partitionFrameworkHistoryPanelTab } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { flattenPanelTabNodes } from "../../🧱️built-node-stores/🟦️.ts";
import type { buildScopeState } from "../🔶️scope/🟦️.tsx";
import type { useDocumentTransferRefState } from "../🔸️document-transfer-ref/🟦️.tsx";

/** 🟡 Builds workbenchLeftTabs, detailsRightTabs, appTabsForBottomAnchor, displayBottomLeftTabs of useDocumentTransferAndTabs. */
export function useWorkbenchLeftTabsState({ session, focusedApp, focusedPanelBodyStores, onAction, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef, focusedBrowserActorPanels, hostMode, hostAppId, openWithFocusRole, openArtifactWithAppRef, dispatchClearDefaultApp, dispatchSetDefaultApp, panel, openWithEntries, treeWindowGeneration, frameworkSettingsTab }: Pick<ReturnType<typeof buildScopeState>, "session" | "focusedApp" | "focusedPanelBodyStores" | "onAction" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "treeWindowHost" | "panelTreeConfigCacheRef" | "hostMode" | "hostAppId" | "openWithFocusRole" | "openArtifactWithAppRef" | "dispatchClearDefaultApp" | "dispatchSetDefaultApp" | "panel" | "openWithEntries" | "treeWindowGeneration" | "frameworkSettingsTab"> & Pick<ReturnType<typeof useDocumentTransferRefState>, "focusedBrowserActorPanels">) {
  const workbenchLeftTabs = useMemo((): PanelTabNode[] => {
    if (!session || !focusedApp) return [];
    const pluginLeftTabs = focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-left").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels));
    if (hostMode && focusedApp.id === hostAppId && pluginLeftTabs.length > 0) return pluginLeftTabs;
    const hasPluginArtifactTab = flattenPanelTabNodes(pluginLeftTabs).some((tab) => tab.id === FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
    if (hasPluginArtifactTab) return pluginLeftTabs;
    const openWithSection = (role: AppRole, entries: readonly OpenWithEntry[]) => ({
      id: `artifact.openWith.${role}`,
      label: `${openArtifactWithText(uiLocale)} — ${surfaceRoleChipText(role, uiLocale)}`,
      defaultOpen: openWithFocusRole == null || openWithFocusRole === role,
      items: entries.map((entry) => ({
        id: `artifact.openWith.${role}.${entry.app.pluginId}.${entry.app.appId}`,
        label: entry.current ? `${entry.pluginLabel} ✓` : entry.pluginLabel,
        onClick: entry.current ? undefined : () => void openArtifactWithAppRef(entry.app, session.app.dialect, role),
        control: (
          <button
            type="button"
            aria-pressed={entry.isDefault}
            onClick={(event) => {
              event.stopPropagation();
              if (entry.isDefault) dispatchClearDefaultApp(session.app.dialect, role);
              else dispatchSetDefaultApp(session.app.dialect, role, entry.app);
            }}
          >
            {entry.isDefault ? "★" : "☆"} {setAsDefaultText(uiLocale)}
          </button>
        ),
      })),
    });
    const artifactTab = singleTreeLeaf({
      id: FRAMEWORK_PANEL_TAB_ARTIFACT_ID,
      icon: shellTabIcon(FRAMEWORK_PANEL_TAB_ARTIFACT_ICON_ID),
      name: shellLabel("ui.panel.artifact"),
      order: 0,
      tree: staticTreePanelDefinition({
        sections: [
          {
            id: "artifact.root",
            label: shellLabel("ui.panel.artifact"),
            items: [{ id: "artifact.empty", label: hostMode ? `${panel?.spawnedApps.length ?? 0} ${shellLabel("ui.panel.spawnedAppsSuffix")}` : shellLabel("ui.panel.artifactEmpty") }],
          },
          ...(openWithEntries && openWithEntries.viewer.length > 0 ? [openWithSection("viewer", openWithEntries.viewer)] : []),
          ...(openWithEntries && openWithEntries.editor.length > 0 ? [openWithSection("editor", openWithEntries.editor)] : []),
        ],
      }),
    });
    return [artifactTab, ...pluginLeftTabs];
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, panel?.spawnedApps.length, focusedPanelBodyStores, session, focusedApp, hostMode, uiLocale, uiTerminology, hostAppId, openWithEntries, openWithFocusRole, openArtifactWithAppRef, dispatchSetDefaultApp, dispatchClearDefaultApp, treeWindowHost, treeWindowGeneration]);

  const detailsRightTabs = useMemo((): PanelTabNode[] => {
    if (!session || !focusedApp) return [];
    return focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "top-right").map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels));
  }, [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]);

  /**
   * 🧭️ The two bottom anchors' app-declared tabs — `PanelGroup::Display` → `bottom-left`,
   * `PanelGroup::Settings` → `bottom-right`. Without these an app could declare either group and the
   * dock assembly dropped the tab silently: the guest still rendered the body (every declared tab is
   * flattened into `buildUiRefreshRequest`) and the result was cached as a panel body and then never
   * mounted, which is exactly how puzzle3d's own Settings section — grid spacing, chunk size,
   * proximity radius, overlap budget — was unreachable while the framework's own `framework.settings`
   * branch beside it opened fine.
   *
   * 🕰️ {@link partitionFrameworkHistoryPanelTab} keeps the framework-injected `framework.panel.history` out: the
   * dock mounts it as its own bottom-right leaf ({@link frameworkHistoryTab}) rather than inside the Settings branch.
   */
  const appTabsForBottomAnchor = useCallback(
    (anchor: ReturnType<typeof panelAnchorForGroup>): PanelTabNode[] =>
      session && focusedApp
        ? partitionFrameworkHistoryPanelTab(focusedApp.panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === anchor)).rest
            .map((tab, order) => panelTabDefinitionToNode(tab, tab.group, focusedPanelBodyStores, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, focusedBrowserActorPanels))
        : [],
    [appLabelsOverlay, focusedBrowserActorPanels, onAction, focusedPanelBodyStores, session, focusedApp, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration],
  );
  const displayBottomLeftTabs = useMemo(() => appTabsForBottomAnchor("bottom-left"), [appTabsForBottomAnchor]);
  const settingsBottomRightTabs = useMemo(() => appTabsForBottomAnchor("bottom-right"), [appTabsForBottomAnchor]);
  const settingsBottomRightDockTab = useMemo(
    () => integrateAppSettingsPanelTabsIntoFrameworkBranch(frameworkSettingsTab, settingsBottomRightTabs),
    [frameworkSettingsTab, settingsBottomRightTabs],
  );
  return { workbenchLeftTabs, detailsRightTabs, displayBottomLeftTabs, settingsBottomRightDockTab };
}
