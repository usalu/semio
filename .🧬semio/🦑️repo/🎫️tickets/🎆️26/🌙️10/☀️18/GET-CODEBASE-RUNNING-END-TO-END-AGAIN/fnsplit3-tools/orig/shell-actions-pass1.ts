/** 🖱️ `onAction`, shell command notes, the stable action and intent callbacks and the browser actor intent.
 *
 * 🧱️ Child hook module of {@link ../🟦️.tsx | ShellHost}: `useShellActions` runs the original statements in order and returns the bindings later hooks and the render read; the parent merges them into the shared core object.
 */

import { shellDialogSessionIsCurrentV1, type ShellDialogOriginV1 } from "../🗨️dialog-origin/🟦️.ts";

import { useCallback, useEffect, useRef } from "react";
// #region 🔌️Adapters
import {
  type ActionDispatchLifecycleV1,
  type ActionDescriptor,
  SemioFaultError,
  SURFACE_FAULT_CODES,
  type HistoryPatch,
  CLEAR_SELECTION_ACTION_ID,
  panelTabKindId,
  RECORD_TUTORIAL_ACTION_ID,
  resolveUiDirtyScope,
  resolveWindowActions,
  SET_ACTIVE_TOOL_ACTION_ID,
  SET_ACTIVE_UTILITY_ACTION_ID,
  EXPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  IMPORT_ARTIFACT_DOCUMENT_ACTION_ID,
  OPEN_ARTIFACT_FILE_ACTION_ID,
  SAVE_ARTIFACT_FILE_ACTION_ID,
  START_INTRODUCTION_ACTION_ID,
  START_TUTORIAL_ACTION_ID,
  windowElementId,
  hostArmedViewContext,
  panelViewContext,
  windowViewContext
} from "@semio-tech/framework";
import { decodePackWire } from "@semio-tech/framework/packs";
import { FRAMEWORK_SYNC_CONTROLLER_ID } from "@semio-tech/framework/backbone";
import {
  programHistoryKeyV1,
  programHistoryProjectionV1,
  spawnedProgramViewStateV1,
  guestActiveUtilityByWindowIdV1,
  guestWindowIdV1,
  spawnedIdOfWindowInstanceV1
} from "../🪟️spawned-program/🟦️.ts";
import {
  causalOrderKeyV1,
  expectedGenerationFromArgsV1,
  INPUT_IDENTITY_SETTLE_BOUND_MS_V1,
  inputActionWindowV1,
  inputActionWithWindowV1,
  inputAppliedV1,
  inputAwaitsHubIdentityV1,
  inputCommitReceiptV1,
  inputDiagnosticV1,
  inputIdentitySettleNoticeTextV1,
  inputRefusalNoticeTextV1,
  inputRefusalNotifiesV1,
  inputRefusalTextV1,
  inputRefusedV1,
  resolveUtilityActivationV1,
  type InputCommitReceiptV1,
  type InputDiagnosticV1,
  type InputDraftDispositionV1,
  type InputOutcomeV1,
  type InputRefusalReasonV1,
  type ShellInputActionV1
} from "../🎯️input-ledger/🟦️.ts";
import { type ActiveSession } from "../../🐚️Shell/📦️program-sessions/🟦️.ts";




import {
  NOTE_WORLD_NAVIGATION_ACTION_ID,
  TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS,
  buildNoteShellCommandAction,
  type ShellLabelTextV1,
  undeclaredActionDiagnostic
} from "../../🛠️ShellHelpers/🔩️plumbing/🆔️action-ids/🟦️.ts";
import {
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1,
  frameworkOwnsExampleSwitch,
  historyRefreshNeededV1
} from "../../🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";

import {
  flattenPanelTabLeaves,
  panelAnchorForGroup,
  appFaultNoticeV1,
  historyFaultNoticeV1,
  historyInputDraftDispositionV1,
  historyOutputNoticeV1,
  historySilentRefusalOfFaultV1,
  historySilentRefusalOfOutputV1,
  frameworkLayoutDeclaredInstances
} from "../../🛠️ShellHelpers/🛣️route-and-chrome/🟦️.tsx";


import { checkinSubmitMessageV1 } from "../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { FRAMEWORK_CHECKIN_CONTROLLER_ID, shellLabel, viewerReadOnlyNoticeText } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { requestFileOpen } from "../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { pasteActionWithRetainedFragment } from "../../🛠️ShellHelpers/🪟️window-panes/📋️clipboard-activation/🟦️.ts";
import { uiIntentToActionDescriptor } from "../../🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx";
import { abortExtensionRequestsForActor, isDeclaredSurfaceCancelAction } from "../../🛠️ShellHelpers/🛑️extension-io/🟦️.ts";
// #endregion 🔌️Adapters
import { browserActorActionRefusalReasonV1 } from "@semio-tech/framework/browser-bundle/action-handoff/requests";
import { BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION } from "@semio-tech/framework/browser-bundle/action-handoff";
import { publishBrowserActorHostEffectsV1 } from "@semio-tech/framework/browser-bundle/action-handoff/publication";


