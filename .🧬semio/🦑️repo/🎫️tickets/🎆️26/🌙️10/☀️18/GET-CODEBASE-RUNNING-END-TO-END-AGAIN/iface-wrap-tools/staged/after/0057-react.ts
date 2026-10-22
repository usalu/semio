/** 🟠️ The play pause tutorial phase of useTutorialOrchestration. */
import { OwnedTutorialRunV1, type TutorialInverseV1 } from "../../🗨️dialog-origin/🎥️tutorial/🟦️.ts";
import { useCallback, useEffect } from "../../../⚛️react-port/🟦️.ts";
import { TUTORIAL_CONVERGE_MS, type TutorialCameraState } from "@semio-tech/framework";
import { encodeMutationEnvelopesPack } from "@semio-tech/framework/packs";
import { composeTutorialUi, getTutorialCameraDriver, interpolateTutorialCamera, tutorialCameraAt, validateTutorial } from "@semio-tech/ui-react";
import { applyTutorialUiSnapshotToShell, captureTutorialUiSnapshot } from "../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { downloadMediaExport } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { TutorialRecorder } from "../../🎬️tutorial-recorder/🟦️.tsx";
import type { useScopeState } from "../🔶️scope/🟦️.ts";

/** 🟡 Builds playPauseTutorial, startTutorial, stopTutorial, toggleTutorialRecording of useTutorialOrchestration. */
export function usePlayPauseTutorialState({ activeTutorial, tutorialRunRef, tutorialDrivenRef, dispatch, tutorialPlaying, tutorialDeviated, session, tutorialClock, uiBridgeCtxRef, activeTutorials, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, tutorialTransitionEpochRef, onActionRef, documentArchivePorts, tutorialInitializingRunRef, tutorialRecorderRef, tutorialRecorderStartingRef, loadedPlugins, publishLocalInteraction, sessionRef, shellStateRef, localInteraction, startTutorialRef, stopTutorialRef, toggleTutorialRecordingRef, tutorialRecording, shellState }: Pick<ReturnType<typeof useScopeState>, "activeTutorial" | "tutorialRunRef" | "tutorialDrivenRef" | "dispatch" | "tutorialPlaying" | "tutorialDeviated" | "session" | "tutorialClock" | "uiBridgeCtxRef" | "activeTutorials" | "captureDialogOrigin" | "captureEffectOwner" | "isCurrentEffectOwner" | "tutorialTransitionEpochRef" | "onActionRef" | "documentArchivePorts" | "tutorialInitializingRunRef" | "tutorialRecorderRef" | "tutorialRecorderStartingRef" | "loadedPlugins" | "publishLocalInteraction" | "sessionRef" | "shellStateRef" | "localInteraction" | "startTutorialRef" | "stopTutorialRef" | "toggleTutorialRecordingRef" | "tutorialRecording" | "shellState">) {
  /** ▶️ Play/pause toggle — the deviation-converge path (design point 6): snaps document+UI to the
   * composed target at the current playhead, tweens the camera over `TUTORIAL_CONVERGE_MS` (real-time,
   * rate-independent) from each window's LIVE pose to its target pose, then resumes the clock. */
  const playPauseTutorial = useCallback(() => {
    if (!activeTutorial) return;
    const run = tutorialRunRef.current;
    if (run === null || run.tutorialId !== activeTutorial.id || !run.ready) return;
    if (tutorialDrivenRef.current.busy) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: !tutorialPlaying });
      return;
    }
    if (tutorialPlaying) {
      dispatch({ type: "SET_TUTORIAL_PLAYING", value: false });
      return;
    }
    if (tutorialDeviated && session) {
      const def = activeTutorial;
      const atMs = tutorialClock.getTimeMs();
      const driveToken = tutorialDrivenRef.current.claim();
      applyTutorialUiSnapshotToShell(dispatch, composeTutorialUi(def, atMs), uiBridgeCtxRef.current);
      const cameraWindowIds = new Set([...def.base.cameras, ...def.tracks.camera].map((keyframe) => keyframe.windowId));
      const startPoseByWindow = new Map<string, TutorialCameraState>();
      for (const windowId of cameraWindowIds) {
        const live = getTutorialCameraDriver(windowId)?.get();
        if (live) startPoseByWindow.set(windowId, live);
      }
      const startedAt = performance.now();
      const tween = (now: number) => {
        if (tutorialRunRef.current !== run || !run.ready || !tutorialDrivenRef.current.accepts(driveToken)) {
          tutorialDrivenRef.current.release(driveToken);
          return;
        }
        const progress = Math.min(1, (now - startedAt) / TUTORIAL_CONVERGE_MS);
        for (const windowId of cameraWindowIds) {
          const targetPose = tutorialCameraAt(def, windowId, atMs);
          if (!targetPose) continue;
          const driver = getTutorialCameraDriver(windowId);
          if (!driver) continue;
          const startPose = startPoseByWindow.get(windowId);
          if (startPose && startPose.kind === targetPose.kind) {
            driver.set(interpolateTutorialCamera({ at: 0, windowId, camera: startPose, easing: "linear" }, { at: TUTORIAL_CONVERGE_MS, windowId, camera: targetPose, easing: "linear" }, progress * TUTORIAL_CONVERGE_MS));
          } else {
            driver.set(targetPose);
          }
        }
        if (progress < 1) requestAnimationFrame(tween);
        else {
          tutorialDrivenRef.current.release(driveToken);
          dispatch({ type: "SET_TUTORIAL_DEVIATED", value: false });
          dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
        }
      };
      requestAnimationFrame(tween);
      return;
    }
    dispatch({ type: "SET_TUTORIAL_PLAYING", value: true });
  }, [activeTutorial, tutorialPlaying, tutorialDeviated, session, tutorialClock]);

  const startTutorial = useCallback(
    (tutorialId: string) => {
      if (!session || !activeTutorials.some((tutorial) => tutorial.id === tutorialId)) return;
      const origin = captureDialogOrigin(session);
      const owner = captureEffectOwner(session, origin);
      if (origin === null || !isCurrentEffectOwner(owner) || owner.plugin === null) return;
      const plugin = owner.plugin;
      const epoch = ++tutorialTransitionEpochRef.current;
      tutorialClock.pause();
      tutorialDrivenRef.current.retire();
      dispatch({ type: "SET_TUTORIAL", value: null });
      void (async () => {
        const previous = tutorialRunRef.current;
        await previous?.stop().catch((error) => undefined);
        if (tutorialRunRef.current === previous) tutorialRunRef.current = null;
        if (epoch !== tutorialTransitionEpochRef.current || !isCurrentEffectOwner(owner)) return;
        const run: OwnedTutorialRunV1<TutorialInverseV1> = new OwnedTutorialRunV1(tutorialId, origin, (): boolean => tutorialRunRef.current === run && isCurrentEffectOwner(owner), {
          drain: () => tutorialDrivenRef.current.drain(),
          revert: async (inverse) => {
            if (!isCurrentEffectOwner(owner)) return;
            if (inverse.kind === "mutations") {
              if (plugin.applyMutations) await plugin.applyMutations(session.instanceId, encodeMutationEnvelopesPack(inverse.envelopes));
            } else if (inverse.kind === "history") await onActionRef.current({ controllerId: session.app.controllerId, action: inverse.action, ...(inverse.args === undefined ? {} : { args: inverse.args }) }, run.origin, true);
          },
          settle: async (unreverted) => {
            if (!isCurrentEffectOwner(owner)) return;
            for (const reason of unreverted) console.warn(`[tutorial] left in place: ${reason}`);
            const ports = documentArchivePorts(plugin, session, () => isCurrentEffectOwner(owner));
            await ports.history();
            await ports.refresh();
          },
        });
        tutorialRunRef.current = run;
        tutorialInitializingRunRef.current = null;
        dispatch({ type: "SET_TUTORIAL", value: tutorialId });
      })().catch((error) => undefined);
    },
    [activeTutorials, session, captureDialogOrigin, captureEffectOwner, isCurrentEffectOwner, documentArchivePorts, tutorialClock],
  );
  const stopTutorial = useCallback(() => {
    tutorialTransitionEpochRef.current += 1;
    tutorialClock.pause();
    tutorialDrivenRef.current.retire();
    void tutorialRunRef.current?.stop().catch((error) => undefined);
    dispatch({ type: "SET_TUTORIAL", value: null });
  }, [tutorialClock]);

  /** ⏺️ Arms/disarms `TutorialRecorder` against the LIVE (never sandboxed) document — a recording IS the
   * user's work. On stop: light `validateTutorial` sanity check, then serialize + trigger a browser
   * download, matching the repo's existing media-export download pattern. */
  const toggleTutorialRecording = useCallback(() => {
    if (!session) return;
    const recorder = tutorialRecorderRef.current;
    if (recorder) {
      tutorialRecorderRef.current = null;
      const id = `recorded-${session.app.id}-${Date.now()}`;
      const def = recorder.build(id, `${session.app.id} recording`);
      const validationError = validateTutorial(def);
      const json = JSON.stringify(def, null, 2);
      downloadMediaExport(`tutorial-${session.app.id}-${Date.now()}.ops`, "text/plain", json);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: false });
      return;
    }
    if (tutorialRecorderStartingRef.current) return;
    const plugin = loadedPlugins.find((entry) => entry.handle.pluginId === session.pluginId)?.handle;
    if (!plugin) return;
    tutorialRecorderStartingRef.current = true;
    void publishLocalInteraction(plugin, session.instanceId).then((interaction) => {
      const active = sessionRef.current;
      if (!interaction || !active || active.pluginId !== session.pluginId || active.instanceId !== session.instanceId) return;
      tutorialRecorderRef.current = new TutorialRecorder(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), active), null);
      dispatch({ type: "SET_TUTORIAL_RECORDING", value: true });
    }).catch((captureError) => undefined).finally(() => {
      tutorialRecorderStartingRef.current = false;
    });
  }, [loadedPlugins, publishLocalInteraction, session]);

  useEffect(() => {
    startTutorialRef.current = startTutorial;
    stopTutorialRef.current = stopTutorial;
    toggleTutorialRecordingRef.current = toggleTutorialRecording;
  }, [startTutorial, stopTutorial, toggleTutorialRecording]);

  useEffect(() => {
    if (!tutorialRecording) return;
    const record = (): void => {
      tutorialRecorderRef.current?.recordUiDiff(captureTutorialUiSnapshot(shellState, localInteraction.get(), session));
    };
    record();
    return localInteraction.subscribe(record);
  }, [tutorialRecording, shellState, session, localInteraction]);

  useEffect(() => {
    if (!tutorialRecording || !session || typeof window === "undefined") return;
    const interval = window.setInterval(() => {
      tutorialRecorderRef.current?.recordSnapshot(captureTutorialUiSnapshot(shellStateRef.current, localInteraction.get(), session));
    }, 5000);
    return () => window.clearInterval(interval);
  }, [tutorialRecording, session]);
  return { playPauseTutorial, stopTutorial, toggleTutorialRecording };
}
