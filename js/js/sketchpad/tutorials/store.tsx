// #region Header

// store.tsx

// 2025 Ueli Saluz

// #endregion

import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import * as Y from "yjs";
import { guid, Guid } from "../../semio";

// #region Types

export interface TutorialMilestone {
  id: Guid;
  title: string;
  description?: string;
  commandPattern?: {
    command: string;
    origin?: string;
    argsPattern?: any;
  };
  focusElement?: {
    selector: string;
    highlightMode: "dim" | "spotlight" | "pulse";
  };
  cursorAnimation?: {
    startX: number;
    startY: number;
    endX: number;
    endY: number;
    duration: number;
    action?: "click" | "drag" | "hover";
  };
  audioUrl?: string;
  videoUrl?: string;
  duration?: number;
  canSkip: boolean;
  order: number;
}

export interface Tutorial {
  id: Guid;
  name: string;
  description?: string;
  milestones: TutorialMilestone[];
  totalDuration?: number;
  icon?: string;
  image?: string;
  concepts?: string[];
}

export interface TutorialRecordingEvent {
  timestamp: number;
  type: "command" | "cursor" | "interaction";
  data: {
    command?: string;
    origin?: string;
    args?: any;
    cursorX?: number;
    cursorY?: number;
    action?: "click" | "drag" | "hover" | "move";
    target?: string;
  };
}

export interface TutorialRecording {
  id: Guid;
  tutorialId?: Guid;
  name: string;
  startTime: number;
  duration: number;
  events: TutorialRecordingEvent[];
}

export enum TutorialPlaybackState {
  IDLE = "idle",
  PLAYING = "playing",
  PAUSED = "paused",
  COMPLETED = "completed",
}

export enum TutorialRecordingState {
  IDLE = "idle",
  RECORDING = "recording",
  PAUSED = "paused",
}

export interface TutorialState {
  activeTutorial: Tutorial | null;
  currentMilestoneIndex: number;
  playbackState: TutorialPlaybackState;
  playbackTime: number;
  recordingState: TutorialRecordingState;
  activeRecording: TutorialRecording | null;
  availableTutorials: Tutorial[];
}

export interface TutorialDiff {
  activeTutorial?: Tutorial | null;
  currentMilestoneIndex?: number;
  playbackState?: TutorialPlaybackState;
  playbackTime?: number;
  recordingState?: TutorialRecordingState;
  activeRecording?: TutorialRecording | null;
}

// #endregion Types

// #region Store

export class TutorialStore {
  private yMap: Y.Map<any>;
  private transact: (fn: () => void) => void;
  private listeners: Set<() => void> = new Set();
  private playbackInterval?: number;
  private recordingStartTime?: number;

  constructor(yMap: Y.Map<any>, transact: (fn: () => void) => void) {
    this.yMap = yMap;
    this.transact = transact;
    this.initialize();
  }

  private initialize(): void {
    this.transact(() => {
      if (!this.yMap.has("activeTutorial")) {
        this.yMap.set("activeTutorial", null);
      }
      if (!this.yMap.has("currentMilestoneIndex")) {
        this.yMap.set("currentMilestoneIndex", 0);
      }
      if (!this.yMap.has("playbackState")) {
        this.yMap.set("playbackState", TutorialPlaybackState.IDLE);
      }
      if (!this.yMap.has("playbackTime")) {
        this.yMap.set("playbackTime", 0);
      }
      if (!this.yMap.has("recordingState")) {
        this.yMap.set("recordingState", TutorialRecordingState.IDLE);
      }
      if (!this.yMap.has("activeRecording")) {
        this.yMap.set("activeRecording", null);
      }
      if (!this.yMap.has("availableTutorials")) {
        this.yMap.set("availableTutorials", []);
      }
    });
  }

  snapshot(): TutorialState {
    return {
      activeTutorial: this.yMap.get("activeTutorial"),
      currentMilestoneIndex: this.yMap.get("currentMilestoneIndex") || 0,
      playbackState: this.yMap.get("playbackState") || TutorialPlaybackState.IDLE,
      playbackTime: this.yMap.get("playbackTime") || 0,
      recordingState: this.yMap.get("recordingState") || TutorialRecordingState.IDLE,
      activeRecording: this.yMap.get("activeRecording"),
      availableTutorials: this.yMap.get("availableTutorials") || [],
    };
  }

