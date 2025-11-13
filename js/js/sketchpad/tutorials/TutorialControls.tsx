// #region Header

// TutorialControls.tsx

// 2025 Ueli Saluz

// #endregion

import { CloseIcon, PauseIcon, PlayIcon, SkipBackIcon, SkipForwardIcon, StopIcon } from "@semio/assets";
import { FC, useEffect } from "react";
import { useAddFooterItem, useMode, useRemoveFooterItem } from "../App";
import { Button, Slider } from "../elements";
import { Mode } from "../sketchpad";
import { TutorialPlaybackState, useActiveTutorial, useCurrentMilestone, useIsTutorialActive, usePlaybackState, useTutorialProgress, useTutorialStore } from "./store";

export const TutorialControls: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const isTutorialActive = useIsTutorialActive();

  useEffect(() => {
    if (isTutorialActive) {
      addFooterItem({
        id: "tutorial-controls",
        content: <TutorialControlsContent />,
        order: 100,
      });
    } else {
      removeFooterItem("tutorial-controls");
    }
    return () => removeFooterItem("tutorial-controls");
  }, [isTutorialActive, addFooterItem, removeFooterItem]);

  return null;
};

const TutorialControlsContent: FC = () => {
  const store = useTutorialStore();
  const tutorial = useActiveTutorial();
  const playbackState = usePlaybackState();
  const progress = useTutorialProgress();
  const currentMilestone = useCurrentMilestone();

  if (!tutorial) return null;

  const isPlaying = playbackState === TutorialPlaybackState.PLAYING;
  const isPaused = playbackState === TutorialPlaybackState.PAUSED;
  const isCompleted = playbackState === TutorialPlaybackState.COMPLETED;

  return (
    <div className="flex items-center gap-single px-2">
      <Button variant="ghost" onClick={() => store.stopTutorial()} className="size-tiny p-0">
        <CloseIcon className="size-tiny" />
      </Button>
      <div className="flex items-center gap-single">
        <Button variant="ghost" onClick={() => store.previousMilestone()} disabled={progress.current === 0} className="size-tiny p-0">
          <SkipBackIcon className="size-tiny" />
        </Button>
        <Button
          variant="ghost"
          onClick={() => {
            if (isPlaying) {
              store.pauseTutorial();
            } else if (isPaused) {
              store.resumeTutorial();
            } else if (isCompleted) {
              store.startTutorial(tutorial);
            }
          }}
          className="size-tiny p-0"
        >
          {isPlaying ? <PauseIcon className="size-tiny" /> : <PlayIcon className="size-tiny" />}
        </Button>
        <Button variant="ghost" onClick={() => store.nextMilestone()} disabled={progress.current >= progress.total - 1} className="size-tiny p-0">
          <SkipForwardIcon className="size-tiny" />
        </Button>
      </div>
      <div className="flex-1 min-w-[200px] max-w-[400px]">
        <div className="flex items-center gap-single">
          <Slider id="tutorial-progress" value={[progress.current]} min={0} max={Math.max(progress.total - 1, 0)} step={1} onValueChange={(value) => store.goToMilestone(value[0])} className="flex-1" />
          <span className="text-xs tabular-nums">
            {progress.current + 1}/{progress.total}
          </span>
        </div>
        {currentMilestone && <div className="text-xs truncate text-muted mt-1">{currentMilestone.title}</div>}
      </div>
      <div className="text-xs text-muted">{tutorial.name}</div>
    </div>
  );
};

export const RecordingControls: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const store = useTutorialStore();
  const mode = useMode();

  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("recording-controls");
      return () => {};
    }
    const state = store.snapshot();
    if (state.recordingState !== "idle") {
      addFooterItem({
        id: "recording-controls",
        content: <RecordingControlsContent />,
        order: 1,
      });
    } else {
      removeFooterItem("recording-controls");
    }
    const unsubscribe = store.subscribe(() => {
      const newState = store.snapshot();
      if (newState.recordingState !== "idle") {
        addFooterItem({
          id: "recording-controls",
          content: <RecordingControlsContent />,
          order: 1,
        });
      } else {
        removeFooterItem("recording-controls");
      }
    });
    return () => {
      unsubscribe();
      removeFooterItem("recording-controls");
    };
  }, [store, addFooterItem, removeFooterItem, mode]);

  return null;
};

const RecordingControlsContent: FC = () => {
  const store = useTutorialStore();
  const state = store.snapshot();
  const isRecording = state.recordingState === "recording";
  const isPaused = state.recordingState === "paused";

  const handleStop = () => {
    const recording = store.stopRecording();
    if (recording) {
      store.downloadRecording(recording);
    }
  };

  return (
    <div className="flex items-center gap-single px-2">
      <div className="flex items-center gap-single">
        <div className={`size-dot rounded-full ${isRecording ? "bg-red-500 animate-pulse" : "bg-gray-500"}`} />
        <span className="text-xs">REC</span>
      </div>
      <Button
        variant="ghost"
        onClick={() => {
          if (isRecording) {
            store.pauseRecording();
          } else if (isPaused) {
            store.resumeRecording();
          }
        }}
        className="size-tiny p-0"
      >
        {isRecording ? <PauseIcon className="size-tiny" /> : <PlayIcon className="size-tiny" />}
      </Button>
      <Button variant="ghost" onClick={handleStop} className="size-tiny p-0">
        <StopIcon className="size-tiny" />
      </Button>
      {state.activeRecording && <div className="text-xs text-muted">{state.activeRecording.name}</div>}
    </div>
  );
};
