// #region 🔖Header
// [👤semio📚js🗃️sketchpad💻tutorials](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Interactive tutorial system with step-by-step guided workflows.

// #endregion 🔖Header

// #region 🔖Imports
// [👤semio📚js🗃️sketchpad💻tutorials🔖imports](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Imports)
// External and internal module imports MUST be declared here.

import { createContext, FC, ReactNode, useCallback, useContext, useEffect, useState } from "react";
import { CloseIcon, PauseIcon, PlayIcon, RecordIcon, SkipBackIcon, SkipForwardIcon, StopIcon } from "@semio/assets";
import { guid, Guid } from "../semio";
import { useAddFooterItem, useMode, useRemoveFooterItem } from "./Sketchpad";
import { Button, Slider } from "./elements";
import { Mode } from "./shared";

// #endregion 🔖Imports

// #region 🔖Components
// [🔖semio/js/sketchpad/Tutorials.tsx#Components](semiorepo://section/semio/js/sketchpad/Tutorials.tsx/COMPONENTS)
// Tutorial UI components MUST provide playback and recording controls.

// #region 🔖Tutorial Controls
// [👤semio📚js🗃️sketchpad💻tutorials🔖components](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components)
// Tutorial playback controls MUST render in the footer during active tutorials.

/**
 * Footer controls component for tutorial playback.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🪨tutorialcontrols](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/d/i/TutorialControls)
 **/
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

/** TutorialControlsContent holds the data fields for a TutorialControlsContent record.
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🪨tutorialcontrolscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/d/i/TutorialControlsContent)
/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🪨tutorialcontrolscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/d/i/TutorialControlsContent)
 **/
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

// #region 🔖Recording Controls
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖recordingcontrols](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Recording%20Controls)
// Recording controls MUST render in the footer during active recording in dev mode.

/**
 * Footer controls component for tutorial recording.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖recordingcontrols🪨recordingcontrols](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Recording%20Controls/d/i/RecordingControls)
 **/
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

/** RecordingControlsContent holds the data fields for a RecordingControlsContent record.
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖recordingcontrols🪨recordingcontrolscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Recording%20Controls/d/i/RecordingControlsContent)
/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖recordingcontrols🪨recordingcontrolscontent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Recording%20Controls/d/i/RecordingControlsContent)
 **/
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
// #endregion 🔖Recording Controls

// #region 🔖Record Button
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖recordbutton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Record%20Button)
// Record button MUST toggle recording in the footer when in dev mode.

/**
 * Footer button component toggling tutorial recording.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖recordbutton🪨recordbutton](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Record%20Button/d/i/RecordButton)
 **/
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
// #endregion 🔖Record Button

// #region 🔖Tutorial Overlay
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay)
// Tutorial overlay MUST render focus highlights and cursor animations during playback.

/**
 * Overlay component rendering focus highlights and animated cursor.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay🪨tutorialoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/TutorialOverlay)
 **/
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

/**
 * FocusOverlayProps holds the data fields for a FocusOverlayProps record.
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay✂️focusoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/FocusOverlay)
 **/
interface FocusOverlayProps {
  focusElement: {
    selector: string;
    highlightMode: "dim" | "spotlight" | "pulse";
  };
}

// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialoverlay🪨focusoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Overlay/d/i/FocusOverlay)
/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialoverlay🪨focusoverlay](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Overlay/d/i/FocusOverlay)
 * FocusOverlay holds the data fields for a FocusOverlay record.
 **/
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

/**
 * AnimatedCursorProps holds the data fields for a AnimatedCursorProps record.
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay✂️animatedcursorprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/AnimatedCursorProps)
 **/
interface AnimatedCursorProps {
  position: { x: number; y: number };
  action?: "click" | "drag" | "hover";
}

/** AnimatedCursor holds the data fields for a AnimatedCursor record.
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay🪨animatedcursor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/AnimatedCursor)
/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialoverlay🪨animatedcursor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Overlay/d/i/AnimatedCursor)
 **/
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

/**
 * MilestoneTooltipProps holds the data fields for a MilestoneTooltipProps record.
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay✂️milestonetooltipprops](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/MilestoneTooltipProps)
 **/
interface MilestoneTooltipProps {
  milestone: {
    title: string;
    description?: string;
  };
}
/** MilestoneTooltip holds the data fields for a MilestoneTooltip record.
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖tutorialcontrols🔖tutorialoverlay🪨milestonetooltip](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Tutorial%20Controls/s/Tutorial%20Overlay/d/i/MilestoneTooltip)
 **/
const MilestoneTooltip: FC<MilestoneTooltipProps> = ({ milestone }) => {
  return (
    <div className="fixed top-small left-1/2 -translate-x-1/2 z-tutorial border rounded-lg shadow-lg p-small max-w-md pointer-events-none">
      <h3 className="font-medium text-sm mb-1">{milestone.title}</h3>
      {milestone.description && <p className="text-xs text-muted">{milestone.description}</p>}
    </div>
  );
};

