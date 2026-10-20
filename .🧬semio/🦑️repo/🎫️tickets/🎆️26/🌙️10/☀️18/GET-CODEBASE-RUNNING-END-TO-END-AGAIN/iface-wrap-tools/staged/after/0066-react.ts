/** 🔶️ The scope phase of useTutorialOrchestration. */
import { OwnedTutorialRunV1, type TutorialInverseV1 } from "../../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import { useCallback, useEffect, useMemo, useRef } from "../../../⚛️react-port/🟦️.ts";
import { type LocalInteractionState } from "@semio-tech/framework/replication";
import { createTutorialClock, getTutorialCameraDriver, type TutorialClock } from "@semio-tech/ui-react";
import { applyTutorialUiSnapshotToShell, type TutorialUiBridgeContext } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { tutorialInteractionSelectionActions } from "../../🎬️tutorial-recorder/🟦️.tsx";
import type { ShellCoreAfterShellActions } from "../../🖱️shell-actions/🟦️.ts";

/** 🔷 Builds scope, shellState, dispatch, localInteraction of useTutorialOrchestration. */
export function useScopeState({ core }: { readonly core: ShellCoreAfterShellActions }) {
  const {
    scope, shellState, dispatch, localInteraction, loadedPlugins, session, uiLocale, uiTerminology, appLabelsOverlay,
    activeTutorialId, tutorialPlaying, tutorialRate, tutorialRecording, tutorialDeviated, extraWindowInstancesRef,
    sessionRef, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, documentArchivePorts, historyStore, makeOwnedDialog,
    activeTutorials, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialDrivenRef,
    tutorialPlayingRef, tutorialRecorderRef, tutorialRecorderStartingRef, shellStateRef, publishLocalInteraction,
    refreshUi, onActionRef
  } = core;
  //#region FrameworkOsShell
  //#region 🎥️TutorialOrchestration
  /** ⏱️ Real-time throttle for the director's UI/document/event application (~10Hz) — camera stays
   * smooth every clock tick regardless (see the `subscribe` callback below). */
  const TUTORIAL_DIRECTOR_TICK_MS = 90;

  const activeTutorial = useMemo(() => activeTutorials.find((tutorial) => tutorial.id === activeTutorialId) ?? null, [activeTutorials, activeTutorialId]);

  const tutorialClockRef = useRef<TutorialClock | null>(null);
  core.tutorialClockRef = tutorialClockRef;
  if (!tutorialClockRef.current) tutorialClockRef.current = createTutorialClock(activeTutorial?.durationMs ?? 0);
  const tutorialClock = tutorialClockRef.current;
  useEffect(() => () => {
    const run = tutorialRunRef.current;
    tutorialRunRef.current = null;
    tutorialTransitionEpochRef.current += 1;
    tutorialDrivenRef.current.retire();
    void run?.stop().catch((error) => undefined);
    tutorialClockRef.current?.dispose();
  }, []);
  useEffect(() => {
    tutorialClock.setDurationMs(activeTutorial?.durationMs ?? 0);
  }, [activeTutorial?.durationMs, tutorialClock]);
  useEffect(() => {
    tutorialClock.setRate(tutorialRate);
  }, [tutorialRate, tutorialClock]);
  useEffect(() => {
    if (tutorialPlaying && !tutorialDrivenRef.current.busy) tutorialClock.play();
    else tutorialClock.pause();
  }, [tutorialPlaying, tutorialClock]);

  const publishTutorialInteractionSelection = useCallback((selection: LocalInteractionState["selection"]) => {
    const active = sessionRef.current;
    if (!active || !tutorialRunRef.current?.ready) return;
    for (const action of tutorialInteractionSelectionActions(active.app.controllerId, selection)) onActionRef.current(action);
  }, []);
  const tutorialRunRef = useRef<OwnedTutorialRunV1<TutorialInverseV1> | null>(null);
  core.tutorialRunRef = tutorialRunRef;
  const tutorialTransitionEpochRef = useRef(0);
  core.tutorialTransitionEpochRef = tutorialTransitionEpochRef;
  const tutorialInitializingRunRef = useRef<OwnedTutorialRunV1<TutorialInverseV1> | null>(null);
  const uiBridgeCtxRef = useRef<TutorialUiBridgeContext>({
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => localInteraction.get().selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  });
  uiBridgeCtxRef.current = {
    session,
    restoreDialog: (dialogId, seedArgs) => {
      const run = tutorialRunRef.current;
      return run?.ready ? makeOwnedDialog(dialogId, run.origin, seedArgs) : null;
    },
    appLabelsOverlay,
    terminology: uiTerminology,
    locale: uiLocale,
    interactionSelection: () => localInteraction.get().selection,
    publishInteractionSelection: publishTutorialInteractionSelection,
  };

  /** ⏱️ Playhead (ms) the director/seek last applied document/UI tracks up to — the "from" side of the
   * next `tutorialSlice(def, from, to)` call. Reset to 0 on sandbox (re)start. */
  const tutorialLastAppliedMsRef = useRef(0);
  useEffect(() => {
    const run = tutorialRunRef.current;
    if (run === null) {
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    if (!run.isCurrent() || activeTutorialId !== run.tutorialId || session === null) {
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      if (activeTutorialId !== null) dispatch({ type: "SET_TUTORIAL", value: null });
      void run.stop().catch((error) => undefined).finally(() => {
        if (tutorialRunRef.current !== run) return;
        tutorialRunRef.current = null;
      });
      return;
    }
    if (tutorialInitializingRunRef.current === run) return;
    const def = activeTutorials.find((tutorial) => tutorial.id === run.tutorialId);
    if (def === undefined) {
      tutorialDrivenRef.current.retire();
      void run.stop().catch((error) => undefined);
      dispatch({ type: "SET_TUTORIAL", value: null });
      return;
    }
    tutorialInitializingRunRef.current = run;
    const driveToken = tutorialDrivenRef.current.claim();
    void (async () => {
      if (!await run.start() || tutorialRunRef.current !== run || !run.isCurrent() || !tutorialDrivenRef.current.accepts(driveToken)) return;
      if (def.base.exampleId) dispatch({ type: "SET_ACTIVE_EXAMPLE_ID", value: def.base.exampleId });
      applyTutorialUiSnapshotToShell(dispatch, def.base.ui, uiBridgeCtxRef.current);
      for (const cameraKeyframe of def.base.cameras) getTutorialCameraDriver(cameraKeyframe.windowId)?.set(cameraKeyframe.camera);
      tutorialLastAppliedMsRef.current = 0;
      tutorialClock.seek(0);
      await refreshUi(session, { kind: "full" });
    })().catch((error) => {
      if (tutorialRunRef.current === run) dispatch({ type: "SET_TUTORIAL", value: null });
    }).finally(() => {
      tutorialDrivenRef.current.release(driveToken);
    });
  }, [activeTutorialId, activeTutorials, session, loadedPlugins, tutorialClock, refreshUi]);
  return { tutorialRunRef, tutorialDrivenRef, captureDialogOrigin, dispatch, uiBridgeCtxRef, loadedPlugins, onActionRef, historyStore, scope, refreshUi, activeTutorial, session, tutorialClock, TUTORIAL_DIRECTOR_TICK_MS, tutorialLastAppliedMsRef, tutorialPlayingRef, tutorialPlaying, tutorialDeviated, activeTutorials, captureEffectOwner, isCurrentEffectOwner, tutorialTransitionEpochRef, documentArchivePorts, tutorialInitializingRunRef, tutorialRecorderRef, tutorialRecorderStartingRef, publishLocalInteraction, sessionRef, shellStateRef, localInteraction, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialRecording, shellState, extraWindowInstancesRef, uiTerminology, uiLocale };
}
