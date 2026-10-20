/** 🛍️ The settings host, the marketplace host and the task manager tab of the theme driver settings hook. */
import { useEffect, useMemo, useRef, useState } from "../../../⚛️react-port/🟦️.ts";
import { type VideoRenderJobRow } from "@semio-tech/framework";
import { setAppearance, setLayout, setLocale, setTerminology } from "@semio-tech/framework/config/mutations";
import { type ElementsSurfaceAppearance, UI_TERMINOLOGY_NATIVE, type UiChromeLayout, type UiLocale } from "@semio-tech/ui-react";
import { appBreadcrumb, resolveAppBreadcrumb } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { actionBindingToActionDescriptor } from "../../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { toolRunPanelTasksV1, type ToolRunPanelControlV1 } from "../../../🛠️ShellHelpers/⏯️tool-run-panel/🟦️.ts";
import { createFrameworkChatPanelTab, createFrameworkTaskManagerPanelTab, createFrameworkSettingsPanelTab } from "../../../📌️ChromePanels/🟦️.tsx";
import { createFrameworkDisplayPanelTabs } from "../../../📌️ChromePanels/🖥️display-panel/🟦️.tsx";
import { type SettingsHostApi } from "../../../📌️ChromePanels/⚙️settings-general/🟦️.tsx";
import { createFrameworkMarketplacePanelTab, type MarketplaceExtensionEntry, type MarketplaceHostApi, type MarketplacePluginEntry } from "../../../📌️ChromePanels/🏪️marketplace-panel/🟦️.tsx";
import { hubCatalogOnlyPluginsV1 } from "@semio-tech/framework/plugin-module/hub-source/resolution";
import type { TrustedPluginModuleIndexV1 } from "@semio-tech/framework/plugin-module";
import { AgentChatPanel } from "../../../💬️AgentChatPanel/🟦️.tsx";
import { TaskManagerWindow, installTasksV1, spawnedJobTasksV1, taskManagerToolRunIdV1, toolCallTasksV1, toolRunTasksV1, videoRenderExportTasksV1, type TaskManagerDocumentTransferV1, type TaskManagerSourcesV1, type TaskManagerTaskV1 } from "../../../🧵️TaskManager/🟦️.tsx";
import { cancelVideoRenderExportV1, subscribeVideoRenderExportJobsV1, videoRenderExportJobsSnapshotV1 } from "../../../🎥️VideoRenderHost/🟦️.ts";
import { cancelSpawnedJobV1, pluginRuntimeActivationRegistryV1, spawnedJobsSnapshotV1, subscribeSpawnedJobsV1, type SpawnedJobRowV1 } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { HUB_CATALOG_REFRESH_MS } from "../../🔌️extension-runtime/🟦️.ts";
import type { useThemeAndDriverState, useThemeSaveAndSettingsHostRef } from "../🎨️theme-and-driver-state/🟦️.tsx";

