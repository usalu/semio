/** 🔶️ The scope phase of useDocumentTransferAndTabs. */
import { type Severity } from "@semio-tech/framework";
import { decodePackValue } from "@semio-tech/framework/packs";
import { decodeDocumentArchiveBytes, encodeDocumentArchiveBytes, type DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { DOCUMENT_ARCHIVE_FILE_EXTENSION, DOCUMENT_ARCHIVE_MEDIA_TYPE, documentArchiveFileNameV1, naturalFileNameV1, naturalFileFormatV1, naturalMediaDescriptorMatchesV1, openNaturalFileOwnerV1, bytesOfDataUrlV1, documentTransferNoticeTextV1, documentTransferNoticeCodeV1, type DocumentTransferNoticeV1 } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1, NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1, readBlobBytesBoundedV1, requestFileSelectionV1, downloadMediaExportBytes, requestFileOpen } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { buildSpacePanelState, parsePanelState, studioPanelFocusingSpawned } from "../../../🛠️ShellHelpers/📌️panel/🟦️.ts";
import { type PluginWasmHandle } from "../../../🔌️PluginRuntime/🟦️.tsx";
import { documentTransferTasksV1, type TaskManagerDocumentTransferV1 } from "../../../🧵️TaskManager/🟦️.tsx";
import { requiredHostPanelLeafId } from "../../🧱️built-node-stores/🟦️.ts";
import type { ShellCoreAfterThemeDriverSettings } from "../../🎨️theme-driver-settings/🟦️.tsx";