// #endregion 🔖Tutorial Overlay

// #endregion 🔖Tutorial Controls
// #region 🔖Built-in Tutorials
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖builtintutorials](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Built-in%20Tutorials)
// Built-in tutorials MUST define default tutorial content shipped with the app.

/**
 * Built-in hello tutorial introducing basic semio concepts.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖builtintutorials🪨hellotutorial](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Built-in%20Tutorials/d/i/helloTutorial)
 **/
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

/**
 * Built-in sketchpad tour tutorial for core features.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖builtintutorials🪨sketchpadtour](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Built-in%20Tutorials/d/i/sketchpadTour)
 **/
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

// #endregion 🔖Built-in Tutorials

// #region 🔖Commands
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Commands)
// Tutorial and recording command definitions MUST map command names to store actions.

/**
 * Context passed to tutorial command handlers.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commands🛠️tutorialcommandcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Commands/d/i/TutorialCommandContext)
 **/
export interface TutorialCommandContext {
  tutorialStore: any;
}

/**
 * Result returned from tutorial command execution.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commands🛠️tutorialcommandresult](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Commands/d/i/TutorialCommandResult)
 **/
export interface TutorialCommandResult {
  success: boolean;
  data?: any;
}

/**
 * Map of tutorial command names to handler functions.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commands🪨tutorialcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Commands/d/i/tutorialCommands)
 **/
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

/**
 * Map of dev-mode recording command names to handler functions.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commands🪨devcommands](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Commands/d/i/devCommands)
 **/
export const devCommands = {
  "semio.recording.start": (context: TutorialCommandContext, name: string, tutorialId?: string): TutorialCommandResult => {
    context.tutorialStore.startRecording(name, tutorialId);
    return { success: true };
  },
  "semio.recording.pause": (context: TutorialCommandContext): TutorialCommandResult => {
    context.tutorialStore.pauseRecording();
    return { success: true };
  },
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

// #endregion 🔖Commands

// #region 🔖Command Interceptor
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commandinterceptor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Command%20Interceptor)
// Command interceptor MUST record events and check milestone completion during playback.

/**
 * Hook intercepting commands to record events and check milestone completion.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖commandinterceptor🪨usetutorialcommandinterceptor](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Command%20Interceptor/d/i/useTutorialCommandInterceptor)
 **/
export const useTutorialCommandInterceptor = (onCommandExecute: (command: string, origin?: string, args?: any) => void) => {
  const store = useTutorialStore();
  const isRecording = useIsRecording();
  return useCallback(
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

// #endregion 🔖Command Interceptor

// #region 🔖Hooks
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks)
// Tutorial hooks MUST provide reactive access to tutorial and recording state.

/**
 * Hook returning the tutorial store instance.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨usetutorialstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useTutorialStore)
 **/
export const useTutorialStore = () => useTutorialContext().store;
/**
 * Hook returning the current tutorial state.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨usetutorialstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useTutorialState)
 **/
export const useTutorialState = () => useTutorialContext().state;
/**
 * Hook returning the currently active tutorial.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useactivetutorial](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useActiveTutorial)
 **/
export const useActiveTutorial = () => useTutorialContext().state.activeTutorial;
/**
 * Hook returning the current milestone of the active tutorial.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨usecurrentmilestone](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useCurrentMilestone)
 **/
export const useCurrentMilestone = () => {
  const { state } = useTutorialContext();
  if (!state.activeTutorial) return null;
  return state.activeTutorial.milestones[state.currentMilestoneIndex] || null;
};
/**
 * Hook returning the current playback state.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useplaybackstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/usePlaybackState)
 **/
export const usePlaybackState = () => useTutorialContext().state.playbackState;
/**
 * Hook returning the current playback time.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useplaybacktime](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/usePlaybackTime)
 **/
export const usePlaybackTime = () => useTutorialContext().state.playbackTime;
/**
 * Hook returning the current recording state.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨userecordingstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useRecordingState)
 **/
export const useRecordingState = () => useTutorialContext().state.recordingState;
/**
 * Hook returning the active recording.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useactiverecording](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useActiveRecording)
 **/
export const useActiveRecording = () => useTutorialContext().state.activeRecording;
/**
 * Hook returning all available tutorials.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useavailabletutorials](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useAvailableTutorials)
 **/
export const useAvailableTutorials = () => useTutorialContext().state.availableTutorials;
/**
 * Hook returning whether recording is active.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useisrecording](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useIsRecording)
 **/
export const useIsRecording = () => useTutorialContext().state.recordingState === TutorialRecordingState.RECORDING;
/**
 * Hook returning whether a tutorial is actively playing.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖hooks🪨useistutorialactive](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Hooks/d/i/useIsTutorialActive)
 **/
export const useIsTutorialActive = () => {
  const state = useTutorialContext().state;
  return state.playbackState !== TutorialPlaybackState.IDLE && state.playbackState !== TutorialPlaybackState.COMPLETED;
};
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖hooks🪨usetutorialprogress](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Hooks/d/i/useTutorialProgress)
 **/
