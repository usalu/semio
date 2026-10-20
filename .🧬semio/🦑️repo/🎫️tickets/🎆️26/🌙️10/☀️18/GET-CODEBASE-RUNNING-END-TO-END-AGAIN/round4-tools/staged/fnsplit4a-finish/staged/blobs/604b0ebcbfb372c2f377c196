import type { ShellCoreAfterDocumentTransferAndTabs } from "../📤️document-transfer-tabs/🟦️.tsx";
import { useHostModeState } from "./🔶️host-mode/🟦️.tsx";
import { useAutoCheckinSchedulerState } from "./🔸️auto-checkin-scheduler/🟦️.tsx";

export type ShellCoreAfterCheckInAndSync = ShellCoreAfterDocumentTransferAndTabs & ReturnType<typeof useCheckInAndSync>;
export function useCheckInAndSync(core: ShellCoreAfterDocumentTransferAndTabs) {
  const { previousCheckpointIdRef, historyCheckpointId, checkpointDispatchedRef, currentDocumentRuntimeKey, requestHubCheckIn, openSpaceIdRef, currentDocumentId, touchSpaceIndexArtifact, isEditorSession, dispatchCheckpoint, focusedHistoryV1, historyStore, session, dispatchCheckpointRef, uiTerminology, uiLocale, focusedApp, focusedPanelBodyStores, onAction, appLabelsOverlay, treeWindowHost, panelTreeConfigCacheRef, focusedBrowserActorPanels, treeWindowGeneration, shellState, syncBackboneUri, syncStatusByDocumentId, syncPillState, syncCardKind, syncDraftPath, dispatch, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath, selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, artifactCreationChoiceRevisionRef, presencePeers, rowActionRefusals, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace, routeAdmission, routeAdmissionNotice, hubOnlineUserIds, cancelHubCheckIn, dismissHubCheckIn, currentCheckIn } = useHostModeState({ core });
  const { historyDomRef, frameworkHistoryTab, frameworkSyncTab } = useAutoCheckinSchedulerState({ previousCheckpointIdRef, historyCheckpointId, checkpointDispatchedRef, currentDocumentRuntimeKey, requestHubCheckIn, openSpaceIdRef, currentDocumentId, touchSpaceIndexArtifact, isEditorSession, dispatchCheckpoint, focusedHistoryV1, historyStore, session, dispatchCheckpointRef, uiTerminology, uiLocale, focusedApp, focusedPanelBodyStores, onAction, appLabelsOverlay, treeWindowHost, panelTreeConfigCacheRef, focusedBrowserActorPanels, treeWindowGeneration, shellState, syncBackboneUri, syncStatusByDocumentId, syncPillState, syncCardKind, syncDraftPath, dispatch, attachSyncTarget, detachSyncBackbone, browseSyncBackbonePath });
  return {
    selectedSpaceArtifactCreationCatalog, selectedSpaceArtifactKinds, artifactCreationChoiceRevisionRef, presencePeers,
    rowActionRefusals, timeTravelPeers, hubConnectionStatuses, hubSessionPresence, hubLink, openHubWorkspace,
    routeAdmission, routeAdmissionNotice, hubOnlineUserIds, cancelHubCheckIn, dismissHubCheckIn, currentCheckIn,
    historyDomRef, frameworkHistoryTab, frameworkSyncTab
  };
}
