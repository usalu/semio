// #region Header

// js/js/sketchpad/Tutorials.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { CloseIcon, PauseIcon, PlayIcon, RecordIcon, SkipBackIcon, SkipForwardIcon, StopIcon } from "@semio/assets";
import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useState } from "react";
import { guid, Guid } from "../semio";
import { useAddFooterItem, useMode, useRemoveFooterItem } from "./Sketchpad";
import { Button, Slider } from "./elements";
import { Mode } from "./shared";

// #endregion Imports

// #region Components

// #region Tutorial Controls

export const TutorialControls: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const isTutorialActive = useIsTutorialActive();
  useEffect(() => {
    if (isTutorialActive) {
      addFooterItem({
        id: "tutorial-controls",
        content: <TutorialControlsContent />,
        className: "aspect-auto",
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
      <Button id="semio.sketchpad.tutorial.controls.stop" variant="ghost" onClick={() => store.stopTutorial()} className="size-tiny p-0">
        <CloseIcon className="size-tiny" />
      </Button>
      <div className="flex items-center gap-single">
        <Button id="semio.sketchpad.tutorial.controls.previous" variant="ghost" onClick={() => store.previousMilestone()} disabled={progress.current === 0} className="size-tiny p-0">
          <SkipBackIcon className="size-tiny" />
        </Button>
        <Button
          id="semio.sketchpad.tutorial.controls.playPause"
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
        <Button id="semio.sketchpad.tutorial.controls.next" variant="ghost" onClick={() => store.nextMilestone()} disabled={progress.current >= progress.total - 1} className="size-tiny p-0">
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

// #endregion Tutorial Controls

// #region Recording Controls

export const RecordingControls: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const [mode] = useMode();
  const recordingState = useRecordingState();
  const isRecordingActive = recordingState !== TutorialRecordingState.IDLE;

  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("recording-controls");
      return;
    }
    if (isRecordingActive) {
      addFooterItem({
        id: "recording-controls",
        content: <RecordingControlsContent />,
        className: "aspect-auto",
        order: 1,
      });
    } else {
      removeFooterItem("recording-controls");
    }
    return () => {
      removeFooterItem("recording-controls");
    };
  }, [mode, isRecordingActive, addFooterItem, removeFooterItem]);
  return null;
};

const RecordingControlsContent: FC = () => {
  const store = useTutorialStore();
  const recordingState = useRecordingState();
  const activeRecording = useActiveRecording();
  const isRecording = recordingState === TutorialRecordingState.RECORDING;
  const isPaused = recordingState === TutorialRecordingState.PAUSED;
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
        id="semio.sketchpad.recording.controls.playPause"
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
      <Button id="semio.sketchpad.recording.controls.stop" variant="ghost" onClick={handleStop} className="size-tiny p-0">
        <StopIcon className="size-tiny" />
      </Button>
      {activeRecording && <div className="text-xs text-muted">{activeRecording.name}</div>}
    </div>
  );
};

// #endregion Recording Controls

// #region Record Button

export const RecordButton: FC = () => {
  const addFooterItem = useAddFooterItem();
  const removeFooterItem = useRemoveFooterItem();
  const [mode] = useMode();
  const isRecording = useIsRecording();
  const store = useTutorialStore();
  useEffect(() => {
    if (mode !== Mode.DEV) {
      removeFooterItem("record-button");
      return;
    }
    addFooterItem({
      id: "record-button",
      content: (
        <Button
          id="footer-record-button"
          variant="ghost"
          onClick={() => {
            if (isRecording) {
              store.stopRecording();
            } else {
              store.startRecording("New Recording", undefined);
            }
          }}
          className={isRecording ? "text-red-500 h-small w-small p-0" : "h-small w-small p-0"}
        >
          <RecordIcon className={isRecording ? "fill-current size-tiny" : "size-tiny"} />
        </Button>
      ),
      order: 0,
    });
    return () => {
      removeFooterItem("record-button");
    };
  }, [mode, isRecording, store, addFooterItem, removeFooterItem]);
  return null;
};

// #endregion Record Button

// #region Tutorial Overlay