  change(diff: TutorialDiff): void {
    this.transact(() => {
      if (diff.activeTutorial !== undefined) {
        this.yMap.set("activeTutorial", diff.activeTutorial);
      }
      if (diff.currentMilestoneIndex !== undefined) {
        this.yMap.set("currentMilestoneIndex", diff.currentMilestoneIndex);
      }
      if (diff.playbackState !== undefined) {
        this.yMap.set("playbackState", diff.playbackState);
      }
      if (diff.playbackTime !== undefined) {
        this.yMap.set("playbackTime", diff.playbackTime);
      }
      if (diff.recordingState !== undefined) {
        this.yMap.set("recordingState", diff.recordingState);
      }
      if (diff.activeRecording !== undefined) {
        this.yMap.set("activeRecording", diff.activeRecording);
      }
    });
    this.notify();
  }

  subscribe(listener: () => void): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify(): void {
    this.listeners.forEach((listener) => listener());
  }

  startTutorial(tutorial: Tutorial): void {
    this.change({
      activeTutorial: tutorial,
      currentMilestoneIndex: 0,
      playbackState: TutorialPlaybackState.PLAYING,
      playbackTime: 0,
    });
    this.startPlaybackTimer();
  }

  pauseTutorial(): void {
    this.change({ playbackState: TutorialPlaybackState.PAUSED });
    this.stopPlaybackTimer();
  }

  resumeTutorial(): void {
    this.change({ playbackState: TutorialPlaybackState.PLAYING });
    this.startPlaybackTimer();
  }

  stopTutorial(): void {
    this.change({
      activeTutorial: null,
      currentMilestoneIndex: 0,
      playbackState: TutorialPlaybackState.IDLE,
      playbackTime: 0,
    });
    this.stopPlaybackTimer();
  }

  nextMilestone(): void {
    const state = this.snapshot();
    if (!state.activeTutorial) return;
    const nextIndex = state.currentMilestoneIndex + 1;
    if (nextIndex >= state.activeTutorial.milestones.length) {
      this.change({ playbackState: TutorialPlaybackState.COMPLETED });
      this.stopPlaybackTimer();
    } else {
      this.change({ currentMilestoneIndex: nextIndex, playbackTime: 0 });
    }
  }

  previousMilestone(): void {
    const state = this.snapshot();
    if (!state.activeTutorial) return;
    const prevIndex = Math.max(0, state.currentMilestoneIndex - 1);
    this.change({ currentMilestoneIndex: prevIndex, playbackTime: 0 });
  }

  goToMilestone(index: number): void {
    const state = this.snapshot();
    if (!state.activeTutorial) return;
    if (index < 0 || index >= state.activeTutorial.milestones.length) return;
    this.change({ currentMilestoneIndex: index, playbackTime: 0 });
  }

  checkCommandCompletion(command: string, origin?: string, args?: any): boolean {
    const state = this.snapshot();
    if (!state.activeTutorial || state.playbackState !== TutorialPlaybackState.PLAYING) return false;
    const milestone = state.activeTutorial.milestones[state.currentMilestoneIndex];
    if (!milestone?.commandPattern) return false;
    const pattern = milestone.commandPattern;
    if (pattern.command !== command) return false;
    if (pattern.origin && pattern.origin !== origin) return false;
    if (pattern.argsPattern) {
      const argsMatch = this.matchArgsPattern(args, pattern.argsPattern);
      if (!argsMatch) return false;
    }
    this.nextMilestone();
    return true;
  }

  private matchArgsPattern(args: any, pattern: any): boolean {
    if (typeof pattern !== "object" || pattern === null) {
      return args === pattern;
    }
    if (Array.isArray(pattern)) {
      if (!Array.isArray(args)) return false;
      return pattern.every((p, i) => this.matchArgsPattern(args[i], p));
    }
    if (typeof args !== "object" || args === null) return false;
    return Object.keys(pattern).every((key) => this.matchArgsPattern(args[key], pattern[key]));
  }

