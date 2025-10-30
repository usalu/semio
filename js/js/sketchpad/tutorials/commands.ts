// #region Header

// commands.ts

// 2025 Ueli Saluz

// #endregion

import { Tutorial, TutorialRecording } from "./store";

export interface TutorialCommandContext {
  tutorialStore: any;
}

export interface TutorialCommandResult {
  success: boolean;
  data?: any;
}

export const commands = {
  "semio.tutorial.start": (context: TutorialCommandContext, tutorial: Tutorial): TutorialCommandResult => {
    context.tutorialStore.startTutorial(tutorial);
    return { success: true };
  },
  "semio.tutorial.pause": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.pauseTutorial();
    return { success: true };
  },
  "semio.tutorial.resume": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.resumeTutorial();
    return { success: true };
  },
  "semio.tutorial.stop": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.stopTutorial();
    return { success: true };
  },
  "semio.tutorial.nextMilestone": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.nextMilestone();
    return { success: true };
  },
  "semio.tutorial.previousMilestone": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.previousMilestone();
    return { success: true };
  },
  "semio.tutorial.goToMilestone": (context: TutorialCommandContext, index: number): TutorialCommandResult => {
    context.tutorialStore.goToMilestone(index);
    return { success: true };
  },
  "semio.recording.start": (context: TutorialCommandContext, name: string, tutorialId?: string): TutorialCommandResult => {
    context.tutorialStore.startRecording(name, tutorialId);
    return { success: true };
  },
  "semio.recording.pause": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.pauseRecording();
    return { success: true };
  },
  "semio.recording.resume": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.resumeRecording();
    return { success: true };
  },
  "semio.recording.stop": (context: TutorialCommandContext): TutorialCommandResult => {
    const recording = context.tutorialStore.stopRecording();
    return { success: true, data: recording };
  },
  "semio.tutorial.add": (context: TutorialCommandContext, tutorial: Tutorial): TutorialCommandResult => {
    context.tutorialStore.addTutorial(tutorial);
    return { success: true };
  },
  "semio.tutorial.remove": (context: TutorialCommandContext, tutorialId: string): TutorialCommandResult => {
    context.tutorialStore.removeTutorial(tutorialId);
    return { success: true };
  },
  "semio.recording.convertToTutorial": (context: TutorialCommandContext, recording: TutorialRecording, name: string, description?: string): TutorialCommandResult => {
    const tutorial = context.tutorialStore.convertRecordingToTutorial(recording, name, description);
    return { success: true, data: tutorial };
  },
};
