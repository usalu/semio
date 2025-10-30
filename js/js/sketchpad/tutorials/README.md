# Tutorial System

A comprehensive tutorial and recording system for Sketchpad that provides guided experiences and the ability to record user interactions.

## Overview

The tutorial system consists of:

1. **Tutorials** - Guided step-by-step experiences with milestones
2. **Recordings** - Capture user interactions for playback or conversion to tutorials
3. **Overlay** - Visual guidance with focus effects and cursor animations
4. **Controls** - Footer-based playback and recording controls

## Features

### Tutorials

- **Milestones** - Break tutorials into discrete steps
- **Command Completion** - Automatically advance when specific commands are executed
- **Visual Focus** - Highlight UI elements with dim, spotlight, or pulse effects
- **Cursor Animation** - Animated cursor showing where to interact
- **Audio/Video** - Optional media for each milestone
- **Timeline Navigation** - Skip forward/backward or jump to specific milestones

### Recordings

- **Command Capture** - Record all commands with origins and arguments
- **Cursor Tracking** - Capture cursor positions and interactions
- **Pause/Resume** - Control recording flow
- **Convert to Tutorial** - Transform recordings into reusable tutorials

### Visual Guidance

- **Focus Modes**:
  - `dim` - Darken everything except the target element
  - `spotlight` - Create a spotlight effect on the target
  - `pulse` - Animated pulsing border

- **Cursor Animations**:
  - Smooth movement from start to end position
  - Visual indicators for click/drag/hover actions
  - Configurable duration

## Usage

### Starting a Tutorial

```typescript
import { useTutorialStore } from "./tutorials";

const store = useTutorialStore();
store.startTutorial(myTutorial);
```

### Creating a Tutorial

```typescript
import { Tutorial, TutorialMilestone } from "./tutorials";
import { guid } from "../semio";

const tutorial: Tutorial = {
  id: guid(),
  name: "My Tutorial",
  description: "Learn how to use feature X",
  milestones: [
    {
      id: guid(),
      title: "Step 1",
      description: "Click the button",
      commandPattern: {
        command: "semio.app.clickButton",
        origin: "semio.sketchpad.app.panel.button",
      },
      focusElement: {
        selector: "[data-button='my-button']",
        highlightMode: "spotlight",
      },
      canSkip: true,
      order: 0,
    },
    // ... more milestones
  ],
};
```

### Recording Interactions

```typescript
const store = useTutorialStore();

// Start recording
store.startRecording("My Recording");

// User performs actions...

// Stop recording
const recording = store.stopRecording();

// Convert to tutorial
const tutorial = store.convertRecordingToTutorial(
  recording,
  "Generated Tutorial",
  "Created from recording"
);

store.addTutorial(tutorial);
```

### Command Integration

The tutorial system automatically intercepts all commands executed in Sketchpad:

1. Checks if the command matches the current milestone's pattern
2. Records the command if recording is active
3. Advances to the next milestone if conditions are met

### UI Element Focus

To enable focus highlighting, add data attributes to your UI elements:

```tsx
<Button data-action="create-kit">Create Kit</Button>
```

Then reference in your milestone:

```typescript
focusElement: {
  selector: "[data-action='create-kit']",
  highlightMode: "spotlight",
}
```

## Architecture

### Store Hierarchy

```
SketchpadStore
  └── TutorialStore
      ├── Tutorial state (active tutorial, playback)
      ├── Recording state (active recording)
      └── Available tutorials
```

### Components

- **TutorialProvider** - React context provider for tutorial state
- **TutorialOverlay** - Visual overlay with focus and cursor effects
- **TutorialControls** - Playback controls in the footer
- **RecordingControls** - Recording controls in the footer

### State Management

Tutorial state is stored in Y.js (`yDoc.getMap("tutorials")`) for:
- Persistence across sessions
- Potential collaboration (future)
- Consistent state management

## Commands

All tutorial commands are prefixed with `semio.tutorial.` or `semio.recording.`:

- `semio.tutorial.start` - Start a tutorial
- `semio.tutorial.pause` - Pause playback
- `semio.tutorial.resume` - Resume playback
- `semio.tutorial.stop` - Stop and exit tutorial
- `semio.tutorial.nextMilestone` - Skip to next step
- `semio.tutorial.previousMilestone` - Go back one step
- `semio.tutorial.goToMilestone` - Jump to specific step
- `semio.recording.start` - Start recording
- `semio.recording.pause` - Pause recording
- `semio.recording.resume` - Resume recording
- `semio.recording.stop` - Stop recording
- `semio.tutorial.add` - Add a tutorial to the library
- `semio.tutorial.remove` - Remove a tutorial
- `semio.recording.convertToTutorial` - Convert recording to tutorial

## Example

See `exampleTutorial.ts` for a complete example of a tutorial with multiple milestones, focus elements, and cursor animations.

## Future Enhancements

- **Voice Narration** - Text-to-speech for milestone descriptions
- **Interactive Quizzes** - Test comprehension at key points
- **Branching Paths** - Multiple tutorial paths based on user choices
- **Collaborative Tutorials** - Multiple users in same tutorial
- **Tutorial Marketplace** - Share and discover tutorials
- **Analytics** - Track completion rates and pain points
- **Localization** - Multi-language tutorial support