  private startPlaybackTimer(): void {
    this.stopPlaybackTimer();
    this.playbackInterval = window.setInterval(() => {
      const state = this.snapshot();
      if (state.playbackState !== TutorialPlaybackState.PLAYING) return;
      const milestone = state.activeTutorial?.milestones[state.currentMilestoneIndex];
      if (!milestone) return;
      const newTime = state.playbackTime + 0.1;
      if (milestone.duration && newTime >= milestone.duration && milestone.canSkip) {
        this.nextMilestone();
      } else {
        this.change({ playbackTime: newTime });
      }
    }, 100);
  }

  private stopPlaybackTimer(): void {
    if (this.playbackInterval) {
      window.clearInterval(this.playbackInterval);
      this.playbackInterval = undefined;
    }
  }

  startRecording(name: string, tutorialId?: Guid): void {
    const recording: TutorialRecording = {
      id: guid(),
      tutorialId,
      name,
      startTime: Date.now(),
      duration: 0,
      events: [],
    };
    this.recordingStartTime = Date.now();
    this.change({
      recordingState: TutorialRecordingState.RECORDING,
      activeRecording: recording,
    });
  }

  pauseRecording(): void {
    const state = this.snapshot();
    if (state.recordingState !== TutorialRecordingState.RECORDING) return;
    this.change({ recordingState: TutorialRecordingState.PAUSED });
  }

  resumeRecording(): void {
    const state = this.snapshot();
    if (state.recordingState !== TutorialRecordingState.PAUSED) return;
    this.change({ recordingState: TutorialRecordingState.RECORDING });
  }

  stopRecording(): TutorialRecording | null {
    const state = this.snapshot();
    if (!state.activeRecording) return null;
    const recording = state.activeRecording;
    recording.duration = Date.now() - recording.startTime;
    this.change({
      recordingState: TutorialRecordingState.IDLE,
      activeRecording: null,
    });
    this.recordingStartTime = undefined;
    return recording;
  }