export const useTutorialProgress = () => {
  const { state } = useTutorialContext();
  if (!state.activeTutorial) return { current: 0, total: 0, percentage: 0 };
  const total = state.activeTutorial.milestones.length;
  const current = state.currentMilestoneIndex;
  const percentage = total > 0 ? (current / total) * 100 : 0;
  return { current, total, percentage };
};

// #endregion 🔖Hooks

// #region 🔖Context
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖context](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Context)
// Tutorial context MUST provide the store and state to descendant components.

/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖context✂️tutorialcontextvalue](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Context/d/i/TutorialContextValue)
 * TutorialContextValue holds the data fields for a TutorialContextValue record.
 **/
interface TutorialContextValue {
  store: TutorialStore;
  state: TutorialState;
}

/**
 * TutorialContext holds the data fields for a TutorialContext record.
 * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖context🪨tutorialcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Context/d/i/TutorialContext)
 **/
const TutorialContext = createContext<TutorialContextValue | null>(null);

/**
 * Provider component supplying tutorial store and state to descendants.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖context🪨tutorialprovider](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Context/d/i/TutorialProvider)
 **/
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
    }
    if (!existingIds.has(sketchpadTour.id)) {
      store.addTutorial(sketchpadTour);
  }, [state.availableTutorials, store]);
  return <TutorialContext.Provider value={{ store, state }}>{children}</TutorialContext.Provider>;
};

/** useTutorialContext holds the data fields for a useTutorialContext record.
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖context🪨usetutorialcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Context/d/i/useTutorialContext)
/**
 * [👤semio📚js🗃️sketchpad💻tutorials🔖context🪨usetutorialcontext](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Context/d/i/useTutorialContext)
 **/
const useTutorialContext = () => {
  const context = useContext(TutorialContext);
  if (!context) throw new Error("Tutorial hooks must be used within TutorialProvider");
  return context;
};

// #endregion 🔖Context

// #region 🔖Types
// [🔖semio/js/sketchpad/Tutorials.tsx#Types](semiorepo://section/semio/js/sketchpad/Tutorials.tsx/TYPES)
// Tutorial type definitions MUST be declared here.

// #region 🔖Tutorial Entities
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities)
// Tutorial entity interfaces MUST define milestones, recordings, and playback state.

/**
 * A single step within a tutorial with optional triggers and animations.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialmilestone](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialMilestone)
 **/
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

/**
 * A complete tutorial with ordered milestones.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorial](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/Tutorial)
 **/
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

/**
 * A timestamped event captured during tutorial recording.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialrecordingevent](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialRecordingEvent)
 **/
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

/**
 * A complete recording session with captured events.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialrecording](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialRecording)
 **/
export interface TutorialRecording {
  id: Guid;
  tutorialId?: Guid;
  name: string;
  startTime: number;
  duration: number;
  events: TutorialRecordingEvent[];
}

/**
 * Playback lifecycle states for a tutorial.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialplaybackstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialPlaybackState)
 **/
export enum TutorialPlaybackState {
  IDLE = "idle",
  PLAYING = "playing",
  PAUSED = "paused",
  COMPLETED = "completed",
}

/**
 * Recording lifecycle states for a tutorial.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialrecordingstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialRecordingState)
 **/
export enum TutorialRecordingState {
  IDLE = "idle",
  RECORDING = "recording",
  PAUSED = "paused",
}

/**
 * Complete state of the tutorial system.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialstate](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialState)
 **/
export interface TutorialState {
  activeTutorial: Tutorial | null;
  currentMilestoneIndex: number;
  playbackState: TutorialPlaybackState;
  playbackTime: number;
  recordingState: TutorialRecordingState;
}

/**
 * Partial state diff for updating tutorial state.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖types🔖tutorialentities🛠️tutorialdiff](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Types/s/Tutorial%20Entities/d/i/TutorialDiff)
 **/
export interface TutorialDiff {
  activeTutorial?: Tutorial | null;
  currentMilestoneIndex?: number;
  playbackState?: TutorialPlaybackState;
  playbackTime?: number;
  recordingState?: TutorialRecordingState;
  activeRecording?: TutorialRecording | null;
}

// #endregion 🔖Tutorial Entities

// #endregion 🔖Types

// #region 🔖Store
// [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖store](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Store)
// Tutorial store MUST manage playback, recording, and milestone navigation state.

/**
 * Tutorial store managing playback, recording, and milestone navigation.
 *
 * TutorialStore MUST synchronize state changes through the notify pattern.
 *
 *  * [👤semio📚js🗃️sketchpad💻tutorials🔖components🔖store🛠️tutorialstore](semiorepo://p/u/semio/b/l/js/fd/org/sketchpad/f/Tutorials.tsx/s/Components/s/Store/d/i/TutorialStore)
 **/
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

// #endregion 🔖Store
