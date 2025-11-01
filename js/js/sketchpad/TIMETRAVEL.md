# Timetravel System Implementation

## Overview

The timetravel system provides complete state dump/load functionality for debugging, testing, and demonstration purposes. It allows you to freeze the entire application state to a JSON file and restore it later.

## Features

### 1. Complete State Snapshot (`CompleteState` interface)

The system captures **everything**:

```typescript
interface CompleteState {
  sketchpad: SketchpadState; // Navigation, settings, UI state
  kits: Array<{
    // All loaded kits
    guid: string;
    local: boolean; // Persistence flags
    remote: boolean;
    kit: Kit; // Full kit data
  }>;
  kitApps: Record<string, any>; // Kit editor states
  typeApps: Record<string, any>; // Type editor states
  qualityApps: Record<string, any>; // Quality editor states
  designApps: Record<string, Record<string, any>>; // Design editor states
  home?: any; // Home screen state
  tutorials: any; // Tutorial/recording state
}
```

### 2. Store Methods

#### `dumpState(): CompleteState`

Exports the complete application state including:

- Sketchpad settings (navigation, theme, layout, expertise, mode, etc.)
- All kits with their types, designs, qualities, files
- All active app states (kit apps, type apps, design apps, quality apps)
- Tutorial and recording state

#### `loadState(state: CompleteState): void`

Destructively loads a complete state:

- Clears all existing kits and apps
- Restores sketchpad settings
- Recreates all kits with correct local/remote flags
- Recreates and restores all app states
- Restores tutorial state

**Warning**: This is a destructive operation with NO validation. Use only for debugging and development.

### 3. Dev Commands

#### `semio.sketchpad.freeze`

- **Action**: Downloads complete state as JSON file
- **Filename**: `semio-freeze-YYYY-MM-DDTHH-MM-SS.json`
- **Hotkey**: None (dev-only)
- **Location**: Footer left (order: -100)

#### `semio.sketchpad.timetravel`

- **Action**: Opens file picker to load state JSON
- **File type**: `.json` only
- **Hotkey**: None (dev-only)
- **Location**: Footer left (order: -99)

### 4. Footer Buttons (Dev Mode Only)

Two new buttons appear in the footer when `mode === Mode.DEV`:

- **Freeze** (Download icon): Exports current state
- **Timetravel** (Upload icon): Loads state from file

Components:

- `FreezeButton.tsx` - Registers freeze button
- `TimetravelButton.tsx` - Registers timetravel button

## Usage

### Exporting State

1. Set mode to DEV:

   ```typescript
   store.execute("semio.sketchpad.setMode", origin, Mode.DEV);
   ```

2. Click the **Freeze** button (download icon) in the footer

3. A JSON file will be downloaded with the complete state

### Loading State

1. Ensure mode is DEV

2. Click the **Timetravel** button (upload icon) in the footer

3. Select a previously saved JSON file

4. The entire application state will be restored

## Storybook Integration

### Story with Complete State

The `HelloSemio` story demonstrates loading a complete state:

```typescript
export const HelloSemio: Story = {
  args: {
    id: "hello-semio-story",
  },
  play: async ({ canvasElement }) => {
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const store = (window as any).__SEMIO_STORE__;
    if (store && store.loadState) {
      store.loadState(helloSemioInitialState);
    }
  },
};
```

The store is exposed on `window.__SEMIO_STORE__` for dev tools and Storybook access.

### Example State Structure

```typescript
const helloSemioInitialState: CompleteState = {
  sketchpad: {
    navigation: "/kits/hello-kit/designs/hello-design",
    expertise: Expertise.NORMAL,
    mode: Mode.DEV,
    // ... all other settings
  },
  kits: [
    {
      guid: "hello-kit",
      local: true,
      remote: false,
      kit: {
        name: "Hello Semio Kit",
        types: [
          /* type definitions */
        ],
        designs: [
          /* design definitions */
        ],
        // ... complete kit structure
      },
    },
  ],
  designApps: {
    "hello-kit": {
      "hello-design": {
        selectedPieces: ["piece-1", "piece-2"],
        activeTool: "selection-normal",
        // ... complete design app state
      },
    },
  },
  // ... other app states
};
```

## Technical Implementation

### Store Hierarchy

All stores now support timetravel:

1. **SketchpadStore**: Top-level state manager
   - `dumpState()` - Collects all substores
   - `loadState()` - Restores all substores

2. **KitStore**: Per-kit data (inherited snapshot)

3. **App Stores**: Design, Type, Kit, Quality, Home
   - Each should implement `loadState()` for full restore

### Y.js Integration

- State is read from Y.js structures via `snapshot()`
- Loading writes directly to Y.js maps/arrays
- IndexedDB automatically syncs changes
- Remote providers sync if enabled

### Command Execution

The freeze/timetravel commands are handled specially in `executeCommand`:

```typescript
if (command === "semio.sketchpad.freeze") {
  const completeState = this.dumpState();
  // ... download as JSON
}

if (command === "semio.sketchpad.timetravel") {
  // ... file picker
  const state = JSON.parse(text);
  this.loadState(state);
}
```

## Files Changed

### Core

- `js/js/sketchpad/store.tsx` - Added CompleteState, dumpState, loadState, window exposure
- `js/js/sketchpad/commands.ts` - Added freeze/timetravel dev commands

### UI Components

- `js/js/sketchpad/panels/FreezeButton.tsx` - New freeze button component
- `js/js/sketchpad/panels/TimetravelButton.tsx` - New timetravel button component
- `js/js/sketchpad/Sketchpad.tsx` - Integrated freeze/timetravel buttons

### Storybook

- `js/js/sketchpad/Sketchpad.stories.tsx` - Added HelloSemio story with state loading

### Localization

- `js/js/locales/en.json` - Added footer.freeze and footer.timetravel translations
- `js/js/locales/de.json` - Added German translations

### Examples

- `examples/hello-semio/initial-state-example.ts` - Complete state example (kept for reference)
- `examples/hello-semio/INITIAL_STATE.md` - Documentation

## Best Practices

1. **Always use Dev Mode**: Freeze/timetravel only available in `Mode.DEV`

2. **Version your states**: Include metadata in filenames or JSON for tracking

3. **Test state compatibility**: State structure may change between versions

4. **Use for debugging**: Primary use case is reproducing issues

5. **Clear IndexedDB**: If state becomes corrupted, clear browser storage

6. **Don't commit state files**: Add `*.semio-freeze.json` to `.gitignore`

## Future Enhancements

- [ ] State validation before loading
- [ ] Partial state loading (selective restore)
- [ ] State diffing and comparison
- [ ] State migration for version compatibility
- [ ] Compressed state format
- [ ] Cloud backup/sync of states
- [ ] Named state snapshots (not just files)
- [ ] Undo/redo integration with timetravel

## Troubleshooting

**State won't load**: Check browser console for JSON parse errors

**Missing features after load**: Ensure all app stores implement `loadState()`

**Performance issues**: Large states (many kits/designs) may take time to load

**State drift**: After loading, changes persist to IndexedDB automatically
