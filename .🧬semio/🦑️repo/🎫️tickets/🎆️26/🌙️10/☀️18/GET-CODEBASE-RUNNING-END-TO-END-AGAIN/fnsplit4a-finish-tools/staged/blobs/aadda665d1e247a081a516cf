/** 🔶️ The core phase of FrameworkOsShellInner. */
import { useShellCoreState, type FrameworkOsShellInnerProps } from "../🧩️shell-core-state/🟦️.ts";
import { useIdentityAndDocumentPorts } from "../🔒️identity-document-ports/🟦️.ts";
import { useBackboneWorker } from "../🧵️backbone-worker/🟦️.ts";
import { useDirectoryAndRoute } from "../🛂️directory-route/🟦️.ts";
import { usePluginLifecycle } from "../🚀️plugin-lifecycle/🟦️.ts";
import { useFocusAndInput } from "../🎮️focus-and-input/🟦️.tsx";
import { useContributionsAndRefresh } from "../🔁️contributions-refresh/🟦️.ts";
import { useSpawnedSession } from "../🛫️spawned-session/🟦️.ts";
import { useApplyHostEffects } from "../⚡️host-effects/🟦️.ts";
import { useShellUriAndDocuments } from "../🗄️shell-uri-documents/🟦️.ts";
import { useShellActions } from "../🖱️shell-actions/🟦️.ts";
import { useTutorialOrchestration } from "../🎥️tutorial-orchestration/🟦️.ts";
import { useDirectoryLaneAndLayout } from "../📐️layout-hotkeys/🟦️.ts";
import { useArtifactOpening } from "../📬️artifact-opening/🟦️.ts";
import { useThemeDriverSettings } from "../🎨️theme-driver-settings/🟦️.tsx";
import { useDocumentTransferAndTabs } from "../📤️document-transfer-tabs/🟦️.tsx";
import { useCheckInAndSync } from "../✅️check-in-sync/🟦️.tsx";
import { useCommandBar } from "../🔎️command-bar/🟦️.tsx";
import { useDockAssembly } from "../🏗️dock-assembly/🟦️.ts";
import { usePanelHosting } from "../📋️panel-hosting/🟦️.tsx";
import { useCanvasAndFooter } from "../🖼️canvas-footer/🟦️.tsx";
import type { ShellCore } from "../🔸️shared/🟦️.tsx";