export const TutorialOverlay: FC = () => {
  const isTutorialActive = useIsTutorialActive();
  const currentMilestone = useCurrentMilestone();
  const playbackTime = usePlaybackTime();
  const [cursorPosition, setCursorPosition] = useState<{ x: number; y: number } | null>(null);
  useEffect(() => {
    if (!currentMilestone?.cursorAnimation) {
      setCursorPosition(null);
      return;
    }
    const animation = currentMilestone.cursorAnimation;
    const progress = Math.min(playbackTime / animation.duration, 1);
    const x = animation.startX + (animation.endX - animation.startX) * progress;
    const y = animation.startY + (animation.endY - animation.startY) * progress;
    setCursorPosition({ x, y });
  }, [currentMilestone, playbackTime]);
  if (!isTutorialActive || !currentMilestone) return null;
  return (
    <>
      {currentMilestone.focusElement && <FocusOverlay focusElement={currentMilestone.focusElement} />}
      {cursorPosition && <AnimatedCursor position={cursorPosition} action={currentMilestone.cursorAnimation?.action} />}
      <MilestoneTooltip milestone={currentMilestone} />
    </>
  );
};

interface FocusOverlayProps {
  focusElement: {
    selector: string;
    highlightMode: "dim" | "spotlight" | "pulse";
  };
}

const FocusOverlay: FC<FocusOverlayProps> = ({ focusElement }) => {
  const [rect, setRect] = useState<DOMRect | null>(null);
  useEffect(() => {
    const element = document.querySelector(focusElement.selector);
    if (!element) return;
    const updateRect = () => {
      setRect(element.getBoundingClientRect());
    };
    updateRect();
    const observer = new ResizeObserver(updateRect);
    observer.observe(element);
    window.addEventListener("resize", updateRect);
    return () => {
      observer.disconnect();
      window.removeEventListener("resize", updateRect);
    };
  }, [focusElement.selector]);
  if (!rect) return null;
  return (
    <>
      {focusElement.highlightMode === "dim" && (
        <div className="fixed inset-0 z-tutorial pointer-events-none">
          <svg width="100%" height="100%">
            <defs>
              <mask id="tutorial-mask">
                <rect width="100%" height="100%" fill="white" />
                <rect x={rect.x} y={rect.y} width={rect.width} height={rect.height} fill="black" rx="4" />
              </mask>
            </defs>
            <rect width="100%" height="100%" fill="rgb(0 0 0 / 0.7)" mask="url(#tutorial-mask)" />
          </svg>
          <div className="absolute border-2 border-primary rounded" style={{ left: rect.x - 4, top: rect.y - 4, width: rect.width + 8, height: rect.height + 8 }} />
        </div>
      )}
      {focusElement.highlightMode === "spotlight" && (
        <div className="fixed inset-0 z-tutorial pointer-events-none">
          <div
            className="absolute rounded shadow-[0_0_0_9999px_rgba(0,0,0,0.7)] border-2 border-primary"
            style={{
              left: rect.x - 4,
              top: rect.y - 4,
              width: rect.width + 8,
              height: rect.height + 8,
            }}
          />
        </div>
      )}
      {focusElement.highlightMode === "pulse" && (
        <div className="fixed inset-0 z-tutorial pointer-events-none">
          <div
            className="absolute rounded border-2 border-primary animate-pulse"
            style={{
              left: rect.x - 4,
              top: rect.y - 4,
              width: rect.width + 8,
              height: rect.height + 8,
            }}
          />
        </div>
      )}
    </>
  );
};

interface AnimatedCursorProps {
  position: { x: number; y: number };
  action?: "click" | "drag" | "hover";
}

const AnimatedCursor: FC<AnimatedCursorProps> = ({ position, action }) => {
  return (
    <div className="fixed z-tutorial pointer-events-none transition-all duration-100" style={{ left: position.x, top: position.y }}>
      <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
        <path d="M3 3L10.07 19.97L12.58 12.58L19.97 10.07L3 3Z" fill="rgb(var(--primary))" stroke="rgb(var(--primary-foreground))" strokeWidth="1" />
      </svg>
      {action === "click" && (
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
          <div className="w-8 h-small rounded-full bg-primary/30 animate-ping" />
        </div>
      )}
    </div>
  );
};