/** ⚙ Builds the settings host api and the marketplace host ref. */
export function useSettingsHost({ session, uiTerminology, uiDriverId, uiDriverBase, uiDriverDirty, uiDriverList, setDriverId, setDriverField, saveDriver, deleteDriver, driverSaveLabel, setDriverSaveLabel, uiAppearance, commitUiPreference, noteOsCommand, uiLayout, mobile, dispatch, dockLayoutStore, dockUiStateStore, uiLocale, uiThemeBase, uiThemeId, uiThemeDirty, uiThemeList, setThemeId, setThemeColor, setThemeSpacing, setThemeFontStack, setThemeStroke, setThemeRadius, setThemeOpacity, setThemeMetric, setThemeAppearancePaint, saveTheme, deleteTheme, resetTheme, exportTheme, importTheme, themeSaveLabel, setThemeSaveLabel, controlKeybindings, controlKeybindingLabel, keybindingCaptureControlId, setKeybindingCaptureControlId, setKeybindingOverride, resetKeybindingOverride, locks, mergePolicy, dispatchSetMergePolicy, settingsHostRef, displayHostRef, displayHost, defaultAppsHostRef, conflictsHostRef, defaultAppsHost, conflictsHost, hubPluginSource, registry, extensionRegistry }: Pick<ReturnType<typeof useThemeAndDriverState>, "session" | "uiTerminology" | "uiDriverId" | "uiDriverList" | "uiAppearance" | "commitUiPreference" | "noteOsCommand" | "uiLayout" | "mobile" | "dispatch" | "dockLayoutStore" | "dockUiStateStore" | "uiLocale" | "uiThemeBase" | "uiThemeId" | "uiThemeDirty" | "uiThemeList" | "setThemeId" | "setThemeColor" | "setThemeSpacing" | "setThemeFontStack" | "setThemeStroke" | "setThemeRadius" | "setThemeOpacity" | "setThemeMetric" | "setThemeAppearancePaint" | "resetTheme" | "controlKeybindings" | "controlKeybindingLabel" | "locks" | "mergePolicy" | "dispatchSetMergePolicy" | "displayHostRef" | "displayHost" | "defaultAppsHostRef" | "conflictsHostRef" | "defaultAppsHost" | "conflictsHost" | "hubPluginSource" | "registry" | "extensionRegistry"> & Pick<ReturnType<typeof useThemeSaveAndSettingsHostRef>, "uiDriverBase" | "uiDriverDirty" | "setDriverId" | "setDriverField" | "saveDriver" | "deleteDriver" | "driverSaveLabel" | "setDriverSaveLabel" | "saveTheme" | "deleteTheme" | "exportTheme" | "importTheme" | "themeSaveLabel" | "setThemeSaveLabel" | "keybindingCaptureControlId" | "setKeybindingCaptureControlId" | "setKeybindingOverride" | "resetKeybindingOverride" | "settingsHostRef">) {
  const settingsHost: SettingsHostApi = useMemo(
    () => ({
      appId: session?.app.id,
      appLabel: session ? appBreadcrumb(resolveAppBreadcrumb(session.app, uiTerminology)) : undefined,
      controllerId: session?.app.controllerId,
      pluginId: session?.pluginId,
      driverId: uiDriverId,
      driver: uiDriverBase,
      driverDirty: uiDriverDirty,
      drivers: uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      appearance: uiAppearance,
      setAppearance: (value: string) => {
        commitUiPreference(setAppearance(value as ElementsSurfaceAppearance));
        noteOsCommand("os.setAppearance", { appearance: value });
      },
      layout: uiLayout,
      setLayout: (value: UiChromeLayout) => {
        commitUiPreference(setLayout(value));
        noteOsCommand("os.setLayout", { layout: value });
      },
      mobileActive: mobile,
      onResetDock: () => {
        dispatch({ type: "RESET_DOCK" });
        dockLayoutStore.reset();
        dockUiStateStore.reset();
        noteOsCommand("os.resetDock");
      },
      locale: uiLocale,
      setLocale: (value: UiLocale) => {
        commitUiPreference(setLocale(value));
        noteOsCommand("os.setLocale", { locale: value });
      },
      terminology: uiTerminology,
      setTerminology: (value: string) => {
        commitUiPreference(setTerminology(value));
        noteOsCommand("os.setTerminology", { terminology: value });
      },
      terminologies: [UI_TERMINOLOGY_NATIVE, ...(session?.app.terminologies ?? [])],
      theme: uiThemeBase,
      themeId: uiThemeId,
      themeDirty: uiThemeDirty,
      themes: uiThemeList,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      controlKeybindings,
      controlKeybindingLabel,
      keybindingCaptureControlId,
      setKeybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      locks,
      mergePolicy,
      setMergePolicy: dispatchSetMergePolicy,
    }),
    [
      session,
      dockLayoutStore,
      uiDriverId,
      uiDriverBase,
      uiDriverDirty,
      uiDriverList,
      setDriverId,
      setDriverField,
      saveDriver,
      deleteDriver,
      driverSaveLabel,
      setDriverSaveLabel,
      controlKeybindings,
      controlKeybindingLabel,
      keybindingCaptureControlId,
      setKeybindingOverride,
      resetKeybindingOverride,
      uiAppearance,
      uiLayout,
      mobile,
      uiLocale,
      uiTerminology,
      uiThemeBase,
      uiThemeId,
      uiThemeDirty,
      uiThemeList,
      locks,
      setThemeId,
      setThemeColor,
      setThemeSpacing,
      setThemeFontStack,
      setThemeStroke,
      setThemeRadius,
      setThemeOpacity,
      setThemeMetric,
      setThemeAppearancePaint,
      saveTheme,
      deleteTheme,
      resetTheme,
      exportTheme,
      importTheme,
      themeSaveLabel,
      setThemeSaveLabel,
      noteOsCommand,
      commitUiPreference,
      mergePolicy,
      dispatchSetMergePolicy,
    ],
  );
  settingsHostRef.current = settingsHost;

  const frameworkDisplayTabs = useMemo(() => createFrameworkDisplayPanelTabs(() => displayHostRef.current), [displayHost, uiLocale]);
  const frameworkSettingsTab = useMemo(
    () => createFrameworkSettingsPanelTab(() => settingsHostRef.current, () => defaultAppsHostRef.current, () => conflictsHostRef.current),
    [settingsHost, defaultAppsHost, conflictsHost],
  );

  /** 🌎️ The serving hub's current catalog of plugin modules, read when the shell starts and once a minute after (a
   * catalog generation changes rarely and publishes no stream); `null` without a hub or before its first answer. A read
   * that fails keeps the last catalog: one unanswered read is not a hub without plugins. */
  const [hubCatalog, setHubCatalog] = useState<TrustedPluginModuleIndexV1 | null>(null);
  useEffect(() => {
    if (hubPluginSource === null) {
      setHubCatalog(null);
      return;
    }
    const abort = new AbortController();
    const read = (): void => {
      void hubPluginSource.catalog(abort.signal).then((index) => {
        if (!abort.signal.aborted && index !== null) setHubCatalog((current) => (current?.generationId === index.generationId ? current : index));
      }, () => undefined);
    };
    read();
    const timer = setInterval(read, HUB_CATALOG_REFRESH_MS);
    return () => {
      abort.abort();
      clearInterval(timer);
    };
  }, [hubPluginSource]);
  /** 🧩️ The shell's plugin registry is this device's build plus every plugin the serving hub's catalog carries that the build
   * lacks ({@link hubCatalogOnlyPluginsV1}): a hub-only plugin is listed, installs from the hub (verified, into the store) through
   * the same `installPlugin`, and then runs like any other device program. */
  const hubOnlyPlugins = useMemo(() => (hubCatalog === null ? [] : hubCatalogOnlyPluginsV1(hubCatalog, new Set([...registry, ...extensionRegistry].map((entry) => entry.pluginId)))), [hubCatalog, registry, extensionRegistry]);
  const marketplaceHostRef = useRef<MarketplaceHostApi | null>(null);
  return { hubOnlyPlugins, marketplaceHostRef, frameworkDisplayTabs, frameworkSettingsTab };
}