import { typedOperationCancellationOwnerIsCurrentV1 } from "../🏁️operation-settle/🟦️.ts";



import type { ShellCoreAfterShellUriAndDocuments } from "../🗄️shell-uri-documents/🟦️.ts";
import type { RetainedBrowserActorUiV1 } from "../🔒️identity-document-ports/🧰️identity-port-types/🟦️.ts";
import { handleSetActiveToolAction, handleSetActiveUtilityAction } from "./🧰️utility-and-tool-activation/🟦️.ts";
import { handleRecoveryAction, handleSetActivePanelTabAction, handleSyncCardAction, handleWorldNavigationAction } from "./🔄️recovery-navigation-sync-and-panel-actions/🟦️.ts";
import { buildDispatchViewState, resolveDispatchTarget, routeRemoteUndo } from "./🎯️dispatch-target-and-view-state/🟦️.ts";
import { dispatchDirectBrowserActorAction, dispatchProgramAction } from "./🚀️program-dispatch/🟦️.ts";

export type ShellCoreAfterShellActions = ShellCoreAfterShellUriAndDocuments & ReturnType<typeof useShellActions>;

//#region FrameworkOsShell
export function useShellActions(core: ShellCoreAfterShellUriAndDocuments) {
  const {
    hostMode, mobile, defaults, dispatch, historyStore, exampleOptionsRef, lastDispatchedExampleIdRef,
    localHistoryOrderRef, loadedPlugins, pluginSupervisorById, session, uiLocale, uiTerminology, applyHistoryPatch,
    applyLeftoverInteractionView, refreshHistorySnapshot, hostApp, hostControllerId, hostCatalogueTabId, spacePrograms,
    syncCardKind, syncDraftPath, clipboardFragmentRef, extraWindowInstancesRef, awaitOperationSettle, sessionWorkRef,
    dropForSealedInstance, dropForRetiredInstance, treeWindowSchedulerRef, identityRef, verifiedSessionAuthorityRef,
    uiLocaleRef, uiTerminologyRef, showTransientNoticeRef, dispatchCheckpointRef, inferenceHistoryByRuntimeKeyRef,
    openDocumentSessionsRef, captureDialogOrigin, isCurrentDialogOrigin, captureEffectOwner, isCurrentEffectOwner,
    browserActorUiByRuntimeKeyRef, ensureBackboneWorker, primaryPluginId, reloadPlugin, uninstallPlugin, panel,
    spawnedIdsRef, historyCanUndo, activeUtilityByWindowIdRef, activeToolIdRef, inputLedgerRef, refusalNoticeThrottleRef,
    hubIdentityGateRef, hubIdentitySettleRef, writeUtilityRegister, clearAllWindowUtilities, writeToolRegister,
    activeWindowIdRef, startTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef, tutorialPlayingRef,
    tutorialRecordingRef, tutorialRecorderRef, shellStateRef, observeLocalInteraction, completeIntroductionInteraction,
    injectActiveUtility, updateSpacePanel, requestServiceOperation, directBrowserActorForSession,
    dispatchDirectBrowserActorCommand, applyHostEffects, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath,
    spawnProgram
  } = core;
  /** 🎯️ The shell's ONE input funnel. Every call mints a `🎯️input-ledger` entry first and settles it with a
   * typed outcome — `applied` for every branch that ran (shell-intercepted or guest-dispatched and settled),
   * `refused(reason)` for every gate that used to be a silent `return`. A refusal prints one plain
   * `console.warn` line and, for a user-origin input whose reason the notice table admits, one throttled
   * notice (§G). `propagateFailure` keeps rethrowing AFTER the ledger is settled (the tutorial director). */
  const onAction = useCallback(
    async (requested: ShellInputActionV1, submittedOrigin?: ShellDialogOriginV1, propagateFailure = false, lifecycle?: ActionDispatchLifecycleV1, requiredOperationOwner?: ReturnType<typeof captureEffectOwner>): Promise<InputOutcomeV1> => {
      const requestedWindowId = inputActionWindowV1(requested, activeWindowIdRef.current);
      const ledger = inputLedgerRef.current;
      const entry = ledger.issue(requested, { windowId: requestedWindowId ?? activeWindowIdRef.current ?? null, origin: requested.provenance?.origin ?? "user" });
      const refuse = (reason: InputRefusalReasonV1, detail?: string, alreadyNotified = false, diagnostic?: InputDiagnosticV1, draftDisposition?: InputDraftDispositionV1): InputOutcomeV1 => {
        const outcome = inputRefusedV1(entry.provenance.inputSeq, reason, detail, diagnostic, draftDisposition);
        if (ledger.settle(outcome) && outcome.kind === "refused") {
          console.warn(inputRefusalTextV1(entry.action, outcome, entry.provenance));
          if (!alreadyNotified && inputRefusalNotifiesV1(outcome, entry.provenance) && refusalNoticeThrottleRef.current.admit(reason, performance.now())) {
            showTransientNoticeRef.current(inputRefusalNoticeTextV1(reason, uiLocaleRef.current), "info");
          }
        }
        return outcome;
      };
      const applied = (commit?: InputCommitReceiptV1): InputOutcomeV1 => {
        const outcome = inputAppliedV1(entry.provenance.inputSeq, commit);
        ledger.settle(outcome);
        return outcome;
      };
      /** 🚦️ The one place a thrown dispatch error becomes a reason — shared by the direct-actor route and
       * the `handleAction` route so both speak the ledger's vocabulary. */
      const refusalReasonForError = (error: unknown): { readonly reason: InputRefusalReasonV1; readonly detail?: string; readonly diagnostic?: InputDiagnosticV1; readonly draftDisposition?: InputDraftDispositionV1 } => {
        const diagnostic = error instanceof SemioFaultError ? inputDiagnosticV1(error.fault) : undefined;
        const draftDisposition = error instanceof SemioFaultError ? historyInputDraftDispositionV1(error.fault) : undefined;
        if (core.isViewerReadOnlyFault(error)) return { reason: "viewer-read-only", diagnostic, draftDisposition };
        if (core.isMutationRejectedFault(error)) return { reason: "mutation-rejected", diagnostic, draftDisposition };
        const text = String(error instanceof Error ? error.message : error);
        if (/queue is full|queue full|another action pending|action-busy/u.test(text)) return { reason: "queue-full", detail: text, diagnostic, draftDisposition };
        if (/action-catching-up/u.test(text)) return { reason: "catching-up", detail: text, diagnostic, draftDisposition };
        if (/action-owner-mismatch|owner retired|ambiguous document owner/u.test(text)) return { reason: "owner-mismatch", detail: text, diagnostic, draftDisposition };
        return { reason: "dispatch-failed", detail: text, diagnostic, draftDisposition };
      };
      try {
        const action = pasteActionWithRetainedFragment(requested, clipboardFragmentRef.current);
        const recoveryOutcome = handleRecoveryAction({ primaryPluginId, dispatch, reloadPlugin, uninstallPlugin, action, refuse, applied });
        if (recoveryOutcome !== undefined) return recoveryOutcome;

        if (!session) return refuse("no-session");
        if (entry.provenance.origin === "user" && inputAwaitsHubIdentityV1(hubIdentityGateRef.current)) {
          showTransientNoticeRef.current(inputIdentitySettleNoticeTextV1(uiLocaleRef.current), "info");
          await hubIdentitySettleRef.current.wait(INPUT_IDENTITY_SETTLE_BOUND_MS_V1);
        }
        const programOfWindow = (windowId: string): { readonly spawnedId: string | null; readonly session: ActiveSession; readonly extraInstances: readonly { readonly id: string; readonly windowKindId: string }[]; readonly guestWindowId: string } => {
          const spawnedId = spawnedIdOfWindowInstanceV1(windowId, spawnedIdsRef.current);
          const spawned = spawnedId === null ? undefined : panel?.spawnedApps.find((candidate) => candidate.id === spawnedId);
          const app = spawned ? loadedPlugins.find((candidate) => candidate.handle.pluginId === spawned.pluginId)?.manifest.apps.find((candidate) => candidate.id === spawned.appId) : undefined;
          if (!spawned || !app) return { spawnedId: null, session, extraInstances: extraWindowInstancesRef.current, guestWindowId: windowId };
          return {
            spawnedId: spawned.id,
            session: { pluginId: spawned.pluginId, instanceId: spawned.instanceId, app, viewState: spawnedProgramViewStateV1(session.viewState, app) },
            extraInstances: frameworkLayoutDeclaredInstances(app.defaultLayout, app.windowKinds),
            guestWindowId: guestWindowIdV1(windowId, spawnedIdsRef.current) ?? windowId,
          };
        };
        // 🛑️ A cancel gesture retires the requesting instance's IN-FLIGHT extension work before the
        // gesture itself is forwarded, and never instead of it: the guest still owns its own
        // bookkeeping (its pending table, its progress ledger, its arming latch) and the extension
        // actor still has to be told through its own capability. This hop is only the third one —
        // the host-side door, which nothing inside the per-actor request queue can reach. The action
        // id is not known to this shell: a mounted surface DECLARES it off its own status contract
        // (`declareSurfaceCancelAction`), so the shell stays domain-neutral
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        if (isDeclaredSurfaceCancelAction(action.action)) {
          const aborted = abortExtensionRequestsForActor(`${session.pluginId}:${session.instanceId}`, `cancelled by ${action.action}`);
        }
        const actionOrigin = submittedOrigin ?? captureDialogOrigin(session);
        if (!isCurrentDialogOrigin(actionOrigin)) return refuse("owner-mismatch", "dialog origin is not current");
        const primaryActionOwner = captureEffectOwner(session, actionOrigin, entry.provenance.inputSeq);

        if (action.action === EXPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
          void core.documentTransferRef.current.exportDocumentArchive();
          return applied();
        }
        if (action.action === IMPORT_ARTIFACT_DOCUMENT_ACTION_ID) {
          void core.documentTransferRef.current.importDocumentArchive();
          return applied();
        }
        if (action.action === SAVE_ARTIFACT_FILE_ACTION_ID) {
          void core.documentTransferRef.current.saveNaturalFile();
          return applied();
        }
        if (action.action === OPEN_ARTIFACT_FILE_ACTION_ID) {
          void core.documentTransferRef.current.openNaturalFile();
          return applied();
        }
        // 🎓️ First-run walkthrough (mirrors setActiveUtility below): fully shell-intercepted, resets
        // playback to the first step, never forwarded to the program.
        if (action.action === START_INTRODUCTION_ACTION_ID) {
          dispatch({ type: "SET_INTRODUCTION_STEP", value: 0 });
          return applied();
        }

        // 🎥️ Fully shell-intercepted, mirroring `START_INTRODUCTION_ACTION_ID` above: sandboxes the
        // document and starts tutorial playback from t=0 (real work happens in `startTutorialRef`, wired up
        // by the TutorialOrchestration block further down this component).
        if (action.action === START_TUTORIAL_ACTION_ID) {
          const args = typeof action.args === "object" && action.args != null ? (action.args as { tutorialId?: unknown }) : {};
          if (typeof args.tutorialId === "string") startTutorialRef.current(args.tutorialId);
          return applied();
        }
        if (action.action === RECORD_TUTORIAL_ACTION_ID) {
          toggleTutorialRecordingRef.current();
          return applied();
        }

        // 🎥️ Deviation detection: any action NOT stamped by the tutorial director/seek/converge path while
        // a tutorial is actively playing means the user diverged from the recording — auto-pause and flag
        // `deviated` so pressing Play again converges instead of resuming blindly mid-drift.
        if (tutorialPlayingRef.current && !tutorialDrivenRef.current.active) {
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: true });
        }

        // ⏺️ Recorder tap: annotational-only capture (see `TutorialTracks.events` doc comment) — never
        // re-dispatched on playback. Skips navigation/introduction/tutorial-control actions (noise, or
        // meaningless to replay) and anything the director itself just dispatched.
        if (tutorialRecordingRef.current && !tutorialDrivenRef.current.active) {
          if (!TUTORIAL_RECORDING_EXCLUDED_ACTION_IDS.has(action.action)) {
            tutorialRecorderRef.current?.recordEvent({ kind: "action", action: action.action, args: action.args as Record<string, unknown> | undefined });
          }
        }

        // 🧭️ Camera-navigation gesture report from a 3D window's `WorldOrbitGated` (shell-only, never
        // forwarded to the program) — completes any pan/zoom/orbit interaction of the active step that
        // targets the window the gesture happened on. Celebrates only `windowId`'s own pane (via
        // `windowElementId`, its unique per-instance element id) — never the whole window-kind alias
        // selector, which would celebrate every other open pane of that same kind too (e.g. a split view).
        if (action.action === NOTE_WORLD_NAVIGATION_ACTION_ID) {
          return handleWorldNavigationAction({ extraWindowInstancesRef, completeIntroductionInteraction, session, action, applied });
        }

        // 🧰️ Utility activation (P5): host-owned session state, never a document operation. Re-clicking the active
        // utility (or an empty utilityId) deactivates. We resolve the target window from the descriptor's tagged
        // `windowId` (see `tagSetActiveUtilityWindow`), falling back to the active window, compare-and-set the
        // window's register (`expectedGeneration` is the generation the pressing widget rendered — a press
        // against a register that moved since is refused `stale-generation`, the L3 replacement for the
        // deleted 8 s `lastUtilityArmAtRef` echo-off window), then forward the resolved utility to the plugin
        // so it can clear/prepare scratch.
        if (action.action === SET_ACTIVE_UTILITY_ACTION_ID) {
          return await handleSetActiveUtilityAction({ activeWindowIdRef, activeUtilityByWindowIdRef, writeUtilityRegister, activeToolIdRef, writeToolRegister, completeIntroductionInteraction, loadedPlugins, uiLocaleRef, uiTerminologyRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied });
        }

        // 🛠️ Tool activation: host-owned session state (mode-scoped, windowless), never a document operation.
        // Re-clicking the active tool (or an empty toolId) deactivates. Mutually exclusive with every
        // window's active utility — activating a tool clears them all, mirroring `SET_ACTIVE_UTILITY_ACTION_ID`.
        // 🔢️ Same compare-and-set over the one tool register.
        if (action.action === SET_ACTIVE_TOOL_ACTION_ID) {
          return await handleSetActiveToolAction({ activeToolIdRef, writeToolRegister, clearAllWindowUtilities, completeIntroductionInteraction, activeWindowIdRef, loadedPlugins, uiLocaleRef, uiTerminologyRef, activeUtilityByWindowIdRef, spawnedIdsRef, applyHistoryPatch, applyLeftoverInteractionView, isCurrentEffectOwner, applyHostEffects, action, refuse, programOfWindow, entry, primaryActionOwner, applied });
        }

        completeIntroductionInteraction((interaction) => interaction.on.kind === "action" && interaction.on.id === action.action);

        if (action.controllerId === FRAMEWORK_CHECKIN_CONTROLLER_ID) {
          if (action.action !== "submit") return refuse("undeclared-action", `check-in: ${action.action}`);
          dispatchCheckpointRef.current(checkinSubmitMessageV1(action.args));
          return applied();
        }

        if (action.controllerId === FRAMEWORK_SYNC_CONTROLLER_ID) {
          return handleSyncCardAction({ dispatch, browseSyncBackbonePath, syncCardKind, syncDraftPath, attachSyncTarget, detachSyncBackbone, action, applied, refuse });
        }

        if (hostMode && action.action === "spawnApp" && action.controllerId !== hostControllerId) {
          const pluginId = typeof action.args === "object" && action.args != null && "pluginId" in action.args ? String((action.args as { pluginId?: string }).pluginId ?? "") : "";
          const program = spacePrograms.find((entry) => entry.pluginId === pluginId);
          if (!program) return refuse("undeclared-action", `spawnApp: no program "${pluginId}"`);
          void spawnProgram(program);
          return applied();
        }

        if (hostMode && action.action === "setActivePanelTab" && (action.controllerId === hostControllerId || action.controllerId === session.app.controllerId)) {
          return handleSetActivePanelTabAction({ hostApp, updateSpacePanel, mobile, dispatch, session, action, refuse, applied });
        }

        let { targetSession, hostWindowId, targetSpawnedId } = resolveDispatchTarget({ hostMode, panel, loadedPlugins, activeWindowIdRef, spawnedIdsRef, session, action });
        const undoOutcome = routeRemoteUndo({ inferenceHistoryByRuntimeKeyRef, openDocumentSessionsRef, historyCanUndo, localHistoryOrderRef, verifiedSessionAuthorityRef, ensureBackboneWorker, action, targetSession, refuse, applied });
        if (undoOutcome !== undefined) return undoOutcome;
        // ⏪️ Reserved history verbs act on the DOCUMENT owner, not the app-chrome session the navbar/
        // keybinding dispatch carries — the retained browser actor is keyed to the open document session,
        // so without this remap `directBrowserActorForSession` misses and the dispatch dies on the retired
        // `plugin.handleAction` path (ticket 26/09/02/PUZZLE-3D-END-TO-END).
        if (action.action === "undo" || action.action === "redo") {
          const directNow = (() => { try { return directBrowserActorForSession(targetSession) !== null ? "yes" : "null"; } catch (error) { return `throw:${String(error)}`; } })();
          const owners = [...openDocumentSessionsRef.current.values()].map((entry) => ({ pluginId: entry.session.pluginId, instanceId: entry.session.instanceId }));
          if (directNow === "null") {
            const documentOwners = [...openDocumentSessionsRef.current.values()].filter((entry) => entry.session.pluginId === targetSession.pluginId);
            if (documentOwners.length === 1) {
              targetSession = documentOwners[0]!.session;
            }
          }
        }
        if (requiredOperationOwner !== undefined && !typedOperationCancellationOwnerIsCurrentV1(requiredOperationOwner.session, targetSession, isCurrentEffectOwner(requiredOperationOwner))) {
          return refuse("owner-mismatch", "typed operation owner is no longer current");
        }
        const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === targetSession.pluginId)?.handle;
        if (!plugin) return refuse("dispatch-failed", `no loaded program for "${targetSession.pluginId}"`);
        // 🪦️ A control that outlived its session (a long-lived canvas host's own callback, a queued
        // pointer gesture) still addresses the instance a switch sealed. Dropping here is what turns the
        // twenty `no actor for instance N` stacks the 6018 journey logged after one role chord into one
        // typed line per late dispatch.
        if (dropForSealedInstance(targetSession, "action", action.action)) return refuse("instance-sealed", `${targetSession.pluginId}#${targetSession.instanceId}`);
        const actionOwner = captureEffectOwner(targetSession, actionOrigin, entry.provenance.inputSeq);
        if (!isCurrentEffectOwner(actionOwner)) {
          return refuse("owner-mismatch", "effect owner is not current");
        }

        const { baseDispatchViewState, dispatchWindowId, dispatchExtraInstances } = buildDispatchViewState({ spawnedIdsRef, extraWindowInstancesRef, applyLeftoverInteractionView, uiLocale, uiTerminology, identityRef, activeUtilityByWindowIdRef, activeWindowIdRef, treeWindowSchedulerRef, hostWindowId, targetSpawnedId, targetSession, action });
        const dispatchViewState = hostArmedViewContext(baseDispatchViewState, activeToolIdRef.current, dispatchWindowId);
        if (!dispatchViewState) {
          return refuse("view-state-unresolved", `window=${dispatchWindowId ?? ""}`);
        }
        // 🚨️ Undeclared-action drop — ALWAYS visible, never `[TRACE]`/diagnostics-gated: this is the one
        // place a fully wired binding dies without a fault reaching anyone, so it names the app, the action
        // and the dispatching window kind (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        const undeclared = undeclaredActionDiagnostic(targetSession.app.id, action.action, targetSession.app.windowKinds, (baseDispatchViewState.windowInstances ?? []).find((instance) => instance.id === dispatchWindowId)?.windowKindId ?? null, targetSession.app.actions ?? []);
        if (undeclared && !frameworkOwnsExampleSwitch(targetSession.app.role, action.action)) {
          console.error(undeclared.message, undeclared);
          return refuse("undeclared-action", undeclared.message);
        }
        // 👁️✏️ Client-side half of the read-only guarantee (contract freeze §2.3/§5) — the SDK-side
        // `VcsArtifactApp` guard is the source of truth (a `ArtifactViewer`-declared session can never
        // even construct a `Mutation`-kind action), this just avoids a pointless round trip and shows the
        // same notice a `"viewer.read-only"` fault reply gets in the `.catch` below. `showTransientNotice`/
        // `isViewerReadOnlyFault` are deliberately NOT in this callback's dep list below — both are stable
        // across renders (refs + `dispatch` only), and are declared later in this component, so adding
        // them would read a not-yet-initialized `const` on the render that first creates this callback.
        if (targetSession.app.role === "viewer" && targetSession.app.windowKinds.some((kind) => resolveWindowActions(targetSession.app, kind).some((entry) => entry.id === action.action && entry.kind === "mutation"))) {
          core.showTransientNotice(viewerReadOnlyNoticeText(uiLocale), "info", SURFACE_FAULT_CODES.ViewerReadOnly);
          return refuse("viewer-read-only", undefined, true);
        }

        // 🎨️ EVERY route that loads an example records which one, not only `NavbarExampleSelect`'s own
        // `onValueChange`. `navbarExampleIdFromHistoryUpserts` restores this id when a `Set Active Example`
        // row comes back live, i.e. on REDO — and a row can be redone that this shell never dispatched
        // through the picker (a palette entry, a context-menu row, a replayed shell command, the boot
        // load). Wave B26 measured `navbar example from history {"navbarExample":"concrete-forest",
        // "remembered":""}` on :6013: undo relabelled the picker (a popped row needs no memory, it falls
        // back to the boot example) while redo silently could not, because nothing outside the picker had
        // ever written the id down.
        lastDispatchedExampleIdRef.current = rememberedExampleIdFromDispatchV1(action, lastDispatchedExampleIdRef.current);

        if (action.action === "openImportSnapshot") {
          void requestFileOpen("application/json,.json", "text", false)
            .then(async (opened) => {
              if (!opened[0]) return;
              onAction({ controllerId: action.controllerId, action: "importSnapshot", args: { payload: opened[0].contents, name: opened[0].name }, provenance: { ...entry.provenance, origin: "user", causedBy: entry.provenance.inputSeq } });
            })
            .catch((error) => undefined);
          return applied();
        }
        let directBrowserActor: ReturnType<typeof directBrowserActorForSession>;
        try {
          directBrowserActor = directBrowserActorForSession(targetSession);
        } catch (error) {
          const outcome = refuse("owner-mismatch", `browser actor owner: ${String(error instanceof Error ? error.message : error)}`);
          if (propagateFailure) throw error;
          return outcome;
        }
        const directOutcome = await dispatchDirectBrowserActorAction({ dispatchDirectBrowserActorCommand, uiTerminologyRef, directBrowserActor, targetSession, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, applied, refusalReasonForError, core, refuse, propagateFailure });
        if (directOutcome !== undefined) return directOutcome;
        return await dispatchProgramAction({ sessionWorkRef, isCurrentEffectOwner, applyHistoryPatch, applyLeftoverInteractionView, historyStore, lastDispatchedExampleIdRef, exampleOptionsRef, defaults, dispatch, refreshHistorySnapshot, applyHostEffects, observeLocalInteraction, awaitOperationSettle, uiTerminologyRef, uiLocale, dropForSealedInstance, dropForRetiredInstance, targetSession, plugin, dispatchViewState, action, dispatchExtraInstances, dispatchWindowId, entry, actionOwner, lifecycle, onAction, actionOrigin, hostWindowId, core, applied, refuse, refusalReasonForError, propagateFailure });
      } catch (unexpected) {
        // 🧯️ L1: an input never dangles — a throw anywhere above that no branch mapped still settles the
        // entry (`settle` is idempotent, so an already-settled entry keeps its first outcome).
        const outcome = refuse("dispatch-failed", `unexpected: ${String(unexpected instanceof Error ? unexpected.message : unexpected)}`, false, unexpected instanceof SemioFaultError ? inputDiagnosticV1(unexpected.fault) : undefined, unexpected instanceof SemioFaultError ? historyInputDraftDispositionV1(unexpected.fault) : undefined);
        if (propagateFailure) throw unexpected;
        return ledger.outcome(entry.provenance.inputSeq) ?? outcome;
      }
    },
    [
      applyHostEffects,
      dropForRetiredInstance,
      dropForSealedInstance,
      refreshHistorySnapshot,
      awaitOperationSettle,
      captureDialogOrigin,
      isCurrentDialogOrigin,
      captureEffectOwner,
      directBrowserActorForSession,
      dispatchDirectBrowserActorCommand,
      isCurrentEffectOwner,
      applyHistoryPatch,
      applyLeftoverInteractionView,
      attachSyncTarget,
      browseSyncBackbonePath,
      clearAllWindowUtilities,
      detachSyncBackbone,
      injectActiveUtility,
      loadedPlugins,
      observeLocalInteraction,
      panel,
      session,
      writeUtilityRegister,
      writeToolRegister,
      spawnProgram,
      hostMode,
      syncCardKind,
      syncDraftPath,
      updateSpacePanel,
      hostControllerId,
      hostApp,
      hostCatalogueTabId,
      historyCanUndo,
      ensureBackboneWorker,
      completeIntroductionInteraction,
      primaryPluginId,
      reloadPlugin,
      uninstallPlugin,
      pluginSupervisorById,
      spacePrograms,
      mobile,
      uiLocale,
    ],
  );

  /** 🧭️ Logs a shell-chrome command (theme change, dock drag, window resize, panel toggle, …) into the
   * plugin's session-only command-history panel — routed through the exact same `onAction` funnel as every
   * other action (see `NOTE_SHELL_COMMAND_ACTION_ID`) so it lands on `targetSession.instanceId` via the
   * standard `handleAction` call, just tagged with an id the plugin intercepts before the app sees it.
   * Answers the ledger outcome, so a caller that retires the target instance right after (closing a spawned
   * program's last window) can wait for the note to land first. No-ops when there's no active app session. */
  const noteShellCommand = useCallback(
    (commandId: string, label: ShellLabelTextV1, detail?: Record<string, unknown>): Promise<InputOutcomeV1 | null> => {
      if (!session) return Promise.resolve(null);
      return onAction(buildNoteShellCommandAction(session.app.controllerId, commandId, label, detail));
    },
    [session, onAction],
  );

  const onActionRef = useRef(onAction);
  core.onActionRef = onActionRef;
  useEffect(() => {
    onActionRef.current = onAction;
  }, [onAction]);

  // 🐢️ `onAction`'s own identity churns every action (its deps include `session`, `panel`, …). Render
  // trees built from `UiNode`s only need a *callable* action dispatcher, not a fresh one each time —
  // route them through this permanently-stable ref indirection so `interpretUiNode`'s `React.memo`
  // (and any `useMemo` keyed on the dispatcher passed to it) can actually bail.
  const onActionStable = useCallback((action: Parameters<typeof onAction>[0], lifecycle?: ActionDispatchLifecycleV1): Promise<InputOutcomeV1> => onActionRef.current(action, undefined, false, lifecycle), []);
  const onIntentStable = useCallback((intent: Parameters<typeof uiIntentToActionDescriptor>[0], windowId?: string): Promise<InputOutcomeV1> => {
    const action = uiIntentToActionDescriptor(intent);
    return onActionStable(windowId === undefined ? action : inputActionWithWindowV1(action, windowId));
  }, [onActionStable]);
  const onBrowserActorIntent = useCallback(async (runtimeKey: string, captured: RetainedBrowserActorUiV1, surfaceKey: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]): Promise<InputOutcomeV1> => {
    const action = uiIntentToActionDescriptor(intent);
    const ledger = inputLedgerRef.current;
    const issued = ledger.issue(action, { windowId: surfaceKey, origin: "user" });
    const applied = (commit?: InputCommitReceiptV1): InputOutcomeV1 => {
      const outcome = inputAppliedV1(issued.provenance.inputSeq, commit);
      ledger.settle(outcome);
      return outcome;
    };
    const refused = (reason: InputRefusalReasonV1, detail?: string, diagnostic?: InputDiagnosticV1): InputOutcomeV1 => {
      const outcome = inputRefusedV1(issued.provenance.inputSeq, reason, detail, diagnostic);
      if (ledger.settle(outcome)) console.warn(inputRefusalTextV1(issued.action, outcome as Extract<InputOutcomeV1, { kind: "refused" }>, issued.provenance));
      return outcome;
    };
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    const store = retained?.windows.get(surfaceKey) ?? retained?.panels.get(surfaceKey);
    if (store === undefined || retained?.actions !== captured.actions || retained.identity === null || entry?.clientInstanceId !== captured.clientInstanceId || entry.session.instanceId !== captured.sessionInstanceId || !shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, entry.session)) return refused("owner-mismatch", "browser actor owner retired");
    const current = (): boolean => {
      const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
      const opening = openDocumentSessionsRef.current.get(runtimeKey);
      return live?.actions === captured.actions && live.identity !== null && opening?.clientInstanceId === captured.clientInstanceId && opening.session.instanceId === captured.sessionInstanceId && shellDialogSessionIsCurrentV1(shellStateRef.current.pluginRuntime.session, opening.session);
    };
    try {
      const result = await retained.actions.dispatchIntent({
        scope: retained.scope,
        verifiedSurfaceId: retained.verifiedSurfaceId,
        activationGeneration: retained.activationGeneration,
        appChannelVersion: BROWSER_ACTOR_ACTION_APP_CHANNEL_VERSION,
        instanceId: retained.identity.instanceId,
        surfaceRevision: store.getRevisionSnapshot(),
      }, surfaceKey, intent);
      if (!current()) return refused("owner-mismatch", "browser actor owner retired after completion");
      for (const patch of result.historyPatches) if (current()) applyHistoryPatch(decodePackWire(Uint8Array.from(patch), "$.historyPatch") as HistoryPatch, false, { pluginId: entry.session.pluginId, instanceId: entry.session.instanceId });
      await publishBrowserActorHostEffectsV1(result.hostEffects, current, (effect) => {
        if ("requestServiceOperation" in effect) return requestServiceOperation(entry.session, effect.requestServiceOperation, current);
        else window.open(effect.openExternalUrl.url, "_blank", "noopener,noreferrer");
      });
      if (!current()) return refused("owner-mismatch", "browser actor owner retired during publication");
      return applied(result.commit ?? undefined);
    } catch (error) {
      const reason = browserActorActionRefusalReasonV1(error) ?? "dispatch-failed";
      const detail = String(error instanceof Error ? error.message : error);
      if (browserActorUiByRuntimeKeyRef.current.get(runtimeKey)?.actions !== captured.actions) return refused("owner-mismatch", detail);
      core.showTransientNotice(shellLabel("ui.common.renderError"), "error");
      return refused(reason, detail, error instanceof SemioFaultError ? inputDiagnosticV1(error.fault) : undefined);
    }
  }, [applyHistoryPatch, requestServiceOperation]);

  return { onAction, noteShellCommand, onActionStable, onIntentStable, onBrowserActorIntent };
}
//#endregion FrameworkOsShell