/** 🔷 Builds core, documentServices, brand, shellContextMenuTitleLabel of FrameworkOsShellInner. */
export function useCoreState({ props }: { readonly props: FrameworkOsShellInnerProps }) {
  const core = {} as ShellCore;
  Object.assign(core, useShellCoreState(props, core));
  Object.assign(core, useIdentityAndDocumentPorts(core));
  Object.assign(core, useBackboneWorker(core));
  Object.assign(core, useDirectoryAndRoute(core));
  Object.assign(core, usePluginLifecycle(core));
  Object.assign(core, useFocusAndInput(core));
  Object.assign(core, useContributionsAndRefresh(core));
  Object.assign(core, useSpawnedSession(core));
  Object.assign(core, useApplyHostEffects(core));
  Object.assign(core, useShellUriAndDocuments(core));
  Object.assign(core, useShellActions(core));
  Object.assign(core, useTutorialOrchestration(core));
  Object.assign(core, useDirectoryLaneAndLayout(core));
  Object.assign(core, useArtifactOpening(core));
  Object.assign(core, useThemeDriverSettings(core));
  Object.assign(core, useDocumentTransferAndTabs(core));
  Object.assign(core, useCheckInAndSync(core));
  Object.assign(core, useCommandBar(core));
  Object.assign(core, useDockAssembly(core));
  Object.assign(core, usePanelHosting(core));
  Object.assign(core, useCanvasAndFooter(core));

  const {
    documentServices, brand, shellContextMenuTitleLabel, hostMode, mobile, dispatch, localInteraction, loadedPlugins,
    session, error, localPrograms, uiLocale, uiTerminology, appLabelsOverlay, appCatalogue, searchOpen, findOpen,
    introductionStepIndex, introductionCompletedInteractions, overlayDialog, transientNotice, tutorialPlaying,
    tutorialRate, tutorialMuted, tutorialCaptionsOn, tutorialRecording, inferencePortRuntimeKey, inferencePort,
    shellContextMenu, setShellContextMenu, setWindowTitle, setWindowIcon, openSpaceId, agentBridgeInstances,
    identityOffline, verifiedSessionAuthority, hubEnv, hubSessionCapability, directoryBootstrapUi,
    spaceArtifactCreationUi, spaceArtifactCreationCatalogUi, cancelSpaceArtifactCreation, spaceAdministration,
    hubWorkspaceOpen, setHubWorkspaceOpen, hubConnectionPort, isCurrentDialogOrigin, closeOwnedDialog,
    bootstrapUiByDocument, executionTargetUiByDocument, cancelDirectoryBootstrap, dispatchSpaceAdministrationIntent,
    dispatchInferencePortIntent, cancelSessionAuthorityBootstrap, navigateHistory, sessionRouteRef, navigateShellUri,
    shellPluginCanvasStatus, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls,
    commandStallClockMs, panel, boardSessionFactory, focusedApp, focusedTimeTravel, focusedReprojection,
    activeIntroduction, tutorialRecorderAvailable, dismissIntroduction, requestContextMenu, closeDocument,
    forgetLocalFolder, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, onAction, onActionStable,
    activeTutorial, tutorialClock, seekTutorial, playPauseTutorial, stopTutorial, toggleTutorialRecording,
    addTutorialChapter, tutorialChapterMarkers, openReadySpaceArtifactCreation, keysByActionId, controlKeybindings,
    currentDocumentRuntimeKey, selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, rowActionRefusals,
    timeTravelPeers, openHubWorkspace, routeAdmission, routeAdmissionNotice, hubOnlineUserIds, cancelHubCheckIn,
    dismissHubCheckIn, currentCheckIn, historyDomRef, onCommand, dock, handleTabDockDrop, handleTreeUnitDockDrop,
    mobilePanel, initialExampleReady, navbarItems, searchItems, canvas, footerItems, buildPanelProps,
    buildShellContextMenuItems
  } = core;
  return { boardSessionFactory, setWindowTitle, setWindowIcon, keysByActionId, appCatalogue, controlKeybindings, timeTravelPeers, rowActionRefusals, requestContextMenu, buildShellContextMenuItems, session, initialExampleReady, error, agentBridgeInstances, historyDomRef, hubEnv, hubSessionCapability, verifiedSessionAuthority, identityOffline, uiLocale, cancelSessionAuthorityBootstrap, bootstrapUiByDocument, closeDocument, executionTargetUiByDocument, spaceArtifactCreationUi, spaceArtifactCreationCatalogUi, cancelSpaceArtifactCreation, openReadySpaceArtifactCreation, inferencePortRuntimeKey, inferencePort, documentServices, dispatchInferencePortIntent, directoryBootstrapUi, cancelDirectoryBootstrap, hubWorkspaceOpen, hubConnectionPort, openSpaceId, hubOnlineUserIds, navigateShellUri, setHubWorkspaceOpen, hostMode, navigateHistory, sessionRouteRef, spaceAdministration, dispatchSpaceAdministrationIntent, routeAdmissionNotice, routeAdmission, openHubWorkspace, transientNotice, dispatch, installingPluginIds, pluginInstallProgress, cancelPluginInstalls, commandStalls, loadedPlugins, panel, uiTerminology, commandStallClockMs, dock, handleTabDockDrop, handleTreeUnitDockDrop, mobile, mobilePanel, navbarItems, onCommand, activeTutorial, tutorialPlaying, tutorialRate, tutorialMuted, tutorialCaptionsOn, tutorialRecording, tutorialRecorderAvailable, tutorialChapterMarkers, tutorialClock, playPauseTutorial, stopTutorial, seekTutorial, toggleTutorialRecording, addTutorialChapter, footerItems, focusedTimeTravel, focusedReprojection, currentCheckIn, folderReconnectOffer, folderReconnectBusy, reconnectLocalFolder, forgetLocalFolder, focusedApp, onActionStable, currentDocumentRuntimeKey, dismissHubCheckIn, cancelHubCheckIn, buildPanelProps, shellPluginCanvasStatus, canvas, searchItems, searchOpen, findOpen, shellContextMenuTitleLabel, shellContextMenu, setShellContextMenu, activeIntroduction, introductionStepIndex, brand, appLabelsOverlay, introductionCompletedInteractions, dismissIntroduction, overlayDialog, isCurrentDialogOrigin, localPrograms, selectedSpaceArtifactKinds, selectedSpaceArtifactCreationCatalog, localInteraction, closeOwnedDialog, onAction };
}