interface MilestoneTooltipProps {
  milestone: {
    title: string;
    description?: string;
  };
}

const MilestoneTooltip: FC<MilestoneTooltipProps> = ({ milestone }) => {
  return (
    <div className="fixed top-small left-1/2 -translate-x-1/2 z-tutorial border rounded-lg shadow-lg p-small max-w-md pointer-events-none">
      <h3 className="font-medium text-sm mb-1">{milestone.title}</h3>
      {milestone.description && <p className="text-xs text-muted">{milestone.description}</p>}
    </div>
  );
};

// #endregion Tutorial Overlay

// #endregion Components
// #region Built-in Tutorials

export const helloTutorial: Tutorial = {
  id: guid(),
  name: "Hello semio Tutorial",
  description: "Learn the basics of semio by creating your first design",
  totalDuration: 300,
  icon: "??",
  image: "/tutorials/hello-semio.png",
  concepts: ["hello-semio", "getting-started", "beginner"],
  milestones: [
    {
      id: guid(),
      title: "Welcome to Semio",
      description: "Let's start by navigating to the home screen",
      commandPattern: {
        command: "semio.sketchpad.navigate",
        argsPattern: ["/"],
      },
      focusElement: {
        selector: "[data-navbar-home]",
        highlightMode: "spotlight",
      },
      cursorAnimation: {
        startX: 100,
        startY: 100,
        endX: 50,
        endY: 50,
        duration: 2,
        action: "click",
      },
      canSkip: true,
      order: 0,
    },
    {
      id: guid(),
      title: "Create a New Kit",
      description: "Click the 'New Kit' button to create your first kit",
      commandPattern: {
        command: "semio.sketchpad.createKit",
      },
      focusElement: {
        selector: "[data-action='create-kit']",
        highlightMode: "pulse",
      },
      canSkip: true,
      order: 1,
    },
    {
      id: guid(),
      title: "Open the Kit",
      description: "Now let's open your newly created kit",
      canSkip: true,
      order: 2,
      duration: 5,
    },
    {
      id: guid(),
      title: "Create a Type",
      description: "Add a new type to your kit",
      commandPattern: {
        command: "semio.kitApp.createType",
      },
      canSkip: true,
      order: 3,
    },
    {
      id: guid(),
      title: "Tutorial Complete!",
      description: "Congratulations! You've completed your first semio tutorial.",
      canSkip: false,
      order: 4,
      duration: 3,
    },
  ],
};

export const sketchpadTour: Tutorial = {
  id: guid(),
  name: "Sketchpad Tour",
  description: "A comprehensive introduction to Sketchpad - learn to create kits, types, designs, and more.",
  totalDuration: 600,
  icon: "??",
  concepts: ["getting-started", "beginner", "introduction"],
  milestones: [
    {
      id: guid(),
      title: "Welcome to Sketchpad",
      description: "Welcome! This tutorial will guide you through the core features of Sketchpad.",
      canSkip: true,
      order: 0,
      duration: 5,
    },
    {
      id: guid(),
      title: "Create a Kit",
      description: "Let''s start by creating a kit. Click the ''+'' button in the home view.",
      commandPattern: { command: "semio.home.kit.create" },
      focusElement: { selector: '[data-panel="home-create-kit"]', highlightMode: "spotlight" },
      canSkip: true,
      order: 1,
      duration: 10,
    },
    {
      id: guid(),
      title: "Open Your Kit",
      description: "Great! Now click on the kit you just created to open it.",
      commandPattern: { command: "semio.home.kit.open" },
      focusElement: { selector: "[data-kit-item]:first-child", highlightMode: "spotlight" },
      canSkip: true,
      order: 2,
      duration: 10,
    },
    {
      id: guid(),
      title: "Create a Type",
      description: "Now let''s create a type. Click the ''+'' button in the types section.",
      commandPattern: { command: "semio.kit.type.create" },
      focusElement: { selector: '[data-panel="kit-create-type"]', highlightMode: "spotlight" },
      canSkip: true,
      order: 3,
      duration: 10,
    },
    {
      id: guid(),
      title: "Tutorial Complete!",
      description: "Congratulations! You''ve completed the Sketchpad tour.",
      canSkip: false,
      order: 4,
      duration: 10,
    },
  ],
};