/** 🔷 Builds scope, localDocumentOwnerRegistry, hostMode, dispatch of useDocumentTransferAndTabs. */
export function buildScopeState({ core }: { readonly core: ShellCoreAfterThemeDriverSettings }) {
  const {
    scope, localDocumentOwnerRegistry, hostMode, dispatch, loadedPlugins, session, uiLocale, uiTerminology,
    uiKeybindingOverrides, hostApp, landingAppId, hostAppId, spacePrograms, appLabelsOverlay, openWithFocusRole,
    clipboardFragmentRef, panelTreeConfigCacheRef, treeWindowGeneration, treeWindowHost, agentBridge, uiLocaleRef,
    showTransientNoticeRef, openDocumentSessionsRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion,
    foldSpaceDirectoryRef, panel, focusedApp, focusedSpawnedId, focusedPanelBodyStores, focusedWindowInstancesRef,
    activeUtilityByWindowIdRef, activeToolIdRef, utilityGenerationFor, toolGeneration, activeWindowIdRef,
    effectiveModeLayoutRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, updateSpacePanel,
    resolveSyncTargetSession, onAction, onActionStable, onBrowserActorIntent, dispatchSetDefaultApp,
    dispatchClearDefaultApp, openArtifactWithAppRef, openWithEntries, showTransientNotice, frameworkSettingsTab,
    taskManagerListenersRef, documentTransfersRef, documentTransferTasksRef
  } = core;
  //#region FrameworkOsShell
  //#region 📤️DocumentTransfer
  const publishDocumentTransfers = (): void => {
    documentTransferTasksRef.current = documentTransferTasksV1([...documentTransfersRef.current.values()].map((entry) => entry.transfer));
    for (const listener of [...taskManagerListenersRef.current]) listener();
  };
  const notifyDocumentTransfer = (notice: DocumentTransferNoticeV1, file: string, kind: Severity): void => {
    showTransientNoticeRef.current(documentTransferNoticeTextV1(notice, file, uiLocaleRef.current), kind, documentTransferNoticeCodeV1(notice));
  };
  /** 🧵️ One file moving into a program as a cancellable Tasks-window task (`documentTransfer` lane): `progress` states
   * `completed` of `total` units (`null` = open-ended), Cancel aborts `signal`, `finish` removes the row. */
  const trackDocumentTransfer = (file: string, pluginId: string): { readonly signal: AbortSignal; readonly progress: (completed: number, total: number | null) => void; readonly finish: () => void } => {
    const id = crypto.randomUUID();
    const abort = new AbortController();
    const update = (patch: Partial<TaskManagerDocumentTransferV1>): void => {
      const entry = documentTransfersRef.current.get(id);
      if (entry === undefined) return;
      documentTransfersRef.current.set(id, { ...entry, transfer: { ...entry.transfer, ...patch } });
      publishDocumentTransfers();
    };
    documentTransfersRef.current.set(id, { transfer: { id, file, pluginId, startedAtMs: Date.now(), progress: null, cancelling: false }, abort });
    abort.signal.addEventListener("abort", () => update({ cancelling: true }), { once: true });
    publishDocumentTransfers();
    return {
      signal: abort.signal,
      progress: (completed, total) => update({ progress: { completed, total, text: `${completed}/${total ?? "?"}` } }),
      finish: () => {
        documentTransfersRef.current.delete(id);
        publishDocumentTransfers();
      },
    };
  };
  /** 🎯️ The focused document program, or `null` when the Home or the space host has the canvas. */
  const documentTransferTarget = (): { readonly target: ActiveSession; readonly handle: PluginWasmHandle } | null => {
    const target = resolveSyncTargetSession();
    if (target === null || target.app.id === hostAppId || target.app.id === landingAppId) return null;
    const handle = loadedPlugins.find((entry) => entry.handle.pluginId === target.pluginId)?.handle;
    return handle === undefined ? null : { target, handle };
  };
  /** 📤️ Export Document: the focused program's archive, written as one `.semio-archive` file. */
  const exportDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const file = documentArchiveFileNameV1(owned.target.app.id, new Date());
    try {
      if (!owned.handle.readAppDocumentArchive) throw new Error("document-backbone.archive-reader-unavailable");
      downloadMediaExportBytes(file, DOCUMENT_ARCHIVE_MEDIA_TYPE, encodeDocumentArchiveBytes(await owned.handle.readAppDocumentArchive(owned.target.instanceId)));
      notifyDocumentTransfer("exported", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("export-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("export-failed", file, "error");
    }
  };
  /** 📥️ Import Document: a picked archive opens as a NEW document of the focused program, loaded with progress and
   * cancellation from the Tasks window; the focused document stays untouched. */
  const importDocumentArchive = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    const load = owned.handle.loadAppDocumentArchive;
    const [picked] = await requestFileOpen(`${DOCUMENT_ARCHIVE_FILE_EXTENSION},${DOCUMENT_ARCHIVE_MEDIA_TYPE}`, "dataUrl", false);
    if (picked === undefined) return;
    let archive: DocumentArchivePack;
    try {
      archive = decodeDocumentArchiveBytes(bytesOfDataUrlV1(picked.contents));
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("import-unreadable")} ${picked.name}`, error);
      return notifyDocumentTransfer("import-unreadable", picked.name, "warning");
    }
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    try {
      if (!load) throw new Error("document-backbone.archive-loader-unavailable");
      instanceId = await owned.handle.createApp(owned.target.app.id);
      await load(instanceId, archive, task.signal, (status) => task.progress(status.completed, status.total > 0 ? status.total : null));
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      updateSpacePanel(studioPanelFocusingSpawned(currentPanel, { id: `${owned.target.pluginId}-${instanceId}`, pluginId: owned.target.pluginId, instanceId, appId: owned.target.app.id, label: program?.label ?? owned.target.app.id, breadcrumb: program?.breadcrumb ?? [] }));
      notifyDocumentTransfer("imported", picked.name, "info");
    } catch (error) {
      if (instanceId !== null) await owned.handle.destroyApp(instanceId).catch((destroyError: unknown) => console.error(`[os-shell] imported instance ${instanceId} could not be closed`, destroyError));
      const notice: DocumentTransferNoticeV1 = task.signal.aborted ? "import-cancelled" : "import-failed";
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1(notice)} ${picked.name}`, error);
      notifyDocumentTransfer(notice, picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  const saveNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const exportMedia = owned.handle.exportAppMedia;
    const file = format === null ? "" : naturalFileNameV1(owned.target.app.id, format.extension, new Date());
    const task = trackDocumentTransfer(file, owned.target.pluginId);
    try {
      if (format === null || exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");
      if (task.signal.aborted) throw task.signal.reason;
      task.progress(0, 1);
      const artifact = await exportMedia(owned.target.instanceId, "artifact:native");
      if (artifact === null || task.signal.aborted) throw task.signal.reason ?? new Error("natural-file.export-empty");
      if (!naturalMediaDescriptorMatchesV1(decodePackValue(artifact.descriptor), format)) throw new Error("natural-file.export-descriptor-mismatch");
      downloadMediaExportBytes(file, format.mediaType, artifact.data);
      task.progress(1, 1);
      notifyDocumentTransfer("file-saved", file, "info");
    } catch (error) {
      console.error(`[os-shell] ${documentTransferNoticeCodeV1("file-save-failed")} ${owned.target.pluginId}#${owned.target.instanceId}`, error);
      notifyDocumentTransfer("file-save-failed", file, "error");
    } finally {
      task.finish();
    }
  };
  const openNaturalFile = async (): Promise<void> => {
    const owned = documentTransferTarget();
    if (owned === null || session === null) return notifyDocumentTransfer("no-document", "", "info");
    const format = naturalFileFormatV1(owned.target.app);
    const importMedia = owned.handle.importAppMedia;
    if (format === null || importMedia === undefined) return notifyDocumentTransfer("file-open-failed", "", "error");
    const [picked] = await requestFileSelectionV1(`${format.extension},${format.mediaType}`);
    if (picked === undefined) return;
    const task = trackDocumentTransfer(picked.name, owned.target.pluginId);
    let instanceId: number | null = null;
    try {
      if (task.signal.aborted) throw task.signal.reason;
      const bytes = await readBlobBytesBoundedV1(picked, {
        signal: task.signal,
        maximumBytes: NATURAL_FILE_BROWSER_MAXIMUM_BYTES_V1,
        chunkBytes: NATURAL_FILE_BROWSER_READ_CHUNK_BYTES_V1,
        progress: task.progress,
      });
      instanceId = await openNaturalFileOwnerV1({
        signal: task.signal,
        create: () => owned.handle.createApp(owned.target.app.id),
        importBytes: (freshInstanceId) => importMedia(freshInstanceId, "artifact:native", { portId: "artifact:native", kindId: format.formatKind, wire: { kind: "binary", format_kind: format.formatKind } }, bytes),
        retire: (freshInstanceId) => owned.handle.destroyApp(freshInstanceId),
      });
      const program = spacePrograms.find((entry) => entry.pluginId === owned.target.pluginId && entry.appId === owned.target.app.id);
      const currentPanel = parsePanelState(session.viewState) ?? buildSpacePanelState([], requiredHostPanelLeafId(hostApp));
      updateSpacePanel(studioPanelFocusingSpawned(currentPanel, { id: `${owned.target.pluginId}-${instanceId}`, pluginId: owned.target.pluginId, instanceId, appId: owned.target.app.id, label: program?.label ?? owned.target.app.id, breadcrumb: program?.breadcrumb ?? [] }));
      notifyDocumentTransfer("file-opened", picked.name, "info");
    } catch (error) {
      console.warn(`[os-shell] ${documentTransferNoticeCodeV1("file-open-failed")} ${picked.name}`, error);
      notifyDocumentTransfer("file-open-failed", picked.name, task.signal.aborted ? "info" : "error");
    } finally {
      task.finish();
    }
  };
  return { exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer, dispatch, agentBridge, session, focusedApp, focusedWindowInstancesRef, activeWindowIdRef, focusedSpawnedId, effectiveModeLayoutRef, uiKeybindingOverrides, activeUtilityByWindowIdRef, onAction, utilityGenerationFor, activeToolIdRef, toolGeneration, appLabelsOverlay, uiTerminology, uiLocale, showTransientNotice, clipboardFragmentRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, scope, openDocumentSessionsRef, localDocumentOwnerRegistry, panel, foldSpaceDirectoryRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion, onBrowserActorIntent, onActionStable, focusedPanelBodyStores, treeWindowHost, panelTreeConfigCacheRef, hostMode, hostAppId, openWithFocusRole, openArtifactWithAppRef, dispatchClearDefaultApp, dispatchSetDefaultApp, openWithEntries, treeWindowGeneration, frameworkSettingsTab };
}