/** 🏪 Builds the marketplace host api and the framework task manager tab. */
export function useMarketplaceHostAndTaskManager({ registry, extensionIdSet, pluginSource, hubOnlyPlugins, loadedPlugins, pluginStatusById, primaryPluginId, session, catalog, extensionLedger, installPlugin, uninstallPlugin, reloadPlugin, installExtension, installExtensionFromFile, uninstallExtension, setExtensionEnabled, marketplaceHostRef, uiLocale, agentBridge, onActionStable, pluginInstallAbortsRef, uiLocaleRef, installingPluginIds, toolRunPanelNode, focusedProgram }: Pick<ReturnType<typeof useThemeAndDriverState>, "registry" | "extensionIdSet" | "pluginSource" | "loadedPlugins" | "pluginStatusById" | "primaryPluginId" | "session" | "catalog" | "extensionLedger" | "installPlugin" | "uninstallPlugin" | "reloadPlugin" | "installExtension" | "installExtensionFromFile" | "uninstallExtension" | "setExtensionEnabled" | "uiLocale" | "agentBridge" | "onActionStable" | "pluginInstallAbortsRef" | "uiLocaleRef" | "installingPluginIds" | "toolRunPanelNode" | "focusedProgram"> & Pick<ReturnType<typeof useSettingsHost>, "hubOnlyPlugins" | "marketplaceHostRef">) {
  const marketplaceHost: MarketplaceHostApi = useMemo(
    () => ({
      plugins: [
        ...registry.filter((entry) => !extensionIdSet.has(entry.pluginId)).map((entry) => ({ pluginId: entry.pluginId, sourceId: pluginSource.id })),
        ...hubOnlyPlugins.map((entry) => ({ pluginId: entry.pluginId, sourceId: `hub · ${entry.generationId.slice(0, 12)}` })),
      ].map(({ pluginId, sourceId }): MarketplacePluginEntry => {
        const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === pluginId);
        return {
          pluginId,
          label: loadedEntry?.manifest.label ?? pluginId,
          version: loadedEntry?.manifest.version,
          status: pluginStatusById[pluginId] ?? "available",
          sourceId,
          canUninstall: pluginId !== primaryPluginId && session?.pluginId !== pluginId,
        };
      }),
      extensions: (() => {
        const byId = new Map<string, MarketplaceExtensionEntry>();
        for (const target of catalog.extensions) {
          const ledger = extensionLedger.find((entry) => entry.extensionId === target.pluginId);
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === target.pluginId);
          byId.set(target.pluginId, {
            extensionId: target.pluginId,
            label: loadedEntry?.manifest.label ?? target.pluginId,
            version: ledger?.version ?? loadedEntry?.manifest.version,
            extendsHost: ledger?.extendsHost ?? target.extends ?? "unscoped",
            enabled: ledger?.enabled ?? false,
            status: pluginStatusById[target.pluginId] ?? (ledger ? "loaded" : "available"),
          });
        }
        for (const ledger of extensionLedger) {
          if (byId.has(ledger.extensionId)) continue;
          const loadedEntry = loadedPlugins.find((candidate) => candidate.handle.pluginId === ledger.extensionId);
          byId.set(ledger.extensionId, {
            extensionId: ledger.extensionId,
            label: loadedEntry?.manifest.label ?? ledger.extensionId,
            version: ledger.version,
            extendsHost: ledger.extendsHost,
            enabled: ledger.enabled,
            status: pluginStatusById[ledger.extensionId] ?? "loaded",
          });
        }
        return [...byId.values()];
      })(),
      installPlugin: (pluginId) => void installPlugin(pluginId),
      uninstallPlugin: (pluginId) => void uninstallPlugin(pluginId),
      reloadPlugin: (pluginId) => void reloadPlugin(pluginId),
      installExtensionFromUrl: (sourceUri) => void installExtension(sourceUri),
      installExtensionFromFile: (file) => void installExtensionFromFile(file),
      uninstallExtension: (extensionId) => void uninstallExtension(extensionId),
      setExtensionEnabled: (extensionId, enabled) => void setExtensionEnabled(extensionId, enabled),
    }),
    [
      catalog,
      registry,
      hubOnlyPlugins,
      extensionIdSet,
      extensionLedger,
      loadedPlugins,
      pluginStatusById,
      pluginSource.id,
      primaryPluginId,
      session?.pluginId,
      installPlugin,
      uninstallPlugin,
      reloadPlugin,
      installExtension,
      installExtensionFromFile,
      uninstallExtension,
      setExtensionEnabled,
    ],
  );
  marketplaceHostRef.current = marketplaceHost;
  const frameworkMarketplaceTab = useMemo(() => createFrameworkMarketplacePanelTab(() => marketplaceHostRef.current), [marketplaceHost, uiLocale]);
  const frameworkChatTab = useMemo(
    () => createFrameworkChatPanelTab(() => <AgentChatPanel status={agentBridge.status} presence={agentBridge.presence} versionMismatch={agentBridge.versionMismatch} conversation={agentBridge.conversation} onSendMessage={agentBridge.sendAgentMessage} onCancelToolCall={agentBridge.cancelToolCall} onResolveApproval={agentBridge.resolveApproval} />),
    [agentBridge.presence, agentBridge.status, agentBridge.versionMismatch, agentBridge.conversation, agentBridge.sendAgentMessage, agentBridge.cancelToolCall, uiLocale],
  );
  /** 🧵️ The `os.task-manager` window — the one central progress + cancel surface: every spawned plugin job
   * (the plugin runtime's own ledger), every plugin installation and every agent tool call still running,
   * each cancellable, above the actor table of the runtime this tab's plugins actually run on. The window
   * reads through stable `sources`, so a job advancing re-renders the window, never the dock. */
  const taskManagerShellTasksRef = useRef<readonly TaskManagerTaskV1[]>([]);
  const taskManagerListenersRef = useRef(new Set<() => void>());
  const taskManagerTasksCacheRef = useRef<{ readonly jobs: readonly SpawnedJobRowV1[]; readonly exports: readonly VideoRenderJobRow[]; readonly shell: readonly TaskManagerTaskV1[]; readonly transfers: readonly TaskManagerTaskV1[]; readonly tasks: readonly TaskManagerTaskV1[] } | null>(null);
  const documentTransfersRef = useRef(new Map<string, { readonly transfer: TaskManagerDocumentTransferV1; readonly abort: AbortController }>());
  const documentTransferTasksRef = useRef<readonly TaskManagerTaskV1[]>([]);
  const installStartedAtRef = useRef(new Map<string, number>());
  const toolRunStartedAtRef = useRef(new Map<string, number>());
  const toolRunControlsRef = useRef(new Map<string, { readonly pause: ToolRunPanelControlV1 | null; readonly resume: ToolRunPanelControlV1 | null; readonly abort: ToolRunPanelControlV1 | null }>());
  const fireToolRunControl = (task: TaskManagerTaskV1, control: "pause" | "resume" | "abort"): void => {
    const owned = toolRunControlsRef.current.get(task.id)?.[control];
    if (owned && !owned.disabled) void onActionStable(actionBindingToActionDescriptor(owned.binding));
  };
  const taskManagerCancelRef = useRef<(task: TaskManagerTaskV1) => void>(() => undefined);
  taskManagerCancelRef.current = (task) => {
    if (task.lane === "job") cancelSpawnedJobV1(task.id.slice("job:".length));
    else if (task.lane === "activation") pluginInstallAbortsRef.current.get(task.id.slice("install:".length))?.abort();
    else if (task.lane === "toolRun") fireToolRunControl(task, "abort");
    else if (task.lane === "documentTransfer") documentTransfersRef.current.get(task.id.slice("documentTransfer:".length))?.abort.abort();
    else if (task.lane === "export") cancelVideoRenderExportV1(task.id);
    else agentBridge.cancelToolCall(task.id.slice("tool:".length));
  };
  const taskManagerSuspendRef = useRef<(task: TaskManagerTaskV1, suspend: boolean) => void>(() => undefined);
  taskManagerSuspendRef.current = (task, suspend) => {
    if (task.lane === "toolRun") fireToolRunControl(task, suspend ? "pause" : "resume");
  };
  const taskManagerSources = useMemo((): TaskManagerSourcesV1 => ({
    registry: pluginRuntimeActivationRegistryV1,
    tasks: () => {
      const jobs = spawnedJobsSnapshotV1();
      const exports = videoRenderExportJobsSnapshotV1();
      const shell = taskManagerShellTasksRef.current;
      const transfers = documentTransferTasksRef.current;
      const cached = taskManagerTasksCacheRef.current;
      if (cached && cached.jobs === jobs && cached.exports === exports && cached.shell === shell && cached.transfers === transfers) return cached.tasks;
      const tasks = [...spawnedJobTasksV1(jobs), ...videoRenderExportTasksV1(exports, uiLocaleRef.current), ...shell, ...transfers];
      taskManagerTasksCacheRef.current = { jobs, exports, shell, transfers, tasks };
      return tasks;
    },
    subscribe: (listener) => {
      taskManagerListenersRef.current.add(listener);
      const unsubscribeJobs = subscribeSpawnedJobsV1(listener);
      const unsubscribeExports = subscribeVideoRenderExportJobsV1(listener);
      return () => {
        taskManagerListenersRef.current.delete(listener);
        unsubscribeJobs();
        unsubscribeExports();
      };
    },
    cancel: (task) => taskManagerCancelRef.current(task),
    suspend: (task) => taskManagerSuspendRef.current(task, true),
    resume: (task) => taskManagerSuspendRef.current(task, false),
  }), []);
  useEffect(() => {
    const startedAt = installStartedAtRef.current;
    for (const pluginId of [...startedAt.keys()]) if (!installingPluginIds.includes(pluginId)) startedAt.delete(pluginId);
    for (const pluginId of installingPluginIds) if (!startedAt.has(pluginId)) startedAt.set(pluginId, Date.now());
    const runs = toolRunPanelTasksV1(toolRunPanelNode);
    const program = { pluginId: focusedProgram?.pluginId ?? "", spawnedId: focusedProgram?.spawnedId ?? "" };
    const runIds = new Set(runs.map((run) => taskManagerToolRunIdV1(program.spawnedId, run.run)));
    const runStartedAt = toolRunStartedAtRef.current;
    for (const id of [...runStartedAt.keys()]) if (!runIds.has(id)) runStartedAt.delete(id);
    for (const id of runIds) if (!runStartedAt.has(id)) runStartedAt.set(id, Date.now());
    toolRunControlsRef.current = new Map(runs.map((run) => [taskManagerToolRunIdV1(program.spawnedId, run.run), { pause: run.pause, resume: run.resume, abort: run.abort }] as const));
    const toolRuns = toolRunTasksV1(runs.map((run) => ({ run: run.run, label: run.label, completed: run.completed, total: run.total, valueText: run.valueText, paused: run.pause === null })), program, runStartedAt);
    taskManagerShellTasksRef.current = [...installTasksV1(installingPluginIds, startedAt), ...toolCallTasksV1(agentBridge.conversation), ...toolRuns];
    for (const listener of [...taskManagerListenersRef.current]) listener();
  }, [agentBridge.conversation, installingPluginIds, toolRunPanelNode, focusedProgram?.pluginId, focusedProgram?.spawnedId]);
  const frameworkTaskManagerTab = useMemo(() => createFrameworkTaskManagerPanelTab(() => <TaskManagerWindow sources={taskManagerSources} />), [taskManagerSources, uiLocale]);
  return { frameworkMarketplaceTab, frameworkChatTab, taskManagerListenersRef, documentTransfersRef, documentTransferTasksRef, frameworkTaskManagerTab };
}
