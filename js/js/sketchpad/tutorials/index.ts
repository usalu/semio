// #region Header

// index.ts

// 2025 Ueli Saluz

// #endregion

export { commands as tutorialCommands } from "./commands";
export type { TutorialCommandContext, TutorialCommandResult } from "./commands";
export { helloTutorial } from "./exampleTutorial";
export { RecordButton } from "./RecordButton";
export { sketchpadTour } from "./sketchpadTour";
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
export { RecordingControls, TutorialControls } from "./TutorialControls";
export { TutorialOverlay } from "./TutorialOverlay";
