# Kit.tsx Deep Dive Tutorial

> A comprehensive guide to understanding the Kit Application - the central "brain" for managing digital building sets in semio's Sketchpad.

---

## Table of Contents

1. [Big Picture Overview](#1-big-picture-overview)
2. [Architecture & Relationships](#2-architecture--relationships)
3. [Responsibilities of This File](#3-responsibilities-of-this-file)
4. [Deep Breakdown of Components](#4-deep-breakdown-of-components)
5. [Simplified Learning Examples](#5-simplified-learning-examples)
6. [Execution Flow Walkthrough](#6-execution-flow-walkthrough)
7. [Common Beginner Mistakes](#7-common-beginner-mistakes)
8. [Diagram Window Deep Dive](#8-diagram-window-deep-dive)
9. [Glossary of Terms](#9-glossary-of-terms)
10. [Final Summary](#10-final-summary)

---

## 1. Big Picture Overview

### What is Kit.tsx?

**Kit.tsx** is the central nervous system for managing a **Kit** in the Sketchpad application. Think of it as a sophisticated file manager, but instead of managing files and folders on your computer, it manages **digital building sets** for architecture and design.

### The Business Problem It Solves

Imagine you're an architect working with a modular building system (like LEGO for buildings). You have:
- **Types**: The blueprints/templates (like "Wall", "Window", "Door")
- **Designs**: The actual arrangements of pieces (like "Kitchen Layout", "Office Floor Plan")
- **Qualities**: Measurement definitions (like "Area", "Height", "Weight")
- **Ports**: Connection compatibility rules (how pieces can connect)
- **Tags**: Labels for categorizing things
- **Concepts**: Semantic groupings
- **Files**: Assets like 3D models, images
- **Folders**: Organizational containers
- **Authors**: People who contributed

Kit.tsx lets you:
1. **View** all these artifacts in a table or diagram
2. **Create** new types, designs, qualities, etc.
3. **Edit** their properties
4. **Organize** them into folders with drag-and-drop
5. **Filter** and **search** to find what you need
6. **Select** and **hover** with visual feedback
7. **Navigate** to edit individual types or designs
8. **Collaborate** in real-time with other users

### Where Does It Fit in the Architecture?

```
┌─────────────────────────────────────────────────────────────┐
│                    SKETCHPAD APPLICATION                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐    │
│  │  Home   │──▶│   Kit   │──▶│ Design  │   │  Type   │    │
│  │  App    │   │   App   │   │   App   │   │   App   │    │
│  └─────────┘   └────┬────┘   └─────────┘   └─────────┘    │
│                     │                                       │
│        ┌────────────┴────────────┐                         │
│        ▼                         ▼                         │
│  ┌──────────────┐    ┌───────────────────┐                 │
│  │  KitStore    │    │  XState Machine   │                 │
│  │  (Y.js CRDT) │◀──▶│  (UI State)       │                 │
│  └──────────────┘    └───────────────────┘                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Navigation Flow:**
1. **Home App** → Lists all kits (temporary, local, remote)
2. **Kit App** → Where you manage ONE kit's contents (**THIS FILE**)
3. **Design App** → Where you edit a specific design's pieces
4. **Type App** → Where you edit a specific type's connectors

---

## 2. Architecture & Relationships

### The Three Pillars of Kit.tsx

```
┌─────────────────────────────────────────────────────────────┐
│                      Kit.tsx Architecture                    │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              1. STATE MANAGEMENT                     │   │
│  │                                                      │   │
│  │  ┌──────────────┐        ┌───────────────────────┐  │   │
│  │  │   KitStore   │        │   XState Machine      │  │   │
│  │  │   (Y.js)     │◀──────▶│   (sketchpadMachine)  │  │   │
│  │  │              │  sync  │                       │  │   │
│  │  │ • Kit data   │        │ • UI state (hover,    │  │   │
│  │  │ • Types      │        │   selection, panels)  │  │   │
│  │  │ • Designs    │        │ • Navigation          │  │   │
│  │  │ • Qualities  │        │ • Transactions        │  │   │
│  │  └──────────────┘        └───────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              2. COMMANDS & HOOKS                     │   │
│  │                                                      │   │
│  │  ┌──────────────┐        ┌───────────────────────┐  │   │
│  │  │  commands    │        │  React Hooks          │  │   │
│  │  │  object      │◀──────▶│                       │  │   │
│  │  │              │        │ • useKitApp()         │  │   │
│  │  │ • select*    │        │ • useKitAppSelection  │  │   │
│  │  │ • add*       │        │ • useKitAppCommands   │  │   │
│  │  │ • update*    │        │ • useKitAppHover      │  │   │
│  │  │ • remove*    │        │ • ...50+ more hooks   │  │   │
│  │  └──────────────┘        └───────────────────────┘  │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              3. UI COMPONENTS                        │   │
│  │                                                      │   │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────────┐ │   │
│  │  │   Table    │  │  Diagram   │  │    Panels      │ │   │
│  │  │   View     │  │   View     │  │                │ │   │
│  │  │            │  │            │  │ • Details      │ │   │
│  │  │ • Rows     │  │ • Nodes    │  │ • Settings     │ │   │
│  │  │ • Sorting  │  │ • Edges    │  │ • Chat         │ │   │
│  │  │ • DnD      │  │ • Force    │  │ • Toolbar      │ │   │
│  │  │ • Expand   │  │   Layout   │  │                │ │   │
│  │  └────────────┘  └────────────┘  └────────────────┘ │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### File Dependencies (What Kit.tsx Imports)

```
┌─────────────────────────────────────────────────────────────┐
│                    IMPORT ARCHITECTURE                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  React Ecosystem:                                           │
│  ├── react (useState, useEffect, useMemo, useCallback...)  │
│  ├── react-router (useParams, useNavigate, useSearchParams)│
│  ├── @xstate/react (useSelector, useActor)                 │
│  └── react-i18next (useTranslation)                        │
│                                                             │
│  Y.js (Real-time Collaboration):                            │
│  ├── yjs (Y.Doc, Y.Map, Y.Array)                           │
│  └── y-indexeddb (persistence)                              │
│                                                             │
│  UI Libraries:                                               │
│  ├── @dnd-kit/core (drag and drop)                         │
│  ├── @xyflow/react (diagram/graph visualization)           │
│  ├── d3-force (physics simulation for diagram layout)      │
│  └── date-fns (date formatting)                            │
│                                                             │
│  Internal Modules:                                           │
│  ├── ../semio.ts (domain types: Kit, Type, Design...)      │
│  ├── ./Sketchpad.tsx (base stores, shared components)      │
│  ├── ./shared.ts (plugin system, event handlers)           │
│  └── ./elements.tsx (UI primitives: Table, Canvas...)      │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Key Design Patterns Used

| Pattern | What It Does | Where in Kit.tsx |
|---------|--------------|------------------|
| **Plugin Architecture** | Apps register themselves; Sketchpad doesn't know about specific apps | `kitAppPlugin` object |
| **Command Pattern** | All state changes go through named commands | `commands` object |
| **Triadic Hooks** | All hooks return `[value, setter, canSet]` | `useKitAppSelection()` |
| **CRDT Sync** | Y.js data syncs with XState machine | `useKitAppYjsToXStateSync()` |
| **Scope Providers** | React context provides current kit/design/type GUIDs | `KitScopeProvider` |

---

## 3. Responsibilities of This File

### What Kit.tsx DOES (Its Jobs)

1. **State Management**
   - Defines `KitAppState` interface (what the app tracks)
   - Defines `KitAppSelection` (which items are selected)
   - Defines `KitAppHover` (which item is being hovered)
   - Manages panel visibility, sorting, filtering, expanded rows

2. **Plugin Registration**
   - Registers `kitAppPlugin` with the Sketchpad machine
   - Defines event handlers for `KIT.*` events
   - Creates default state for new kit apps

3. **Commands**
   - 50+ command handlers for all operations
   - Selection: `selectType`, `deselectType`, `selectAll`, etc.
   - CRUD: `createType`, `updateType`, `removeType`, etc.
   - UI: `togglePanel`, `setSortColumn`, `toggleRow`, etc.

4. **React Hooks**
   - 30+ hooks for accessing and modifying state
   - Follow triadic pattern: `[value, setter, canSet]`
   - Examples: `useKitAppSelection`, `useKitAppHover`, `useKitAppCommands`

5. **UI Components**
   - `AppContent` - Main table/list view with hierarchical rows
   - `KitDiagram` - Force-directed graph view
   - `KitDropZone` - Drag and drop wrapper
   - `KitToolbarFilters` - Filter toggles in toolbar
   - Panel sections: `KitSection`, `TypeSection`, `DesignSection`, etc.

6. **Hierarchical Data Building**
   - Builds flat list of `TableRow` from nested kit data
   - Handles parent/child relationships for types and designs
   - Handles folder organization
   - Supports expand/collapse

7. **Drag and Drop**
   - Move artifacts between folders
   - Re-parent types and designs
   - Drop files to upload

### What Kit.tsx DOES NOT DO (Delegated Elsewhere)

- **Actual kit data storage** → `KitStore` in Sketchpad.tsx
- **3D rendering** → Design App and Type App
- **Navigation state** → XState machine in Sketchpad.tsx
- **Theming/styling** → globals.css, theme.css
- **i18n translations** → locales/en.json, locales/de.json

---

## 4. Deep Breakdown of Components

### 4.1 State Interfaces

```typescript
// ═══════════════════════════════════════════════════════════════
// WHAT THE KIT APP TRACKS
// ═══════════════════════════════════════════════════════════════

// Selection: Which items are currently selected (highlighted in blue)
// This is an OBJECT with arrays of GUIDs for each artifact type
interface KitAppSelection {
  types?: Guid[];           // Selected type GUIDs (e.g., ["abc-123", "def-456"])
  designs?: Guid[];         // Selected design GUIDs
  qualities?: string[];     // Selected quality keys
  ports?: Guid[];           // Selected port GUIDs
  tags?: Guid[];            // Selected tag GUIDs
  concepts?: Guid[];        // Selected concept GUIDs
  files?: string[];         // Selected file GUIDs
  folders?: Guid[];         // Selected folder GUIDs
  authors?: string[];       // Selected author names
}

// Hover: Which single item is being hovered (highlighted differently)
// Only ONE thing can be hovered at a time
interface KitAppHover {
  type?: Guid;              // Hovered type GUID
  design?: Guid;            // Hovered design GUID
  quality?: string;         // Hovered quality key
  port?: Guid;              // Hovered port GUID
  tag?: Guid;               // Hovered tag GUID
  concept?: Guid;           // Hovered concept GUID
  file?: string;            // Hovered file GUID
  folder?: Guid;            // Hovered folder GUID
  author?: string;          // Hovered author name
}

// Full app state: Everything the Kit App needs to function
interface KitAppState {
  panelVisibility: PanelVisibility;    // Which panels are open
  selection?: KitAppSelection;          // Currently selected items
  hover?: KitAppHover;                  // Currently hovered item
  fullscreenWindow: string;             // "none" or window kind
  filterSearch?: string;                // Search box text
  expandedRows?: Set<string>;           // Which tree rows are expanded
  sortColumn?: SortColumn;              // Which column is sorted
  sortDirection?: "asc" | "desc";       // Sort direction
  windowLayout?: any;                   // Golden Layout config
  diagramForce?: DiagramForceSettings;  // Force simulation params
}
```

### 4.2 The Plugin System

```typescript
// ═══════════════════════════════════════════════════════════════
// HOW KIT APP REGISTERS WITH SKETCHPAD
// ═══════════════════════════════════════════════════════════════

// The plugin object tells Sketchpad:
// 1. What events this app handles
// 2. What state shape this app uses
// 3. What actions to run when events occur

export const kitAppPlugin: AppPlugin = {
  // Unique identifier for this app
  id: "kit",
  
  // Event prefix - all events start with "KIT."
  namespace: "KIT",
  
  // Machine contributions
  machine: {
    // Actions that modify state
    actions: {
      // Example: When KIT.SELECT_TYPE fires, run this action
      kitSelectType: assign({
        kitApp: (context, event) => {
          // Add the type GUID to selection
          const prev = context.kitApp?.selection?.types ?? [];
          return {
            ...context.kitApp,
            selection: {
              ...context.kitApp?.selection,
              types: [...prev, event.typeGuid],
            },
          };
        },
      }),
    },
    
    // Guards that check conditions
    guards: {
      // Example: Only allow selection if we're in the kit app
      isInKitApp: (context) => context.navigation === "kit",
    },
    
    // Default state when kit app first loads
    createDefaultState: () => ({
      panelVisibility: defaultPanelVisibility,
      selection: undefined,
      hover: undefined,
      expandedRows: new Set(),
      sortColumn: undefined,
      sortDirection: undefined,
    }),
  },
};
```

### 4.3 The Commands Object

The `commands` object is the **control panel** for all Kit App operations:

```typescript
// ═══════════════════════════════════════════════════════════════
// COMMAND PATTERN: ALL MUTATIONS GO THROUGH NAMED COMMANDS
// ═══════════════════════════════════════════════════════════════

// WHY use commands instead of direct state mutation?
// 1. Centralized - all changes in one place
// 2. Auditable - can log every operation
// 3. Undoable - can record inverse operations
// 4. Testable - can call commands in tests
// 5. Origin tracking - know which UI element triggered it

const commands = {
  // ─────────────────────────────────────────────────────────
  // SELECTION COMMANDS
  // ─────────────────────────────────────────────────────────
  
  selectType: (typeGuid: Guid) => {
    // Sends event to XState machine
    // Machine updates context.kitApp.selection.types
    actor.send({ type: "KIT.SELECT_TYPE", kitGuid, typeGuid });
  },
  
  deselectType: (typeGuid: Guid) => {
    actor.send({ type: "KIT.DESELECT_TYPE", kitGuid, typeGuid });
  },
  
  selectAll: () => {
    // Select everything visible
    actor.send({ type: "KIT.SELECT_ALL", kitGuid });
  },
  
  deselectAll: () => {
    // Clear all selection
    actor.send({ type: "KIT.DESELECT_ALL", kitGuid });
  },
  
  // ─────────────────────────────────────────────────────────
  // CRUD COMMANDS (Create, Read, Update, Delete)
  // ─────────────────────────────────────────────────────────
  
  createType: (type: Type) => {
    // Calls the underlying KitStore method
    kitStore.addType(type);
  },
  
  updateType: (typeGuid: Guid, changes: Partial<Type>) => {
    kitStore.updateType(typeGuid, changes);
  },
  
  removeType: (typeGuid: Guid) => {
    kitStore.removeType(typeGuid);
  },
  
  // ─────────────────────────────────────────────────────────
  // UI STATE COMMANDS
  // ─────────────────────────────────────────────────────────
  
  togglePanel: (panel: PanelKind) => {
    actor.send({ type: "KIT.TOGGLE_PANEL", kitGuid, panel });
  },
  
  setSortColumn: (column: SortColumn) => {
    actor.send({ type: "KIT.SET_SORT_COLUMN", kitGuid, column });
  },
  
  toggleRow: (rowId: string) => {
    // Expand or collapse a tree row
    actor.send({ type: "KIT.TOGGLE_ROW", kitGuid, rowId });
  },
  
  // ... 40+ more commands
};
```

### 4.4 The Triadic Hook Pattern

All hooks follow the same pattern: `[value, setter, canSet]`

```typescript
// ═══════════════════════════════════════════════════════════════
// TRIADIC HOOK PATTERN: [value, setter, canSet]
// ═══════════════════════════════════════════════════════════════

// WHY three values?
// 1. value - what the current state is
// 2. setter - function to change it (undefined if can't change)
// 3. canSet - boolean to disable UI elements

function useKitAppSelection(): HookResult<KitAppSelection | undefined> {
  const actor = useSketchpadActor();
  const kitGuid = useKitScope()?.guid ?? "";
  
  // Read current selection from XState machine
  const selection = useSelector(
    actor,
    (state) => state.context.kitApp?.[kitGuid]?.selection
  );
  
  // Check if we CAN set selection (are we in the right state?)
  const canSet = useSelector(
    actor,
    (state) => state.can({ type: "KIT.SET_SELECTION", kitGuid })
  );
  
  // Create setter function (only if allowed)
  const setSelection = useMemo(() => {
    if (!canSet) return undefined;
    return (newSelection: KitAppSelection) => {
      actor.send({ type: "KIT.SET_SELECTION", kitGuid, selection: newSelection });
    };
  }, [actor, kitGuid, canSet]);
  
  // Return the triadic tuple
  return [selection, setSelection, canSet];
}

// USAGE IN COMPONENTS:
function MyComponent() {
  const [selection, setSelection, canSetSelection] = useKitAppSelection();
  
  return (
    <Button
      onClick={() => setSelection?.({ types: ["abc-123"] })}
      disabled={!canSetSelection}  // Gray out if can't select
    >
      Select Type
    </Button>
  );
}
```

### 4.5 Building Table Rows

The most complex part of Kit.tsx is building the hierarchical table data:

```typescript
// ═══════════════════════════════════════════════════════════════
// HIERARCHICAL ROW BUILDING
// ═══════════════════════════════════════════════════════════════

// The challenge: We have NESTED data (types with subtypes, designs
// with subdesigns, folders containing things) but we need a FLAT
// list for the table.

interface TableRow {
  id: string;           // Unique row identifier (e.g., "type-abc-123")
  kind: ArtifactKind;   // "types" | "designs" | "qualities" | etc.
  artifact: string;     // Display name
  level: number;        // Nesting depth (0 = root, 1 = child, etc.)
  hasChildren: boolean; // Does this row have children?
  isExpanded: boolean;  // Is this row expanded?
  data: any;            // The actual Type/Design/Quality/etc. object
  updatedAt?: string;   // Formatted date
  createdAt?: string;   // Formatted date
  concepts?: string[];  // Concept names for filtering
}

// The algorithm:
// 1. Group artifacts by their parent/folder
// 2. Build tree structure
// 3. Flatten to list respecting expand/collapse
// 4. Apply search filter
// 5. Apply sort

function buildRows(kit: Kit, expandedRows: Set<string>, filterSearch: string): TableRow[] {
  const result: TableRow[] = [];
  
  // Build lookup maps for fast parent finding
  const typesByParent = new Map<string | null, Type[]>();
  const designsByParent = new Map<string | null, Design[]>();
  const foldersByParent = new Map<string | null, Folder[]>();
  
  // Group types by parent
  for (const type of kit.types ?? []) {
    const parentKey = type.parent?.guid ?? type.folder ?? null;
    if (!typesByParent.has(parentKey)) {
      typesByParent.set(parentKey, []);
    }
    typesByParent.get(parentKey)!.push(type);
  }
  
  // Recursive function to add a type and its children
  function addTypeAndChildren(type: Type, level: number) {
    const rowId = `type-${type.guid}`;
    const children = typesByParent.get(type.guid) ?? [];
    const hasChildren = children.length > 0;
    const isExpanded = expandedRows.has(rowId);
    
    // Apply search filter
    if (filterSearch && !type.name.toLowerCase().includes(filterSearch.toLowerCase())) {
      // Skip this row if it doesn't match search
      // BUT still check children (they might match)
      if (isExpanded) {
        for (const child of children) {
          addTypeAndChildren(child, level + 1);
        }
      }
      return;
    }
    
    // Add this row
    result.push({
      id: rowId,
      kind: "types",
      artifact: type.name,
      level,
      hasChildren,
      isExpanded,
      data: type,
      updatedAt: formatDate(type.updatedAt),
      createdAt: formatDate(type.createdAt),
      concepts: type.concepts?.map(c => c.name),
    });
    
    // Add children if expanded
    if (isExpanded) {
      for (const child of children) {
        addTypeAndChildren(child, level + 1);
      }
    }
  }
  
  // Start with root-level types (no parent, no folder)
  for (const type of typesByParent.get(null) ?? []) {
    addTypeAndChildren(type, 0);
  }
  
  // Similar logic for designs, folders, qualities, etc.
  // ...
  
  return result;
}
```

### 4.6 The Diagram View

Kit.tsx includes a force-directed graph visualization:

```typescript
// ═══════════════════════════════════════════════════════════════
// FORCE-DIRECTED DIAGRAM
// ═══════════════════════════════════════════════════════════════

// Uses D3 force simulation to position nodes
// Uses React Flow to render and interact with the graph

// Force simulation parameters (user-adjustable)
interface DiagramForceSettings {
  chargeStrength: number;  // How much nodes repel each other (-300 default)
  linkDistance: number;    // Target distance between connected nodes (100)
  collideRadius: number;   // Minimum distance between node centers (50)
  centerStrength: number;  // How strongly nodes are pulled to center (0.1)
}

// Build nodes and edges from kit data
function buildKitDiagramData(kit: Kit): { nodes: Node[], edges: Edge[] } {
  const nodes: Node[] = [];
  const edges: Edge[] = [];
  
  // Add nodes for each artifact type
  for (const type of kit.types ?? []) {
    nodes.push({
      id: type.guid,
      type: "artifact",
      position: { x: 0, y: 0 },  // Will be set by simulation
      data: { guid: type.guid, name: type.name, kind: "type" },
    });
    
    // Add edge to parent if exists
    if (type.parent?.guid) {
      edges.push({
        id: `${type.parent.guid}-${type.guid}`,
        source: type.parent.guid,
        target: type.guid,
        type: "floating",
        style: { stroke: "var(--foreground)", strokeWidth: 2 },
        data: { relationship: "part-of" },
      });
    }
  }
  
  // Add "reference" edges for type usage in designs
  for (const design of kit.designs ?? []) {
    for (const piece of design.pieces ?? []) {
      if (piece.type?.guid) {
        edges.push({
          id: `ref-${design.guid}-${piece.type.guid}`,
          source: piece.type.guid,
          target: design.guid,
          type: "floating",
          style: { stroke: "var(--foreground)", strokeWidth: 1, strokeDasharray: "5,5" },
          data: { relationship: "reference" },
        });
      }
    }
  }
  
  return { nodes, edges };
}
```

---

## 5. Simplified Learning Examples

### Example 1: Minimal Selection Hook

```typescript
// ═══════════════════════════════════════════════════════════════
// SIMPLIFIED: How a selection hook works
// ═══════════════════════════════════════════════════════════════

// This is a SIMPLIFIED version to understand the concept.
// The real implementation has more edge cases.

function useSimpleSelection() {
  // Step 1: Get the XState actor (state machine)
  const actor = useSketchpadActor();
  
  // Step 2: Read state using a selector
  const selection = useSelector(actor, state => state.context.selection);
  
  // Step 3: Create a function to change it
  const setSelection = (newSelection) => {
    actor.send({ type: "SET_SELECTION", selection: newSelection });
  };
  
  // Step 4: Return value and setter
  return [selection, setSelection];
}

// Usage:
function SelectButton() {
  const [selection, setSelection] = useSimpleSelection();
  
  return (
    <button onClick={() => setSelection({ types: ["my-type-id"] })}>
      Select My Type
    </button>
  );
}
```

### Example 2: Minimal Command Handler

```typescript
// ═══════════════════════════════════════════════════════════════
// SIMPLIFIED: How a command works
// ═══════════════════════════════════════════════════════════════

// Commands are just functions that dispatch events

const simpleCommands = {
  // Toggle a panel open/closed
  togglePanel: (panelName) => {
    actor.send({ 
      type: "KIT.TOGGLE_PANEL", 
      panel: panelName 
    });
  },
  
  // Create a new type
  createType: (name) => {
    // 1. Generate a new unique ID
    const newType = {
      guid: generateGuid(),
      name: name,
      connectors: [],
    };
    
    // 2. Add to the kit store (which updates Y.js)
    kitStore.addType(newType);
    
    // 3. Select the new type
    actor.send({
      type: "KIT.SELECT_TYPE",
      typeGuid: newType.guid,
    });
    
    return newType;
  },
};
```

### Example 3: Minimal Table Row Building

```typescript
// ═══════════════════════════════════════════════════════════════
// SIMPLIFIED: How table rows are built
// ═══════════════════════════════════════════════════════════════

function buildSimpleRows(items, expandedIds) {
  const rows = [];
  
  for (const item of items) {
    // Skip if doesn't match search (simplified)
    
    // Create the row
    rows.push({
      id: item.id,
      name: item.name,
      isExpanded: expandedIds.has(item.id),
      level: 0,  // Root level
    });
    
    // Add children if expanded
    if (expandedIds.has(item.id) && item.children) {
      for (const child of item.children) {
        rows.push({
          id: child.id,
          name: child.name,
          isExpanded: expandedIds.has(child.id),
          level: 1,  // Child level
        });
      }
    }
  }
  
  return rows;
}
```

---

## 6. Execution Flow Walkthrough

### Flow 1: User Clicks a Type Row

```
┌─────────────────────────────────────────────────────────────┐
│                  CLICK FLOW: Select a Type                   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. User clicks row in Table                                │
│     │                                                       │
│     ▼                                                       │
│  2. handleRowClick(row, index, event) called                │
│     │                                                       │
│     ├─── if Shift held: Range select                        │
│     ├─── if Ctrl/Cmd held: Toggle selection                 │
│     └─── else: Single select (with debounce for dbl-click)  │
│          │                                                  │
│          ▼                                                  │
│  3. setSelectionAction({ types: [typeGuid] })               │
│     │                                                       │
│     ▼                                                       │
│  4. actor.send({ type: "KIT.SET_SELECTION", ... })          │
│     │                                                       │
│     ▼                                                       │
│  5. XState machine runs kitSetSelection action              │
│     │                                                       │
│     ▼                                                       │
│  6. Context updated: kitApp.selection = { types: [...] }    │
│     │                                                       │
│     ▼                                                       │
│  7. useSelector triggers re-render                          │
│     │                                                       │
│     ▼                                                       │
│  8. Row gets "selected" CSS class                           │
│     │                                                       │
│     ▼                                                       │
│  9. Details panel shows type properties                     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Flow 2: User Creates a New Type

```
┌─────────────────────────────────────────────────────────────┐
│                  CREATE FLOW: New Type                       │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. User clicks "+" button in toolbar                       │
│     │                                                       │
│     ▼                                                       │
│  2. handleCreateArtifact("types") called                    │
│     │                                                       │
│     ▼                                                       │
│  3. Generate unique name: "New Type", "New Type 2", ...     │
│     │                                                       │
│     ▼                                                       │
│  4. Create Type object with guid()                          │
│     │                                                       │
│     ▼                                                       │
│  5. kitCommands.createType(newType)                         │
│     │                                                       │
│     ▼                                                       │
│  6. KitStore.addType(type)                                  │
│     │                                                       │
│     ├─── Y.js transaction starts                            │
│     ├─── type pushed to yTypes array                        │
│     └─── Y.js transaction commits                           │
│          │                                                  │
│          ▼                                                  │
│  7. Y.js observer fires                                     │
│     │                                                       │
│     ▼                                                       │
│  8. KIT.SYNC event sent to XState                           │
│     │                                                       │
│     ▼                                                       │
│  9. Selection updated to new type                           │
│     │                                                       │
│     ▼                                                       │
│  10. Table re-renders with new row                          │
│     │                                                       │
│     ▼                                                       │
│  11. (Optional) Navigate to Type App                        │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Flow 3: Drag and Drop Type into Folder

```
┌─────────────────────────────────────────────────────────────┐
│                  DRAG FLOW: Move Type to Folder              │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  1. User starts dragging a type row                         │
│     │                                                       │
│     ▼                                                       │
│  2. DndKit onDragStart fires                                │
│     │                                                       │
│     ├─── setActiveId(row.id)                                │
│     └─── Drag overlay appears                               │
│          │                                                  │
│          ▼                                                  │
│  3. User drags over folder row                              │
│     │                                                       │
│     ▼                                                       │
│  4. Folder row highlights as drop target                    │
│     │                                                       │
│     ▼                                                       │
│  5. User drops                                              │
│     │                                                       │
│     ▼                                                       │
│  6. handleDragEnd(event) called                             │
│     │                                                       │
│     ├─── Extract dragged row from event.active              │
│     ├─── Extract target row from event.over                 │
│     │                                                       │
│     ├─── Validate: Can this be dropped here?                │
│     │    └─── Types can go into folders or other types      │
│     │                                                       │
│     └─── if target is folder:                               │
│          │                                                  │
│          ▼                                                  │
│  7. kitCommands.updateType(typeGuid, { folder: folderGuid })│
│     │                                                       │
│     ▼                                                       │
│  8. Y.js update propagates                                  │
│     │                                                       │
│     ▼                                                       │
│  9. Table rebuilds with type now under folder               │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 7. Common Beginner Mistakes

### Mistake 1: Forgetting the Triadic Pattern

```typescript
// ❌ WRONG: Only destructuring two values
const [selection, setSelection] = useKitAppSelection();
// setSelection might be undefined! This will crash.

// ✅ CORRECT: Always destructure all three
const [selection, setSelection, canSet] = useKitAppSelection();
if (canSet && setSelection) {
  setSelection({ types: ["abc"] });
}

// ✅ EVEN BETTER: Use optional chaining
setSelection?.({ types: ["abc"] });
```

### Mistake 2: Direct State Mutation

```typescript
// ❌ WRONG: Mutating the selection object directly
const [selection] = useKitAppSelection();
selection.types.push("new-guid");  // MUTATION! React won't re-render!

// ✅ CORRECT: Use the setter with a new object
const [selection, setSelection] = useKitAppSelection();
setSelection?.({
  ...selection,
  types: [...(selection?.types ?? []), "new-guid"],
});
```

### Mistake 3: Missing Scope Provider

```typescript
// ❌ WRONG: Using kit hooks outside KitScopeProvider
function MyComponent() {
  const kit = useKit();  // Returns undefined! No scope!
  return <div>{kit.name}</div>;  // CRASH!
}

// ✅ CORRECT: Wrap in scope provider
function App() {
  return (
    <KitScopeProvider guid="kit-123">
      <MyComponent />  {/* Now useKit() works */}
    </KitScopeProvider>
  );
}
```

### Mistake 4: Unstable Selector References

```typescript
// ❌ WRONG: Inline selector creates new function every render
// This causes infinite re-render loops with useSyncExternalStore
const types = useSelector(actor, (state) => state.kitApp?.types ?? []);

// ✅ CORRECT: Define selector outside component or memoize
const selectTypes = (state) => state.kitApp?.types ?? EMPTY_TYPES;
const EMPTY_TYPES: Type[] = [];

function MyComponent() {
  const types = useSelector(actor, selectTypes);
}
```

### Mistake 5: Not Handling Loading States

```typescript
// ❌ WRONG: Assuming kit is always available
function MyComponent() {
  const kit = useKit() as Kit;  // Might be undefined!
  return <div>{kit.types.length} types</div>;  // CRASH!
}

// ✅ CORRECT: Handle loading/missing states
function MyComponent() {
  const kit = useKit();
  
  if (!kit) {
    return <LoadingSpinner />;
  }
  
  return <div>{kit.types?.length ?? 0} types</div>;
}
```

---

## 8. Diagram Window Deep Dive

### Overview

The Diagram Window provides a **visual force-directed graph** representation of your kit's artifacts and their relationships. Unlike the table view which shows a flat hierarchical list, the diagram reveals the **connection structure** between artifacts using physics-based positioning.

### Core Concepts

#### 1. **Node Types**

Every artifact in your kit becomes a node in the diagram:

| Node Kind | Represents | Visual |
|-----------|------------|--------|
| `type` | Type definitions (blueprints) | Avatar with type icon |
| `design` | Design compositions | Avatar with design icon |
| `quality` | Quality definitions | Avatar with quality icon |
| `port` | Connection ports | Avatar with port icon |
| `tag` | Model tags | Avatar with tag icon |
| `concept` | Semantic concepts | Avatar with concept icon |
| `file` | Kit files | Avatar with file icon |
| `folder` | Organizational folders | Avatar with folder icon |
| `author` | Contributors | Avatar with author icon |

Each node displays:
- **Avatar** - Icon + name (first letter if no icon)
- **Selection ring** - Blue ring when selected
- **Hover ring** - Gray ring when hovered

#### 2. **Edge Types**

Edges represent relationships between artifacts:

| Relationship | Visual | Meaning | Example |
|-------------|--------|---------|---------|
| **part-of** | Solid line (width: 2) | Parent-child hierarchy | Design is child of another design |
| **reference** | Dashed line (width: 1) | Usage relationship | Design uses a type through its pieces |

**Edge routing:**
- Edges connect at the **closest point** on each node's circle
- Automatically calculate handle positions (Top/Bottom/Left/Right)
- Use Bezier curves for smooth paths

#### 3. **Force Simulation**

The diagram uses D3's force-directed layout with configurable physics:

```typescript
interface DiagramForceSettings {
  chargeStrength: -80;   // Node repulsion (negative = push apart)
  linkDistance: 60;      // Target edge length in pixels
  collideRadius: 30;     // Collision detection radius
  centerStrength: 0.15;  // Gravity toward center (0-1)
}
```

**How it works:**
1. **Initialization** - Nodes start in a circular pattern
2. **120 tick simulation** - Runs synchronously on mount
3. **Forces applied:**
   - **Charge force** - Nodes repel each other
   - **Link force** - Edges pull connected nodes together
   - **Collide force** - Prevents node overlap
   - **Center force** - Pulls entire graph toward center
4. **Manual positioning** - Drag nodes to override physics

### Features

#### 🎯 **Selection**

Click any node to select its artifact:
- **Single click** - Selects the node's artifact
- **Background click** - Deselects all
- **Visual feedback** - Blue ring appears around selected nodes
- **Synced with table** - Selection updates in both views

#### 🖱️ **Hover**

Mouse over nodes to highlight them:
- **Hover ring** - Gray ring appears on hover
- **Tooltip** - Shows artifact name
- **Synced with table** - Hovering in diagram highlights table row
- **Clear on exit** - Hover state clears when mouse leaves

#### 🎨 **Drag & Drop**

Reposition nodes manually:
- **Grab node** - Cursor changes to `grab`
- **Drag** - Cursor changes to `grabbing`, node follows mouse
- **Release** - Node stays in new position
- **Physics disabled** - Simulation doesn't run during drag

#### 🔍 **Filtering**

The diagram respects filter settings from the toolbar:

**Search filter:**
- Type in search box
- Only nodes matching the search term appear
- Case-insensitive matching

**Expanded rows:**
- Only nodes whose ancestors are expanded in the table view appear
- Collapse a folder in the table → its contents disappear from diagram
- This creates a "focus mode" for specific parts of your kit

**Kind filters:**
- Toggle artifact kinds (types, designs, qualities, etc.)
- Hidden kinds are removed from diagram
- Edges connecting to hidden nodes are also removed

#### 🔗 **Relationship Visualization**

**Part-of relationships (solid lines):**
- Type → Parent Type (type hierarchy)
- Design → Parent Design (design hierarchy)
- File → Folder (containment)
- Folder → Parent Folder (folder hierarchy)

**Reference relationships (dashed lines):**
- Type → Design (design uses type via pieces)
- Design → Design (nested design reference)

**Automatic edge calculation:**
- Edges only appear if both source and target nodes are visible
- No duplicate edges (checked by ID)
- Edge style automatically applied based on relationship type

#### 📐 **Layout & Navigation**

**Viewport controls:**
- **Pan** - Disabled by default (prevents accidental panning)
- **Zoom** - Disabled by default (prevents accidental zooming)
- **Fit view** - Automatically fits all nodes on first render
- **Background grid** - 20px grid for spatial reference

**Window integration:**
- **Split screen** - Default 50/50 split with table view
- **Fullscreen toggle** - Maximize diagram to full canvas
- **Layout persistence** - Window arrangement saved in Y.js
- **Multi-window** - Can open multiple diagram windows side-by-side

### Implementation Details

#### Data Flow

```
┌─────────────────────────────────────────────────────────┐
│                   Diagram Data Flow                      │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  1. Kit Data (Y.js)                                     │
│     ↓                                                   │
│  2. buildKitDiagramData()                               │
│     • Extract types, designs, qualities, etc.           │
│     • Create nodes for each artifact                    │
│     • Create edges for relationships                    │
│     ↓                                                   │
│  3. Filter by visibleGuids                              │
│     • Apply search filter                               │
│     • Apply expanded rows filter                        │
│     ↓                                                   │
│  4. D3 Force Simulation                                 │
│     • Position nodes using physics                      │
│     • Run 120 ticks synchronously                       │
│     ↓                                                   │
│  5. React Flow Rendering                                │
│     • Render nodes as KitArtifactNode                   │
│     • Render edges as FloatingEdge                      │
│     • Handle interactions (click, hover, drag)          │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

#### Key Components

**1. KitDiagram**
- Wrapper providing React Flow context
- Ensures single provider instance

**2. KitDiagramInner**
- Main logic component
- Manages force simulation
- Handles node/edge state
- Processes interactions

**3. KitArtifactNode**
- Custom React Flow node type
- Renders avatar with selection/hover state
- Invisible handles at 4 positions (Top/Bottom/Left/Right)

**4. FloatingEdge**
- Custom edge type
- Calculates intersection points dynamically
- Routes edges to closest handle position
- Applies relationship-specific styling

#### Hooks Used

```typescript
// State
const [nodes, setNodes] = useState<Node<KitDiagramNode>[]>([]);
const [edges, setEdges] = useState<Edge[]>([]);
const [isSimulating, setIsSimulating] = useState(true);

// Kit data
const kit = useKit();
const kitScope = useKitScope();

// Selection & hover
const [selection] = useKitAppSelection();
const [hover] = useKitAppHover();
const [setHover] = useKitAppSetHover();
const [clearHover] = useKitAppClearHover();

// Commands
const kitCommands = useKitAppCommands();

// Filters
const filterSearch = useSelector(actor, filterSearchSelector);
const expandedRows = useSelector(actor, expandedRowsSelector);

// Force settings
const [diagramForce] = useKitAppDiagramForce();

// React Flow
const { fitView } = useReactFlow();
```

### User Workflows

#### Workflow 1: Exploring Kit Structure

1. **Open kit** in Kit App
2. **Toggle to diagram view** (or use split screen)
3. **Observe graph structure:**
   - Clusters indicate related artifacts
   - Solid lines show hierarchies
   - Dashed lines show usage
4. **Click nodes** to select artifacts
5. **View details** in right panel

#### Workflow 2: Finding Related Artifacts

1. **Search** for an artifact by name
2. **Diagram filters** to show only matching nodes
3. **Observe connections:**
   - Which designs use this type?
   - What is the parent/child hierarchy?
4. **Click connected nodes** to explore relationships

#### Workflow 3: Organizing Kit

1. **Collapse folders** in table view
2. **Diagram updates** to show only expanded items
3. **Focus on specific subsection** of kit
4. **Drag nodes** to mentally organize layout
5. **Expand folders** to reveal more content

#### Workflow 4: Understanding Dependencies

1. **Select a type** in diagram
2. **Visual feedback** shows selection
3. **Trace dashed lines** to see which designs use it
4. **Click design node** to open design editor
5. **Verify pieces** using the type

### Troubleshooting

#### Common Issues

**Issue: Nodes overlap**
- **Cause**: `collideRadius` too small or too many nodes
- **Solution**: Increase `collideRadius` in diagram force settings

**Issue: Graph is too spread out**
- **Cause**: `chargeStrength` too high (more negative)
- **Solution**: Decrease absolute value (e.g., -80 → -50)

**Issue: Edges cross through nodes**
- **Cause**: Edge routing calculates intersection incorrectly
- **Solution**: This is a known limitation of 2D projection

**Issue: Node positions reset**
- **Cause**: Filter changes trigger re-simulation
- **Solution**: Manual positions are not persisted currently

**Issue: Diagram is blank**
- **Cause**: All nodes filtered out or kit is empty
- **Solution**: Clear search filter or add artifacts to kit

### Performance Considerations

**Current implementation:**
- ✅ **Synchronous simulation** - 120 ticks on mount
- ✅ **Filtered nodes** - Only visible nodes simulated
- ✅ **Static after init** - No continuous simulation
- ✅ **Manual drag** - Direct position update

**Performance limits:**
- **< 50 nodes** - Instant layout
- **50-200 nodes** - Slight delay (< 500ms)
- **> 200 nodes** - Noticeable delay (> 1s)

**Optimization strategies:**
1. **Filter aggressively** - Use search and expanded rows
2. **Split by kind** - View only types, or only designs
3. **Use folders** - Organize kit into smaller sections

### Next Steps to Work On

#### 1. **Layout Persistence** 🔴 HIGH PRIORITY

**Problem:** Manual node positions are lost on filter change or remount.

**Solution:**
```typescript
// Add to KitAppState
interface KitAppState {
  // ... existing fields
  diagramNodePositions?: Record<string, { x: number; y: number }>;
}

// Save positions on drag stop
const handleNodeDragStop = (node: Node) => {
  store.change({
    diagramNodePositions: {
      ...state.diagramNodePositions,
      [node.id]: node.position,
    },
  });
};

// Restore positions on mount
useEffect(() => {
  const savedPositions = state.diagramNodePositions;
  if (savedPositions) {
    setNodes(nodes.map(n => ({
      ...n,
      position: savedPositions[n.id] || n.position,
    })));
  }
}, []);
```

#### 2. **Zoom & Pan Controls** 🟡 MEDIUM PRIORITY

**Problem:** Currently disabled - users can't zoom or pan.

**Solution:**
```typescript
// Add zoom controls
<ReactFlow
  minZoom={0.1}
  maxZoom={4}
  panOnScroll={true}  // Enable pan with scroll
  zoomOnScroll={true} // Enable zoom with scroll + ctrl
  zoomOnPinch={true}  // Enable pinch-to-zoom
  zoomOnDoubleClick={true} // Enable double-click zoom
>
  <Controls /> {/* Add zoom/pan UI controls */}
  <MiniMap /> {/* Add overview minimap */}
</ReactFlow>
```

#### 3. **Force Settings UI** 🟡 MEDIUM PRIORITY

**Problem:** Force settings are hardcoded, users can't customize layout.

**Solution:**
- Add slider controls in settings panel:
  - Charge Strength (-200 to -20)
  - Link Distance (20 to 200)
  - Collide Radius (10 to 100)
  - Center Strength (0 to 1)
- Preview changes in real-time
- Reset to defaults button

#### 4. **Edge Labels** 🟢 LOW PRIORITY

**Problem:** Edges show relationship type only via style (solid/dashed).

**Solution:**
```typescript
<BaseEdge 
  id={id} 
  path={edgePath}
  label="uses" // Add text label
  labelStyle={{ fontSize: 10 }}
  labelBgStyle={{ fill: 'var(--background)' }}
/>
```

#### 5. **Node Grouping** 🟢 LOW PRIORITY

**Problem:** No visual grouping of related nodes.

**Solution:**
- Add background rectangles around node clusters
- Group by: folder, concept, author
- Collapsible groups (click to hide/show children)

#### 6. **Layout Algorithms** 🔵 FUTURE

**Problem:** Force-directed layout isn't always optimal.

**Alternative layouts to add:**
- **Hierarchical** - Tree-like top-down layout (dagre)
- **Circular** - Arrange nodes in circle
- **Grid** - Snap nodes to grid
- **Manual** - Free-form positioning only

#### 7. **Relationship Filters** 🔵 FUTURE

**Problem:** Can't filter by relationship type.

**Solution:**
- Toggle switches:
  - [ ] Show part-of relationships
  - [ ] Show reference relationships
- Separate sliders:
  - Part-of edge opacity (0-100%)
  - Reference edge opacity (0-100%)

#### 8. **Multi-Selection** 🔵 FUTURE

**Problem:** Can only select one node at a time.

**Solution:**
- **Ctrl+Click** - Add to selection
- **Shift+Click** - Range selection
- **Drag box** - Rectangle selection
- **Select connected** - Select all nodes connected to current

#### 9. **Export Diagram** 🔵 FUTURE

**Problem:** Can't save or share diagram view.

**Solution:**
- Export as PNG/SVG
- Export layout coordinates as JSON
- Import layout from JSON
- Share diagram URL with preserved layout

#### 10. **Animation** 🔵 FUTURE

**Problem:** Layout changes happen instantly (jarring).

**Solution:**
- Animate node positions with spring physics
- Fade in/out nodes when filtering
- Smooth zoom/pan transitions
- Highlight path on hover (animate stroke-dashoffset)

### Styling & UI Customization

The diagram window is highly customizable through CSS variables, inline styles, and Tailwind classes. Here's a complete guide to all visual elements you can modify.

#### 📐 Node Styling

**Current Implementation:**

```typescript
// Node dimensions (in Kit.tsx)
const NODE_WIDTH = 220;  // Width in pixels
const NODE_HEIGHT = 140; // Height in pixels

// Node wrapper styling
<div
  data-kit-node="v3"
  style={{ 
    width: NODE_WIDTH, 
    height: NODE_HEIGHT, 
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    background: "transparent", 
    border: "0",
    outline: "0",
    boxShadow: "none",
    pointerEvents: "auto",
    padding: 0,
    margin: 0
  }}
>
```

**What You Can Change:**

| Property | Current Value | Purpose | Suggested Range |
|----------|---------------|---------|-----------------|
| `NODE_WIDTH` | 220px | Node width | 100-400px |
| `NODE_HEIGHT` | 140px | Node height | 80-300px |
| `background` | transparent | Node background | Any CSS color |
| `border` | 0 | Node border | "1px solid #ccc" |
| `boxShadow` | none | Drop shadow | "0 2px 8px rgba(0,0,0,0.1)" |
| `padding` | 0 | Inner spacing | 0-20px |

**Node Shape Options:**

```css
/* Square nodes */
border-radius: 0;

/* Rounded nodes (current) */
border-radius: 8px;

/* Circular nodes */
border-radius: 50%;
width: 140px;
height: 140px;

/* Hexagonal nodes (requires clip-path) */
clip-path: polygon(25% 0%, 75% 0%, 100% 50%, 75% 100%, 25% 100%, 0% 50%);
```

#### 🎨 Avatar Styling

Nodes use the `TableAvatar` component which displays:
- **Image icon** (if provided)
- **React icon** (if no image)
- **Initials** (fallback)

```typescript
<TableAvatar 
  name={data.name} 
  icon={data.icon} 
  isSelected={isSelected} 
  isHovered={isHovered} 
/>
```

**Avatar states:**

```typescript
// Normal state
className="shrink-0"

// Selected state
className="ring-1 ring-[color:var(--active-base)]"
AvatarFallback: "bg-[color:var(--active-base)] text-[color:var(--active-foreground)]"

// Hovered state
className="ring-1 ring-[color:var(--hover-base)]"
AvatarFallback: "bg-[color:var(--hover-base)]"
```

**CSS Variables Used:**

```css
/* In globals.css */
--active-base: #3b82f6;      /* Blue for selection */
--active-foreground: #ffffff; /* White text on selection */
--hover-base: #9ca3af;        /* Gray for hover */
--foreground: #000000;        /* Default text color */
```

**Customization Examples:**

```css
/* Larger avatars */
.avatar {
  width: 80px !important;
  height: 80px !important;
  font-size: 24px !important;
}

/* Square avatars */
.avatar {
  border-radius: 0 !important;
}

/* Thicker selection ring */
.avatar.selected {
  ring-width: 3px !important;
}

/* Custom selection color */
.avatar.selected {
  --active-base: #10b981; /* Green */
}

/* Glow effect */
.avatar.selected {
  box-shadow: 0 0 20px var(--active-base) !important;
}
```

#### 🔗 Edge Styling

**Current Implementation:**

```typescript
const edgeStyle = {
  "part-of": { 
    stroke: "var(--accent-secondary)", 
    strokeWidth: 3 
  },
  reference: { 
    stroke: "var(--foreground)", 
    strokeWidth: 1, 
    strokeDasharray: "5,5" 
  },
};
```

**What You Can Change:**

| Property | Part-of | Reference | Purpose |
|----------|---------|-----------|---------|
| `stroke` | var(--accent-secondary) | var(--foreground) | Line color |
| `strokeWidth` | 3 | 1 | Line thickness |
| `strokeDasharray` | - | "5,5" | Dash pattern |
| `strokeLinecap` | - | - | Line end style |
| `opacity` | - | - | Transparency |

**Edge Style Options:**

```typescript
// Thick solid lines
{ stroke: "#3b82f6", strokeWidth: 5 }

// Dotted lines
{ strokeDasharray: "2,2" }

// Long dashes
{ strokeDasharray: "10,5" }

// Gradient (requires SVG gradient definition)
{ stroke: "url(#myGradient)" }

// Animated dashes
{ 
  strokeDasharray: "10,5",
  animation: "dash 1s linear infinite"
}

// Semi-transparent
{ opacity: 0.5 }

// Rounded ends
{ strokeLinecap: "round" }
```

**Edge Colors by Relationship:**

```typescript
// Color by hierarchy depth
const depthColors = ["#3b82f6", "#8b5cf6", "#ec4899"];
const color = depthColors[depth % depthColors.length];

// Color by artifact kind
const kindColors = {
  type: "#3b82f6",
  design: "#10b981",
  quality: "#f59e0b",
  // ...
};

// Rainbow edges
const hue = (index / totalEdges) * 360;
const color = `hsl(${hue}, 70%, 60%)`;
```

#### 🎯 Handle Styling

Handles are invisible connection points at node edges:

```typescript
<Handle 
  type="target" 
  position={Position.Top} 
  className="!bg-transparent !border-none !w-0 !h-0 !min-w-0 !min-h-0" 
/>
```

**To make handles visible:**

```typescript
// Small circles
className="!bg-blue-500 !w-2 !h-2 !border-2 !border-white"

// Larger targets
className="!bg-green-500 !w-4 !h-4 !rounded-full"

// Color by position
Top: "!bg-red-500"
Right: "!bg-blue-500"
Bottom: "!bg-green-500"
Left: "!bg-yellow-500"
```

#### 📊 Background Grid

```typescript
<Background gap={20} size={1} />
```

**Customization:**

```typescript
// Larger grid
<Background gap={50} size={2} />

// Dots instead of lines
<Background 
  variant={BackgroundVariant.Dots} 
  gap={20} 
  size={2} 
/>

// Custom color
<Background 
  gap={20} 
  size={1}
  style={{ backgroundColor: '#f0f0f0' }}
/>

// Hide grid
{/* Remove <Background /> component */}
```

#### 🖱️ Cursor Styling

```css
/* Normal cursor */
.react-flow__node {
  cursor: grab;
}

/* Dragging cursor */
.react-flow__node.dragging {
  cursor: grabbing;
}

/* Pointer for clickable */
.react-flow__node:hover {
  cursor: pointer;
}

/* Custom cursor */
.react-flow__node {
  cursor: url('custom-cursor.png'), auto;
}
```

#### 🎭 Interaction States

**CSS Override Styles:**

The diagram includes inline `<style>` to override React Flow defaults:

```css
/* Remove React Flow's default node styles */
[data-kit-node] {
  background: transparent !important;
  border: 0 !important;
  outline: 0 !important;
  box-shadow: none !important;
  padding: 0 !important;
  margin: 0 !important;
}

.react-flow__node,
.react-flow__node-artifact {
  background: transparent !important;
  border: 0 !important;
  outline: 0 !important;
  box-shadow: none !important;
}

/* Remove selection rectangle */
.react-flow__nodesselection-rect {
  display: none !important;
}
```

**To add custom hover effects:**

```css
/* Glow on hover */
.react-flow__node:hover [data-kit-node] {
  filter: drop-shadow(0 0 10px rgba(59, 130, 246, 0.5));
}

/* Scale on hover */
.react-flow__node:hover {
  transform: scale(1.05);
  transition: transform 0.2s;
}

/* Brighten on hover */
.react-flow__node:hover {
  filter: brightness(1.2);
}
```

#### 🎨 Color Schemes

**Current theme system:**

```typescript
// Selection color
--active-base: #3b82f6;        // Blue
--active-foreground: #ffffff;  // White

// Hover color
--hover-base: #9ca3af;         // Gray

// Edge colors
--accent-secondary: #8b5cf6;   // Purple (part-of edges)
--foreground: #000000;         // Black (reference edges)
```

**Alternative color schemes:**

```css
/* Warm theme */
--active-base: #f59e0b;        /* Orange */
--hover-base: #fbbf24;         /* Yellow */
--accent-secondary: #dc2626;   /* Red */

/* Cool theme */
--active-base: #0ea5e9;        /* Sky blue */
--hover-base: #06b6d4;         /* Cyan */
--accent-secondary: #6366f1;   /* Indigo */

/* Nature theme */
--active-base: #10b981;        /* Green */
--hover-base: #84cc16;         /* Lime */
--accent-secondary: #14b8a6;   /* Teal */

/* Monochrome */
--active-base: #000000;        /* Black */
--hover-base: #6b7280;         /* Gray */
--accent-secondary: #374151;   /* Dark gray */
```

#### 📏 Layout & Spacing

**ReactFlow viewport:**

```typescript
<ReactFlow
  minZoom={0.1}      // Zoom out limit
  maxZoom={4}        // Zoom in limit
  panOnDrag={[1, 2]} // Mouse buttons for panning
  panOnScroll={false}
  zoomOnScroll={true}
  zoomOnPinch={true}
  zoomOnDoubleClick={true}
>
```

**Initial view:**

```typescript
defaultViewport={{ x: 0, y: 0, zoom: 1 }}

// Start zoomed in
defaultViewport={{ x: 0, y: 0, zoom: 2 }}

// Start at specific position
defaultViewport={{ x: 100, y: 50, zoom: 1 }}
```

**Fit view settings:**

```typescript
fitView({ 
  padding: 0.3,    // 30% padding around nodes
  duration: 200,   // Animation duration in ms
  minZoom: 1,      // Don't zoom below 1x
  maxZoom: 1       // Don't zoom above 1x (prevents over-zoom)
})
```

#### 🎬 Animation Options

**Smooth transitions:**

```css
/* Animate node position changes */
.react-flow__node {
  transition: transform 0.3s ease-in-out;
}

/* Animate edge opacity */
.react-flow__edge path {
  transition: opacity 0.2s, stroke-width 0.2s;
}

/* Fade in on appear */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.react-flow__node {
  animation: fadeIn 0.3s;
}
```

**Pulsing selection:**

```css
@keyframes pulse {
  0%, 100% { 
    box-shadow: 0 0 0 0 rgba(59, 130, 246, 0.7); 
  }
  50% { 
    box-shadow: 0 0 0 10px rgba(59, 130, 246, 0); 
  }
}

.avatar.selected {
  animation: pulse 2s infinite;
}
```

**Animated edges:**

```css
@keyframes dash {
  to {
    stroke-dashoffset: -20;
  }
}

.react-flow__edge path {
  stroke-dasharray: 10 5;
  animation: dash 1s linear infinite;
}
```

#### 🏷️ Node Labels

**To add labels below nodes:**

```typescript
<div style={{ textAlign: "center", marginTop: "8px" }}>
  <div className="text-xs font-medium truncate max-w-[220px]">
    {data.name}
  </div>
  <div className="text-xs text-muted-foreground">
    {data.kind}
  </div>
</div>
```

**Label positioning:**

```css
/* Above node */
position: absolute;
top: -30px;
left: 50%;
transform: translateX(-50%);

/* Inside node */
position: absolute;
bottom: 10px;
left: 0;
right: 0;
text-align: center;

/* Beside node */
position: absolute;
left: calc(100% + 10px);
top: 50%;
transform: translateY(-50%);
```

#### 🎨 Conditional Styling

**Style nodes by kind:**

```typescript
const getNodeStyle = (kind: DiagramNodeKind) => {
  const styles = {
    type: { borderColor: "#3b82f6", backgroundColor: "#eff6ff" },
    design: { borderColor: "#10b981", backgroundColor: "#f0fdf4" },
    quality: { borderColor: "#f59e0b", backgroundColor: "#fffbeb" },
    // ...
  };
  return styles[kind] || {};
};

<div style={getNodeStyle(data.kind)}>
```

**Style by selection count:**

```typescript
const selectedCount = selection?.types?.length || 0;

// Highlight frequently selected items
const opacity = selectedCount > 5 ? 1 : 0.6;
```

**Style by hierarchy level:**

```typescript
// Darker colors for root nodes
const depth = calculateDepth(data.guid);
const opacity = Math.max(0.3, 1 - depth * 0.1);
```

#### 🎯 Advanced Customization

**Custom node shapes:**

```typescript
// Triangle
<div style={{
  width: 0,
  height: 0,
  borderLeft: "60px solid transparent",
  borderRight: "60px solid transparent",
  borderBottom: "100px solid blue"
}} />

// Diamond
<div style={{
  width: "100px",
  height: "100px",
  transform: "rotate(45deg)",
  backgroundColor: "blue"
}} />

// Star (requires SVG)
<svg width="100" height="100">
  <polygon points="50,0 61,35 98,35 68,57 79,91 50,70 21,91 32,57 2,35 39,35" />
</svg>
```

**Image backgrounds:**

```typescript
<div style={{
  backgroundImage: `url(${data.icon})`,
  backgroundSize: "cover",
  backgroundPosition: "center",
  width: NODE_WIDTH,
  height: NODE_HEIGHT
}} />
```

**Gradient backgrounds:**

```typescript
<div style={{
  background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
  width: NODE_WIDTH,
  height: NODE_HEIGHT
}} />
```

### Complete Styling Example

Here's a fully customized diagram with all styling options:

```typescript
// Custom node with gradient, shadow, and animations
const CustomKitNode: FC<NodeProps<Node<KitDiagramNode>>> = ({ data }) => {
  const isSelected = /* selection logic */;
  const isHovered = /* hover logic */;
  
  return (
    <div
      style={{
        width: 200,
        height: 120,
        background: isSelected 
          ? "linear-gradient(135deg, #667eea 0%, #764ba2 100%)"
          : "linear-gradient(135deg, #f6f8fb 0%, #e9ecef 100%)",
        borderRadius: "12px",
        border: isSelected ? "3px solid #667eea" : "2px solid #dee2e6",
        boxShadow: isSelected
          ? "0 8px 24px rgba(102, 126, 234, 0.4)"
          : "0 2px 8px rgba(0, 0, 0, 0.1)",
        padding: "16px",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        justifyContent: "center",
        transition: "all 0.3s ease",
        transform: isHovered ? "scale(1.05)" : "scale(1)",
        cursor: "pointer",
      }}
    >
      <TableAvatar 
        name={data.name} 
        icon={data.icon}
        className="mb-2"
      />
      <div className="text-sm font-medium text-center">
        {data.name}
      </div>
      <div className="text-xs text-muted-foreground">
        {data.kind}
      </div>
    </div>
  );
};

// Custom edges with gradient and animation
const customEdgeStyle = {
  "part-of": {
    stroke: "url(#gradient1)",
    strokeWidth: 4,
    filter: "drop-shadow(0 2px 4px rgba(0,0,0,0.2))",
  },
  reference: {
    stroke: "#6b7280",
    strokeWidth: 2,
    strokeDasharray: "8,4",
    animation: "dash 1s linear infinite",
  },
};

// Add gradient definition to diagram
<svg width="0" height="0">
  <defs>
    <linearGradient id="gradient1" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" stopColor="#667eea" />
      <stop offset="100%" stopColor="#764ba2" />
    </linearGradient>
  </defs>
</svg>
```

### Learning Resources

**Force-Directed Graphs:**
- [D3 Force Simulation](https://github.com/d3/d3-force)
- [Observable: Force-Directed Graph](https://observablehq.com/@d3/force-directed-graph)

**React Flow:**
- [React Flow Docs](https://reactflow.dev/)
- [Custom Nodes](https://reactflow.dev/learn/customization/custom-nodes)
- [Custom Edges](https://reactflow.dev/learn/customization/custom-edges)
- [Styling Guide](https://reactflow.dev/learn/customization/theming)

**CSS & Animations:**
- [CSS Animations](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Animations)
- [CSS Variables](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties)
- [Tailwind CSS](https://tailwindcss.com/docs)

**Graph Theory:**
- Understanding node degree (connections per node)
- Detecting connected components
- Finding shortest paths

---

## 9. Glossary of Terms

| Term | Definition |
|------|------------|
| **Artifact** | Any entity in a kit: Type, Design, Quality, Port, Tag, Concept, File, Folder, or Author |
| **CRDT** | Conflict-free Replicated Data Type - allows multiple users to edit simultaneously |
| **DnD Kit** | Drag and Drop Kit - library for drag and drop interactions |
| **Force Simulation** | Physics-based algorithm that positions nodes by simulating forces (repulsion, attraction) |
| **GUID** | Globally Unique Identifier - a string like "abc-123-def-456" that uniquely identifies an entity |
| **Hook (React)** | A function starting with `use` that lets you use React features in function components |
| **KitStore** | The Y.js-backed store that holds kit data and enables real-time collaboration |
| **Plugin** | A module that registers itself with the Sketchpad to add functionality |
| **React Flow** | Library for building interactive node-based graphs and diagrams |
| **Scope Provider** | React context provider that makes a GUID available to child components |
| **Selector** | A function that extracts a specific piece of state from a larger state object |
| **Triadic Hook** | A hook that returns `[value, setter, canSet]` - the standard pattern in Sketchpad |
| **XState** | State machine library used for managing application state transitions |
| **Y.js** | CRDT library that enables real-time collaborative editing |

---

## 10. Final Summary

### What You've Learned

1. **Kit.tsx is the management hub** for a single kit's contents
2. **Three pillars**: State Management (Y.js + XState), Commands & Hooks, UI Components
3. **Plugin architecture**: Kit App registers itself with Sketchpad
4. **Command pattern**: All changes go through named commands
5. **Triadic hooks**: Always `[value, setter, canSet]`
6. **Hierarchical data**: Flat table rows built from nested kit data
7. **Two views**: Table (hierarchical list) and Diagram (force-directed graph)
8. **Diagram window**: Force-directed graph with selection, hover, drag, and filtering
9. **Relationship visualization**: Part-of (solid) and reference (dashed) edges
10. **Physics-based layout**: D3 force simulation with configurable settings

### Key Takeaways

1. **Never mutate state directly** - always use setters
2. **Always check `canSet`** before calling setters
3. **Wrap components in scope providers** to access kit/design/type data
4. **Use stable selector references** to avoid infinite loops
5. **Handle loading states** - data might not be available immediately
6. **Diagram respects filters** - search and expanded rows affect node visibility
7. **Manual positioning available** - drag nodes to override physics
8. **Selection synced** - table and diagram views stay in sync

### Where to Go Next

1. **Design.tsx** - Learn how pieces and connections are managed
2. **Type.tsx** - Learn how connectors and models are managed
3. **Sketchpad.tsx** - Understand the base stores and state machine
4. **semio.ts** - Deep dive into domain types and diff system
5. **React Flow docs** - Learn about custom nodes and edges
6. **D3 force simulation** - Understand graph layout algorithms

### Next Development Priorities

**High Priority:**
1. ✅ Layout persistence (save manual node positions)

**Medium Priority:**
2. ✅ Zoom & pan controls with UI widgets
3. ✅ Force settings UI panel

**Low Priority:**
4. ✅ Edge labels for relationship types
5. ✅ Node grouping by folder/concept

**Future Enhancements:**
6. ⬜ Alternative layout algorithms (hierarchical, circular, grid)
7. ⬜ Relationship filters and opacity controls
8. ⬜ Multi-selection (Ctrl+Click, box selection)
9. ⬜ Export diagram (PNG, SVG, JSON)
10. ⬜ Smooth animations for layout changes

---

*Tutorial created for Kit.tsx in the semio Sketchpad application. Last updated: January 26, 2026.*