// #endregion Built-in Tutorials

// #region Commands

export interface TutorialCommandContext {
  tutorialStore: any;
}

export interface TutorialCommandResult {
  success: boolean;
  data?: any;
}

export const tutorialCommands = {
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
  "semio.tutorial.add": (context: TutorialCommandContext, tutorial: Tutorial): TutorialCommandResult => {
    context.tutorialStore.addTutorial(tutorial);
    return { success: true };
  },
  "semio.tutorial.remove": (context: TutorialCommandContext, tutorialId: string): TutorialCommandResult => {
    context.tutorialStore.removeTutorial(tutorialId);
    return { success: true };
  },
};

export const devCommands = {
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
  "semio.recording.convertToTutorial": (context: TutorialCommandContext, recording: TutorialRecording, name: string, description?: string): TutorialCommandResult => {
    const tutorial = context.tutorialStore.convertRecordingToTutorial(recording, name, description);
    return { success: true, data: tutorial };
  },
};

// #endregion Commands

// #region Command Interceptor

export const useTutorialCommandInterceptor = (onCommandExecute: (command: string, origin?: string, args?: any) => void) => {
  const store = useTutorialStore();
  const isRecording = useIsRecording();
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
    [store, isRecording, onCommandExecute],
  );
};

// #endregion Command Interceptor

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
    const existingIds = new Set(state.availableTutorials.map((t) => t.id));
    if (!existingIds.has(helloTutorial.id)) {
      store.addTutorial(helloTutorial);
    }
    if (!existingIds.has(sketchpadTour.id)) {
      store.addTutorial(sketchpadTour);
    }
  }, [state.availableTutorials, store]);

  return <TutorialContext.Provider value={{ store, state }}>{children}</TutorialContext.Provider>;
};

const useTutorialContext = () => {
  const context = useContext(TutorialContext);
  if (!context) throw new Error("Tutorial hooks must be used within TutorialProvider");
  return context;
};

// #endregion Context

// #region Types

// #region Tutorial Entities

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

// #endregion Tutorial Entities

// #endregion Types

// #region Store

export class TutorialStore {
  private state: TutorialState;
  private readonly listeners: Set<() => void> = new Set();
  private playbackInterval?: number;
  private recordingStartTime?: number;

  constructor(_yMap?: any, _transact?: (fn: () => void) => void) {
    this.state = {
      activeTutorial: null,
      currentMilestoneIndex: 0,
      playbackState: TutorialPlaybackState.IDLE,
      playbackTime: 0,
      recordingState: TutorialRecordingState.IDLE,
      activeRecording: null,
      availableTutorials: [],
    };
  }

  snapshot(): TutorialState {
    return this.state;
  }

  change(diff: TutorialDiff): void {
    const newState = { ...this.state };
    if (diff.activeTutorial !== undefined) newState.activeTutorial = diff.activeTutorial;
    if (diff.currentMilestoneIndex !== undefined) newState.currentMilestoneIndex = diff.currentMilestoneIndex;
    if (diff.playbackState !== undefined) newState.playbackState = diff.playbackState;
    if (diff.playbackTime !== undefined) newState.playbackTime = diff.playbackTime;
    if (diff.recordingState !== undefined) newState.recordingState = diff.recordingState;
    if (diff.activeRecording !== undefined) newState.activeRecording = diff.activeRecording;
    this.state = newState;
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
    const recording = { ...state.activeRecording };
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
    const recording = { ...state.activeRecording };
    recording.events = [...recording.events, fullEvent];
    this.state = { ...this.state, activeRecording: recording };
    this.notify();
  }

  addTutorial(tutorial: Tutorial): void {
    this.state = {
      ...this.state,
      availableTutorials: [...this.state.availableTutorials, tutorial],
    };
    this.notify();
  }

  removeTutorial(tutorialId: Guid): void {
    this.state = {
      ...this.state,
      availableTutorials: this.state.availableTutorials.filter((t) => t.id !== tutorialId),
    };
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
