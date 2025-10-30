// #region Header

// types.ts

// Type definitions for the tutorial system

// #endregion

import { Guid } from "../../semio";

/**
 * A complete tutorial with multiple milestones
 */
export interface Tutorial {
  id: Guid;
  name: string;
  description?: string;
  milestones: TutorialMilestone[];
  totalDuration?: number;
  icon?: string;
  image?: string;
}

/**
 * A single step in a tutorial
 */
export interface TutorialMilestone {
  id: Guid;
  title: string;
  description?: string;
  commandPattern?: CommandPattern;
  focusElement?: FocusElement;
  cursorAnimation?: CursorAnimation;
  audioUrl?: string;
  videoUrl?: string;
  duration?: number;
  canSkip: boolean;
  order: number;
}

/**
 * Pattern for matching commands to advance milestones
 */
export interface CommandPattern {
  command: string;
  origin?: string;
  argsPattern?: any;
}

/**
 * Configuration for highlighting UI elements
 */
export interface FocusElement {
  selector: string;
  highlightMode: "dim" | "spotlight" | "pulse";
}

/**
 * Configuration for cursor animations
 */
export interface CursorAnimation {
  startX: number;
  startY: number;
  endX: number;
  endY: number;
  duration: number;
  action?: "click" | "drag" | "hover";
}

/**
 * A recorded session that can be played back or converted to a tutorial
 */
export interface TutorialRecording {
  id: Guid;
  tutorialId?: Guid;
  name: string;
  startTime: number;
  duration: number;
  events: TutorialRecordingEvent[];
}

/**
 * A single event in a recording
 */
export interface TutorialRecordingEvent {
  timestamp: number;
  type: "command" | "cursor" | "interaction";
  data: EventData;
}

/**
 * Event data payload
 */
export interface EventData {
  command?: string;
  origin?: string;
  args?: any;
  cursorX?: number;
  cursorY?: number;
  action?: "click" | "drag" | "hover" | "move";
  target?: string;
}

/**
 * Playback state of a tutorial
 */
export enum TutorialPlaybackState {
  IDLE = "idle",
  PLAYING = "playing",
  PAUSED = "paused",
  COMPLETED = "completed",
}

/**
 * Recording state
 */
export enum TutorialRecordingState {
  IDLE = "idle",
  RECORDING = "recording",
  PAUSED = "paused",
}

/**
 * Complete tutorial system state
 */
export interface TutorialState {
  activeTutorial: Tutorial | null;
  currentMilestoneIndex: number;
  playbackState: TutorialPlaybackState;
  playbackTime: number;
  recordingState: TutorialRecordingState;
  activeRecording: TutorialRecording | null;
  availableTutorials: Tutorial[];
}

/**
 * Diff for updating tutorial state
 */
export interface TutorialDiff {
  activeTutorial?: Tutorial | null;
  currentMilestoneIndex?: number;
  playbackState?: TutorialPlaybackState;
  playbackTime?: number;
  recordingState?: TutorialRecordingState;
  activeRecording?: TutorialRecording | null;
}
