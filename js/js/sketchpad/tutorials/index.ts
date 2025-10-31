// #region Header

// index.ts

// 2025 Ueli Saluz

// #endregion

export { commands as tutorialCommands } from "./commands";
export { RecordButton } from "./RecordButton";
export { RecordingControls, TutorialControls } from "./TutorialControls";
export { TutorialOverlay } from "./TutorialOverlay";
export {
  TutorialPlaybackState,
  TutorialProvider,
  TutorialRecordingState,
  TutorialStore,
  useActiveRecording,
  useActiveTutorial,
  useAvailableTutorials,
  useCurrentMilestone,
  useIsRecording,
  useIsTutorialActive,
  usePlaybackState,
  usePlaybackTime,
  useRecordingState,
  useTutorialCommandInterceptor,
  useTutorialProgress,
  useTutorialState,
  useTutorialStore,
} from "./store";
export type { Tutorial, TutorialMilestone, TutorialRecording, TutorialRecordingEvent, TutorialState } from "./store";
export type { TutorialCommandContext, TutorialCommandResult } from "./commands";
export { helloTutorial } from "./exampleTutorial";
export { sketchpadTour } from "./sketchpadTour";
