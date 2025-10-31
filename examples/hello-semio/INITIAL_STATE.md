# Initial State Feature

## Overview

The Sketchpad component now supports an `initialState` prop that allows you to pre-configure the application state on first load. This is useful for:

- **Example/Demo setups**: Pre-load kits, types, and designs to demonstrate features
- **Template projects**: Start users with a predefined kit and design structure
- **Testing**: Create consistent test scenarios with known state
- **Tutorials**: Pre-configure the app for tutorial walkthroughs

## Basic Usage

```tsx
import { Sketchpad } from "@semio/js/sketchpad";
import { Expertise, Mode } from "@semio/js/sketchpad/store";

const initialState = {
  expertise: Expertise.NORMAL,
  mode: Mode.USER,
  navigation: "/kits/my-kit",
  kits: [
    {
      kit: myKitDefinition,
      local: true,
      remote: false,
    },
  ],
};

function App() {
  return <Sketchpad id="my-app" initialState={initialState} />;
}
```

## Extended Initial State Structure

The `ExtendedInitialState` interface extends `Partial<SketchpadState>` with:

### Sketchpad Settings

```typescript
{
  // Navigation
  navigation?: string;  // Current path, e.g., "/kits/abc-123/designs/xyz-789"
  navigationHistory?: string[];
  navigationHistoryIndex?: number;
  
  // UI Settings
  access?: Access;  // USER or GUEST
  theme?: Theme;    // SYSTEM, LIGHT, or DARK
  layout?: Layout;  // NORMAL or TOUCH
  expertise?: Expertise;  // BEGINNER, NORMAL, or EXPERT
  mode?: Mode;  // USER or DEV
  
  // UI State
  isFullscreen?: boolean;
  isNavbarExpanded?: boolean;
  
  // App-specific settings
  appSettings?: {
    design?: {
      snappiness?: number;
      gridSize?: number;
    };
    type?: Record<string, any>;
    kit?: Record<string, any>;
  };
  
  // Panel sizes
  panelSizes?: {
    toolbarHeight?: number;
    workbenchWidth?: number;
    toolsWidth?: number;
    // ... other panel sizes
  };
  
  // Search & focus
  recentSearches?: string[];
  recentFocusItems?: Record<string, string[]>;
  
  // Hotkeys
  hotkeyOverrides?: Record<string, string>;
  activeHotkeySetting?: string;
}
```

### Kits

```typescript
{
  kits?: Array<{
    kit: Kit;      // Kit definition (types, designs, qualities, files, etc.)
    local?: boolean;   // Persist to IndexedDB (default: false)
    remote?: boolean;  // Sync to remote provider (default: false)
  }>;
}
```

## Kit Structure

A `Kit` includes:

- **guid**: Unique identifier
- **name**: Kit name
- **version**: Semantic version (e.g., "1.0.0")
- **types**: Array of Type definitions with representations and ports
- **designs**: Array of Design definitions with pieces and connections
- **qualities**: Array of Quality definitions for measurements
- **files**: Array of File resources
- **authors**: Array of author GUIDs
- **concepts**: Array of concept strings for categorization
- **metadata**: icon, image, description, attributes

### Type Structure

```typescript
{
  guid: string;
  name: string;
  variant?: string;
  representations?: Array<{
    guid: string;
    file: string;  // Path to 3D model or image
    tags?: string[];
    description?: string;
    attributes?: Attribute[];
  }>;
  ports?: Array<{
    guid: string;
    point: { x: number; y: number; z: number };
    direction: { x: number; y: number; z: number };
    t: number;  // Position on diagram ring (0-1)
    mandatory?: boolean;
    family?: string;
    compatibleFamilies?: string[];
    description?: string;
    attributes?: Attribute[];
  }>;
  props?: Prop[];
  stock?: number;  // Available quantity
  virtual?: boolean;  // Is intermediate type?
  unit?: string;  // "m", "cm", etc.
  location?: Location;
  authors?: string[];
  concepts?: string[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
}
```

### Design Structure

```typescript
{
  guid: string;
  name: string;
  variant?: string;
  view?: string;  // Serialized Camera JSON
  pieces?: Array<{
    guid: string;
    type?: string;  // Type GUID
    design?: string;  // Nested design GUID
    plane?: Plane;  // Fixed position/orientation
    center?: { x: number; y: number };  // Diagram position
    scale?: number;
    mirrorPlane?: Plane;
    isHidden?: boolean;
    isLocked?: boolean;
    color?: string;
    description?: string;
    attributes?: Attribute[];
  }>;
  connections?: Array<{
    guid: string;
    connected: { piece: string; port: string };
    connecting: { piece: string; port: string };
    gap?: number;      // Y-offset
    shift?: number;    // X-offset
    rise?: number;     // Z-offset
    rotation?: number; // Around Y-axis
    turn?: number;     // Around Z-axis
    tilt?: number;     // Around X-axis
    x?: number;        // Diagram X position
    y?: number;        // Diagram Y position
    description?: string;
    attributes?: Attribute[];
  }>;
  stats?: Stat[];
  props?: Prop[];
  layers?: Layer[];
  activeLayer?: string;
  groups?: Group[];
  unit?: string;
  location?: Location;
  authors?: string[];
  concepts?: string[];
  icon?: string;
  image?: string;
  description?: string;
  attributes?: Attribute[];
}
```

## Complete Example

See `examples/hello-semio/initial-state-example.ts` for a full working example that creates:

1. A kit with one type (Connector Block with 2 ports)
2. A design with two pieces positioned and ready to connect
3. Sketchpad configured to navigate directly to the design
4. Both pieces selected in "selection-normal" tool mode

## How It Works

1. **Initialization Order**:
   - SketchpadStore constructor is called with `initialState`
   - Y.js persistence layer loads (if `id` is provided)
   - Default values are set for missing fields
   - `initialState` values override defaults
   - Kits are created after all other state is initialized

2. **Kit Creation**:
   - Each kit in `initialState.kits` is created via `store.createKit()`
   - `local: true` enables IndexedDB persistence
   - `remote: true` enables sync with remote provider
   - Kit GUIDs are used in navigation paths

3. **Persistence**:
   - Initial state is applied on first load
   - After that, IndexedDB takes precedence
   - To reset, clear IndexedDB or use a different `id`

## Best Practices

1. **Generate Consistent GUIDs**: Use the same GUID generator for related entities
2. **Set Navigation Path**: Point to the specific kit/design you want to show
3. **Use Access.GUEST for Demos**: Prevent modifications in read-only scenarios
4. **Set Expertise Level**: Match the target user's skill level
5. **Minimal Initial State**: Only set values you need to override defaults

## TypeScript Support

All interfaces are fully typed:

- `ExtendedInitialState` - Root initial state interface
- `InitialStateKit` - Kit with local/remote flags
- `Kit`, `Type`, `Design`, `Piece`, `Connection` - Full entity types

Import types from `@semio/js/sketchpad/store` and `@semio/js/semio`.