  downloadRecording(recording: TutorialRecording): void {
    const json = JSON.stringify(recording, null, 2);
    const blob = new Blob([json], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `${recording.name.replace(/[^a-z0-9]/gi, "_").toLowerCase()}_${recording.id}.json`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  }

  recordEvent(event: Omit<TutorialRecordingEvent, "timestamp">): void {
    const state = this.snapshot();
    if (state.recordingState !== TutorialRecordingState.RECORDING || !state.activeRecording) return;
    const timestamp = this.recordingStartTime ? Date.now() - this.recordingStartTime : 0;
    const fullEvent: TutorialRecordingEvent = { timestamp, ...event };
    this.transact(() => {
      const recording = this.yMap.get("activeRecording");
      if (recording) {
        recording.events.push(fullEvent);
        this.yMap.set("activeRecording", recording);
      }
    });
    this.notify();
  }

  addTutorial(tutorial: Tutorial): void {
    this.transact(() => {
      const tutorials = this.yMap.get("availableTutorials") || [];
      tutorials.push(tutorial);
      this.yMap.set("availableTutorials", tutorials);
    });
    this.notify();
  }

  removeTutorial(tutorialId: Guid): void {
    this.transact(() => {
      const tutorials = this.yMap.get("availableTutorials") || [];
      const filtered = tutorials.filter((t: Tutorial) => t.id !== tutorialId);
      this.yMap.set("availableTutorials", filtered);
    });
    this.notify();
  }

  convertRecordingToTutorial(recording: TutorialRecording, name: string, description?: string): Tutorial {
    const milestones: TutorialMilestone[] = [];
    const commandEvents = recording.events.filter((e) => e.type === "command");
    commandEvents.forEach((event, index) => {
      if (event.data.command) {
        const milestone: TutorialMilestone = {
          id: guid(),
          title: `Step ${index + 1}: ${event.data.command}`,
          description: description || `Execute ${event.data.command}`,
          commandPattern: {
            command: event.data.command,
            origin: event.data.origin,
          },
          canSkip: true,
          order: index,
        };
        milestones.push(milestone);
      }
    });
    const tutorial: Tutorial = {
      id: guid(),
      name,
      description,
      milestones,
      totalDuration: recording.duration,
    };
    return tutorial;
  }
}

// #endregion Store

// #region Context

interface TutorialContextValue {
  store: TutorialStore;
  state: TutorialState;
}

const TutorialContext = createContext<TutorialContextValue | null>(null);

export const TutorialProvider: FC<{ store: TutorialStore; children: ReactNode }> = ({ store, children }) => {
  const [state, setState] = useState<TutorialState>(store.snapshot());

  useEffect(() => {
    const unsubscribe = store.subscribe(() => {
      setState(store.snapshot());
    });
    return unsubscribe;
  }, [store]);

  useEffect(() => {
    const registerBuiltInTutorials = async () => {
      try {
        const { helloTutorial, sketchpadTour } = await import("./index");
        const existingIds = state.availableTutorials.map((t) => t.id);
        if (!existingIds.includes(helloTutorial.id)) {
          store.addTutorial(helloTutorial);
        }
        if (!existingIds.includes(sketchpadTour.id)) {
          store.addTutorial(sketchpadTour);
        }
      } catch (e) {
        console.error("Failed to register built-in tutorials:", e);
      }
    };
    registerBuiltInTutorials();
  }, [store, state.availableTutorials]);

  return <TutorialContext.Provider value={{ store, state }}>{children}</TutorialContext.Provider>;
};

const useTutorialContext = () => {
  const context = useContext(TutorialContext);
  if (!context) throw new Error("Tutorial hooks must be used within TutorialProvider");
  return context;
};

// #endregion Context

// #region Hooks

export const useTutorialStore = () => useTutorialContext().store;
export const useTutorialState = () => useTutorialContext().state;
export const useActiveTutorial = () => useTutorialContext().state.activeTutorial;
export const useCurrentMilestone = () => {
  const { state } = useTutorialContext();
  if (!state.activeTutorial) return null;
  return state.activeTutorial.milestones[state.currentMilestoneIndex] || null;
};
export const usePlaybackState = () => useTutorialContext().state.playbackState;
export const usePlaybackTime = () => useTutorialContext().state.playbackTime;
export const useRecordingState = () => useTutorialContext().state.recordingState;
export const useActiveRecording = () => useTutorialContext().state.activeRecording;
export const useAvailableTutorials = () => useTutorialContext().state.availableTutorials;
export const useIsRecording = () => useTutorialContext().state.recordingState === TutorialRecordingState.RECORDING;
export const useIsTutorialActive = () => {
  const state = useTutorialContext().state;
  return state.playbackState !== TutorialPlaybackState.IDLE && state.playbackState !== TutorialPlaybackState.COMPLETED;
};

export const useTutorialProgress = () => {
  const { state } = useTutorialContext();
  if (!state.activeTutorial) return { current: 0, total: 0, percentage: 0 };
  const total = state.activeTutorial.milestones.length;
  const current = state.currentMilestoneIndex;
  const percentage = total > 0 ? (current / total) * 100 : 0;
  return { current, total, percentage };
};

// #endregion Hooks

// #region Command Interceptor

export const useTutorialCommandInterceptor = (onCommandExecute: (command: string, origin?: string, args?: any) => void) => {
  const store = useTutorialStore();
  const isRecording = useIsRecording();

  useEffect(() => {
    const originalExecute = onCommandExecute;
    return () => { };
  }, [store, isRecording, onCommandExecute]);

  return useCallback(
    (command: string, origin?: string, args?: any) => {
      if (isRecording) {
        store.recordEvent({
          type: "command",
          data: { command, origin, args },
        });
      }
      store.checkCommandCompletion(command, origin, args);
      onCommandExecute(command, origin, args);
    },
    [store, isRecording, onCommandExecute]
  );
};

// #endregion Command Interceptor
