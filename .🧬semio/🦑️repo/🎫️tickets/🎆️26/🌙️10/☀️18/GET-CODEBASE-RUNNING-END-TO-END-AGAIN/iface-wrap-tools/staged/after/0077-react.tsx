/** 🔸️ The document transfer ref phase of useDocumentTransferAndTabs. */
import { useCallback, useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { type LocalizedLabel, panelTabKindId, resolveWindowActions, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID } from "@semio-tech/framework";
import { KEYBINDING_UNOWNED_CODE, keybindingUnownedTextV1, modeLayoutStacksV1, reservedShellChordsV1, resolveKeybindingTargetWindowV1, type WindowScopeInstanceV1, type WindowScopeKindV1, type WindowScopeLayoutNodeV1 } from "../../⌨️window-scope/🟦️.ts";
import { SHELL_KEYBINDINGS, formatKeybindingShortcut, useShellKeydown } from "@semio-tech/ui-react";
import { localDocumentOwnerIdentityForSessionV1 } from "../../../🗣️Interpreter/🧭️local-document-owner/🟦️.ts";
import { actionStageKey } from "../../../🐚️Shell/🗃️state-shapes/🟦️.ts";
import { actionRequiresStagedForm } from "../../../🛠️ShellHelpers/🪟️window-panes/📝️staged-args/🟦️.tsx";
import { controlOwnsKeyboardEvent, keyboardEventMatchesChord, resolveKeybindingIntent } from "../../../🛠️ShellHelpers/🪟️window-panes/⌨️keyboard-chords/🟦️.ts";
import { resolveAppLabel, resolveManifestLabel } from "../../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { panelAnchorForGroup } from "../../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";
import { type BrowserActorPanelHostV1, uiIntentToActionDescriptor } from "../../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { pasteActionWithRetainedFragment, type ClipboardActionDescriptor, pasteArgsFragment } from "../../../🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
import { FRAMEWORK_CHAT_PANEL_ID } from "../../../📌️ChromePanels/🆔️panel-ids/🟦️.ts";
import { focusAgentApprovalV1 } from "../../../🤖️AgentApprovals/🟦️.tsx";
import type { ShellCoreAfterThemeDriverSettings } from "../../🎨️theme-driver-settings/🟦️.tsx";
import type { buildScopeState } from "../🔶️scope/🟦️.tsx";

/** 🔹 Builds documentTransferRef, revealAgentApproval, revealedApprovalIdsRef, handleAppKeydown of useDocumentTransferAndTabs. */
export function useDocumentTransferRefState({ exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer, core, dispatch, agentBridge, session, focusedApp, focusedWindowInstancesRef, activeWindowIdRef, focusedSpawnedId, effectiveModeLayoutRef, uiKeybindingOverrides, activeUtilityByWindowIdRef, onAction, utilityGenerationFor, activeToolIdRef, toolGeneration, appLabelsOverlay, uiTerminology, uiLocale, showTransientNotice, clipboardFragmentRef, actionPaneExpandedByWindowIdRef, actionPaneStagedArgsByKeyRef, scope, openDocumentSessionsRef, localDocumentOwnerRegistry, panel, foldSpaceDirectoryRef, browserActorUiByRuntimeKeyRef, browserActorUiVersion, onBrowserActorIntent, onActionStable }: Pick<ReturnType<typeof buildScopeState>, "exportDocumentArchive" | "importDocumentArchive" | "saveNaturalFile" | "openNaturalFile" | "trackDocumentTransfer" | "notifyDocumentTransfer" | "dispatch" | "agentBridge" | "session" | "focusedApp" | "focusedWindowInstancesRef" | "activeWindowIdRef" | "focusedSpawnedId" | "effectiveModeLayoutRef" | "uiKeybindingOverrides" | "activeUtilityByWindowIdRef" | "onAction" | "utilityGenerationFor" | "activeToolIdRef" | "toolGeneration" | "appLabelsOverlay" | "uiTerminology" | "uiLocale" | "showTransientNotice" | "clipboardFragmentRef" | "actionPaneExpandedByWindowIdRef" | "actionPaneStagedArgsByKeyRef" | "scope" | "openDocumentSessionsRef" | "localDocumentOwnerRegistry" | "panel" | "foldSpaceDirectoryRef" | "browserActorUiByRuntimeKeyRef" | "browserActorUiVersion" | "onBrowserActorIntent" | "onActionStable"> & { readonly core: ShellCoreAfterThemeDriverSettings }) {
  const documentTransferRef = useRef({ exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer });
  core.documentTransferRef = documentTransferRef;
  documentTransferRef.current = { exportDocumentArchive, importDocumentArchive, saveNaturalFile, openNaturalFile, trackDocumentTransfer, notifyDocumentTransfer };
  //#endregion 📤️DocumentTransfer
  /** ⛩️ An approval request has ONE affordance: its row in the agent conversation. Revealing the chat panel
   * is what makes it visible; `focus` additionally moves keyboard focus onto it (the footer notice's
   * Review), retried across a few frames while the panel mounts. */
  const revealAgentApproval = useCallback((approvalId: string, focus = true) => {
    dispatch({ type: "SET_PANEL_PATH", anchor: "top-right", value: [FRAMEWORK_CHAT_PANEL_ID] });
    dispatch({ type: "SET_PANEL_VISIBLE", anchor: "top-right", value: true });
    if (!focus) return;
    let frames = 0;
    const attempt = () => {
      if (focusAgentApprovalV1(approvalId) || frames++ >= 30) return;
      requestAnimationFrame(attempt);
    };
    requestAnimationFrame(attempt);
  }, []);
  /** 🔔️ Every NEWLY arrived approval reveals the conversation it lives in — without taking focus from what
   * the human is typing; the footer notice and its live region carry the announcement. */
  const revealedApprovalIdsRef = useRef(new Set<string>());
  useEffect(() => {
    for (const approval of agentBridge.pendingApprovals) {
      if (revealedApprovalIdsRef.current.has(approval.approvalId)) continue;
      revealedApprovalIdsRef.current.add(approval.approvalId);
      revealAgentApproval(approval.approvalId, false);
    }
  }, [agentBridge.pendingApprovals, revealAgentApproval]);

  const handleAppKeydown = useCallback(
    (event: globalThis.KeyboardEvent) => {
      if (event.defaultPrevented || controlOwnsKeyboardEvent(event)) return;
      if (!session) return;
      const keyApp = focusedApp ?? session.app;
      const parseKeys = (keys: string) =>
        keys
          .split(",")
          .map((key) => key.trim().toLowerCase())
          .filter(Boolean);
      const instances = focusedWindowInstancesRef.current;
      const focusedWindowId = activeWindowIdRef.current ?? (focusedSpawnedId !== null ? (instances[0]?.id ?? null) : (session.viewState.windowId ?? session.viewState.activeWindowKindId ?? null));
      const mounted: readonly WindowScopeInstanceV1[] = modeLayoutStacksV1(effectiveModeLayoutRef.current as WindowScopeLayoutNodeV1 | null)
        .flatMap((stack) => [...stack.windowIds])
        .map((id) => ({ id, windowKindId: instances.find((instance) => instance.id === id)?.windowKindId ?? id }));
      const scopeKinds: readonly WindowScopeKindV1[] = keyApp.windowKinds.map((kind) => ({ id: kind.id, actionIds: resolveWindowActions(keyApp, kind).map((action) => action.id) }));
      const reservedChords = reservedShellChordsV1(SHELL_KEYBINDINGS, uiKeybindingOverrides);
      const actionDefinition = (windowKindId: string, actionId: string) => {
        const kind = keyApp.windowKinds.find((entry) => entry.id === windowKindId);
        return kind ? resolveWindowActions(keyApp, kind).find((action) => action.id === actionId) : undefined;
      };
      if (event.key === "Escape") {
        const windowId = activeWindowIdRef.current;
        if (windowId && activeUtilityByWindowIdRef.current[windowId]) {
          event.preventDefault();
          void onAction({ controllerId: keyApp.controllerId, action: SET_ACTIVE_UTILITY_ACTION_ID, args: { windowId, utilityId: "", expectedGeneration: utilityGenerationFor(windowId) } });
          return;
        }
        if (activeToolIdRef.current) {
          event.preventDefault();
          void onAction({ controllerId: keyApp.controllerId, action: SET_ACTIVE_TOOL_ACTION_ID, args: { toolId: "", expectedGeneration: toolGeneration() } });
          return;
        }
      }
      for (const binding of keyApp.keybindings) {
        for (const chord of parseKeys(binding.keys)) {
          if (!keyboardEventMatchesChord(event, chord)) continue;
          if (reservedChords.has(chord)) continue;
          const target = resolveKeybindingTargetWindowV1({ kinds: scopeKinds, mounted, focusedWindowId, actionId: binding.action.action });
          event.preventDefault();
          if (target.windowId === null) {
            const anyDefinition = keyApp.windowKinds.flatMap((kind) => resolveWindowActions(keyApp, kind)).find((action) => action.id === binding.action.action);
            const label = anyDefinition ? resolveAppLabel(appLabelsOverlay, "action", anyDefinition.id, resolveManifestLabel(anyDefinition.label as LocalizedLabel | string, uiTerminology, uiLocale)) : binding.action.action;
            showTransientNotice(keybindingUnownedTextV1(uiLocale, formatKeybindingShortcut(chord), label), "info", KEYBINDING_UNOWNED_CODE);
            return;
          }
          const targetWindowId = target.windowId;
          const targetKindId = mounted.find((instance) => instance.id === targetWindowId)?.windowKindId ?? targetWindowId;
          const definition = actionDefinition(targetKindId, binding.action.action);
          if (!definition) return;
          if (target.kind === "owner") dispatch({ type: "SET_ACTIVE_WINDOW_ID", value: targetWindowId });
          if (actionRequiresStagedForm(definition)) {
            const retainedPaste = pasteActionWithRetainedFragment<ClipboardActionDescriptor>({ action: definition.id }, clipboardFragmentRef.current);
            const retainedArgs = retainedPaste.args;
            if (pasteArgsFragment(retainedPaste) !== undefined) {
              onAction({ controllerId: keyApp.controllerId, action: retainedPaste.action, args: { ...(typeof retainedArgs === "object" && retainedArgs !== null ? retainedArgs : {}), windowId: targetWindowId } });
              return;
            }
            const expanded = actionPaneExpandedByWindowIdRef.current[targetWindowId] ?? null;
            const staged = actionPaneStagedArgsByKeyRef.current[actionStageKey(targetWindowId, definition.id)] ?? {};
            const intent = resolveKeybindingIntent(definition, expanded, staged);
            if (intent.kind === "execute") {
              onAction({ controllerId: keyApp.controllerId, action: intent.actionId, args: { ...(typeof intent.args === "object" && intent.args !== null ? intent.args : {}), windowId: targetWindowId } });
            } else if (intent.kind === "open") {
              dispatch({ type: "SET_ACTION_PANE_FOLDED", windowId: targetWindowId, value: false });
              dispatch({ type: "SET_ACTION_PANE_EXPANDED", windowId: targetWindowId, value: intent.actionId });
            }
            return;
          }
          onAction({ ...binding.action, args: { ...(typeof binding.action.args === "object" && binding.action.args !== null ? (binding.action.args as Record<string, unknown>) : {}), windowId: targetWindowId } });
          return;
        }
      }
      if ((event.ctrlKey || event.metaKey) && !event.altKey && event.key.toLowerCase() === "z") {
        event.preventDefault();
        onAction({ controllerId: keyApp.controllerId, action: event.shiftKey ? "redo" : "undo" });
        return;
      }
      if ((event.ctrlKey || event.metaKey) && !event.altKey && !event.shiftKey && event.key.toLowerCase() === "y") {
        event.preventDefault();
        onAction({ controllerId: keyApp.controllerId, action: "redo" });
        return;
      }
    },
    [onAction, session, focusedApp, focusedSpawnedId, appLabelsOverlay, showTransientNotice, uiKeybindingOverrides, uiLocale, uiTerminology, utilityGenerationFor, toolGeneration],
  );
  useShellKeydown(scope.rootRef, handleAppKeydown, [handleAppKeydown]);

  /** 📌️ Reverse-lookup: `openDocumentSessionsRef` is keyed by exact runtime identity, never
   * the other way around (no `ActiveSession.documentId` field exists — see `📓️w3-a-report.md`'s
   * "Design decisions"). This tiny scan intentionally runs on every render because `openDocument`
   * fills the ref while retaining the same visible session identity. */
  const currentDocumentRuntimeKey = (() => {
    if (!session) return null;
    for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return runtimeKey;
    }
    return null;
  })();
  const currentDocumentEntry = currentDocumentRuntimeKey === null ? undefined : openDocumentSessionsRef.current.get(currentDocumentRuntimeKey);
  const currentLocalDocumentOwnerIdentity = !session
    ? null
    : localDocumentOwnerIdentityForSessionV1(
      { pluginId: session.pluginId, appId: session.app.id, sessionInstanceId: session.instanceId },
      currentDocumentRuntimeKey === null || currentDocumentEntry === undefined ? undefined : { runtimeKey: currentDocumentRuntimeKey, clientInstanceId: currentDocumentEntry.clientInstanceId },
    );
  const currentLocalDocumentOwner = currentLocalDocumentOwnerIdentity === null ? null : localDocumentOwnerRegistry.acquire(currentLocalDocumentOwnerIdentity);
  useEffect(() => {
    localDocumentOwnerRegistry.retain([
      ...[...openDocumentSessionsRef.current.entries()].map(([runtimeKey, entry]) => ({ pluginId: entry.session.pluginId, appId: entry.session.app.id, sessionInstanceId: entry.session.instanceId, runtimeKey, clientInstanceId: entry.clientInstanceId })),
      ...(panel?.spawnedApps ?? []).map(spawned => ({ pluginId: spawned.pluginId, appId: spawned.appId, sessionInstanceId: spawned.instanceId, runtimeKey: `spawned:${spawned.id}`, clientInstanceId: spawned.id })),
      ...(currentDocumentEntry === undefined && currentLocalDocumentOwnerIdentity !== null ? [currentLocalDocumentOwnerIdentity] : []),
    ]);
  });
  useEffect(() => {
    foldSpaceDirectoryRef.current();
  }, [currentDocumentRuntimeKey, session?.pluginId, session?.instanceId]);
  const currentBrowserActorUi = useMemo(
    () => currentDocumentRuntimeKey === null ? undefined : browserActorUiByRuntimeKeyRef.current.get(currentDocumentRuntimeKey),
    [browserActorUiVersion, currentDocumentRuntimeKey],
  );
  /** 🎭️ The panel route of an actor-bound document ({@link BrowserActorPanelHostV1}): its intents go to the verified
   * actor under the panel's own surface; `browserActorUiVersion` moves when a panel store first arrives. */
  const browserActorPanelIntent = useMemo(
    () => currentDocumentRuntimeKey === null || currentBrowserActorUi === undefined ? null : (tabId: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, tabId, intent),
    [currentBrowserActorUi, currentDocumentRuntimeKey, onBrowserActorIntent],
  );
  const browserActorPanels = useMemo(
    (): BrowserActorPanelHostV1 | null => !session || browserActorPanelIntent === null || currentBrowserActorUi?.sessionInstanceId !== session.instanceId ? null : { stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: onActionStable },
    [browserActorPanelIntent, browserActorUiVersion, currentBrowserActorUi, onActionStable, session],
  );

  const focusedBrowserActorPanels = focusedSpawnedId === null ? browserActorPanels : null;
  const activeRightPanelTab = focusedApp?.panelTabs.find((tab) => panelAnchorForGroup(tab.group) === "top-right");
  const activePanelTabId = panel?.activePanelTab ?? (activeRightPanelTab ? panelTabKindId(activeRightPanelTab.kind) : undefined) ?? (focusedApp?.panelTabs[0] ? panelTabKindId(focusedApp.panelTabs[0].kind) : undefined);
  return { focusedBrowserActorPanels, revealAgentApproval, currentDocumentRuntimeKey, currentLocalDocumentOwner, currentBrowserActorUi };
}
