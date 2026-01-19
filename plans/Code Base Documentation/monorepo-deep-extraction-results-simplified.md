# Semio Explained: A Beginner's Guide to the Architecture

> **What You're Reading**: A complete explanation of the Semio codebase for someone who has never programmed before  
> **Date**: January 12, 2026  
> **Size**: About 75,000 lines of code (roughly equivalent to a 1,500-page book)

---

## 🎯 What Is Semio? The Big Picture

Imagine you're playing with LEGO bricks. You have:

- **Individual brick types** (2x4 red brick, 1x1 blue brick, etc.)
- **Instruction manuals** that show how to combine bricks
- **Finished models** (a house, a car, a spaceship)

**Semio is like digital LEGO for architects and designers**, but instead of plastic bricks, they use:

- **Types** = reusable building components (walls, windows, doors, solar panels)
- **Designs** = instructions for combining components into buildings
- **Kits** = collections of component types and design instructions

### Real-World Example: Building a Solar House

Let's say you want to design eco-friendly houses:

1. **Create a Kit** called "Sustainable Housing Kit v1.0"
2. **Add Types** (components):
   - Wall panels (2m × 3m, insulated)
   - Solar panels (1m × 2m, 300W each)
   - Windows (1m × 1.5m, triple-glazed)
   - Doors (0.9m × 2m, wooden)

3. **Create a Design** called "2-Bedroom Solar Home":
   - Use 20 wall panels
   - Connect them at specific angles
   - Add 4 windows on the south side
   - Place 12 solar panels on the roof

The beauty? **Anyone can take your kit and create their own variations** by rearranging the same components.

---

## 🏗️ Architecture: How Semio Is Built

Think of Semio as a city with different districts, each serving a specific purpose.

### The City Map (C4 Context Diagram)

```
👥 PEOPLE WHO USE SEMIO
├── Architects (design buildings)
├── Designers (create furniture systems)
├── Engineers (optimize structures)
└── Developers (extend the software)

🏢 SEMIO CITY (The Software)
├── 🌐 Web District (Sketchpad in your browser)
├── 💻 Desktop District (Sketchpad as an app on your computer)
├── 🦏 Rhino/Grasshopper District (plugin for 3D modeling software)
├── 🛠️ VS Code District (tools for developers)
└── 📚 Documentation District (help guides)

🗄️ STORAGE BASEMENT
├── SQLite (file-based database, like a digital filing cabinet)
├── IndexedDB (browser storage, like cookies but bigger)
└── Y.js (real-time sync, like Google Docs for design data)

🔌 CONNECTED SERVICES
├── OpenAI (AI assistance)
├── GitHub (code hosting)
└── Other design tools (Speckle, Ladybug)
```

### The Building Blocks (Containers)

Think of each container as a separate building in our city:

| Building Name          | What It's Built With | What It Does                                          | Analogy                         |
| ---------------------- | -------------------- | ----------------------------------------------------- | ------------------------------- |
| **@semio/js**          | TypeScript + React   | The brain and heart - core logic and visual interface | City Hall (central operations)  |
| **@semio/desktop**     | Electron             | Wraps the web app as a desktop program                | Mobile city hall branch         |
| **@semio/docs**        | Astro + Markdown     | User manuals and tutorials                            | Public library                  |
| **@semio-repo/vscode** | VS Code Extension    | Developer tools                                       | Inspector's office              |
| **@semio/engine**      | Python + FastAPI     | Backend server for complex operations                 | Power plant (backend services)  |
| **@semio/net**         | C# .NET              | Core library for Rhino integration                    | Bridge to Rhino City            |
| **@semio/grasshopper** | C# Grasshopper       | Visual programming plugin                             | LEGO Mindstorms (visual coding) |
| **@semio/repo**        | Go                   | Command-line tools for managing the codebase          | City maintenance crew           |
| **@semio/mcp**         | Go MCP               | Interface for AI agents to work with code             | Robot assistant protocol        |

#### Real-World Comparison:

- **TypeScript/JavaScript** = The universal language spoken everywhere on the web
- **Python** = Great for data science and AI
- **C#** = Microsoft's language, perfect for Windows desktop apps and Rhino plugins
- **Go** = Fast and efficient, good for command-line tools

---

## 🧩 The Core Concepts: Kit-of-Parts Design

Let's break down the main ideas using our solar house example:

### 1. Kit (The Box Everything Comes In)

**Analogy**: A LEGO set box

**Contains**:

- Types (1-256 different component types)
- Designs (1-128 different instruction manuals)
- Qualities (1-1024 measurements like "weight," "cost," "energy rating")
- Files (3D models, images, PDFs)
- Authors (who created this kit)
- Metadata (version, description, license)

**Technical Details**:

- **GUID**: Globally Unique Identifier (like a social security number for digital objects)
- **Version**: Uses semantic versioning (e.g., v2.1.4 = major.minor.patch)
- **License**: Legal terms (e.g., "free for personal use")

**Example JSON**:

```json
{
  "guid": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  "name": "Sustainable Housing Kit",
  "version": "1.0.0",
  "description": "Eco-friendly modular home components",
  "license": "CC-BY-4.0",
  "types": [...],
  "designs": [...]
}
```

---

### 2. Type (A Reusable Component)

**Analogy**: A LEGO brick specification (dimensions, color, attachment points)

**Contains**:

- **Models**: 3D geometry files (like STL for 3D printing)
  - Limit: 1-32 different visual representations
  - Example: "Wall_Panel_Default.glb", "Wall_Panel_Winter.glb"
- **Connectors**: Attachment points (where this component connects to others)
  - Limit: 1-32 connection points
  - Example: A wall panel has connectors on all 4 edges
- **Properties** (Props): Measurements
  - Example: Weight = 45kg, Cost = $120, R-value = 5.0

**Technical Details**:

- **isVirtual**: If true, this is an abstract type (like "Wall" as a category, not a specific wall)
- **canScale**: Can you make it bigger/smaller? (stretch a wall panel from 2m to 3m)
- **canMirror**: Can you flip it? (make a left-hand door into a right-hand door)
- **unit**: Measurement system (metric = meters, imperial = feet)

**Real Example**: Solar Panel Type

```typescript
{
  guid: "solar-panel-300w",
  name: "300W Solar Panel",
  isVirtual: false,
  canScale: false,  // Cannot stretch solar panels
  canMirror: false, // Cannot flip solar panels
  unit: "m",

  models: [
    {
      name: "Standard Model",
      file: "solar_panel.glb",
      tags: ["default", "high-detail"]
    }
  ],

  connectors: [
    {
      id: "mount-1",
      point: { x: 0, y: 0, z: 0 },      // Position in 3D space
      direction: { x: 0, y: 0, z: -1 }, // Points downward (toward roof)
      mandatory: true,                   // Must be connected
      interface: "mounting-bolt-m8"      // Compatible with M8 bolts
    }
  ],

  props: [
    { key: "power-output", value: 300, unit: "W" },
    { key: "weight", value: 18, unit: "kg" },
    { key: "cost", value: 250, unit: "USD" }
  ]
}
```

---

### 3. Design (Instructions for Assembly)

**Analogy**: LEGO instruction manual

**Contains**:

- **Pieces**: Instances of types placed in 3D space (1-512 pieces)
  - Example: "Wall panel #1 at position (0, 0, 0), Wall panel #2 at position (2, 0, 0)"
- **Connections**: How pieces link together
  - Example: "Wall #1's right connector links to Wall #2's left connector"
- **Layers**: Organizational grouping (like Photoshop layers)
  - Example: "Ground floor layer," "First floor layer," "Roof layer"
- **Groups**: Logical clusters of pieces
  - Example: "Kitchen group" contains cabinets, sink, appliances

**Technical Details**:

- **Plane**: Position + orientation in 3D space (origin point + X/Y/Z axes)
- **Scale**: Size multiplier (1.0 = normal, 2.0 = double size)
- **Color**: RGB color override (e.g., #FF5733 = orange-red)

**Real Example**: 2-Bedroom House Design

```typescript
{
  guid: "house-2bed-solar",
  name: "2-Bedroom Solar Home",

  pieces: [
    {
      id: "wall-north-1",
      type: "wall-panel-2x3",           // References a Type
      plane: {
        origin: { x: 0, y: 0, z: 0 },   // Position
        xAxis: { x: 1, y: 0, z: 0 },    // Facing east
        yAxis: { x: 0, y: 1, z: 0 }     // Facing north
      },
      scale: { x: 1.0, y: 1.0, z: 1.0 },
      color: "#FFFFFF"
    },
    {
      id: "wall-east-1",
      type: "wall-panel-2x3",
      plane: {
        origin: { x: 2, y: 0, z: 0 },
        xAxis: { x: 0, y: 1, z: 0 },    // Rotated 90°
        yAxis: { x: -1, y: 0, z: 0 }
      },
      scale: { x: 1.0, y: 1.0, z: 1.0 },
      color: "#FFFFFF"
    },
    {
      id: "solar-1",
      type: "solar-panel-300w",
      plane: {
        origin: { x: 1, y: 1, z: 3 },   // On roof
        xAxis: { x: 1, y: 0, z: 0 },
        yAxis: { x: 0, y: 0.866, z: 0.5 } // 30° angle
      },
      scale: { x: 1.0, y: 1.0, z: 1.0 },
      color: "#1a1a1a"
    }
  ],

  connections: [
    {
      connected: {
        piece: "wall-north-1",
        connector: "right-edge"
      },
      connecting: {
        piece: "wall-east-1",
        connector: "left-edge"
      },
      gap: 0,        // No space between walls
      shift: 0,      // No sideways offset
      rise: 0,       // No vertical offset
      rotation: 90,  // 90° corner
      turn: 0,
      tilt: 0
    }
  ],

  layers: [
    { path: "ground-floor", color: "#8B4513", isHidden: false },
    { path: "first-floor", color: "#A0522D", isHidden: false },
    { path: "roof", color: "#654321", isHidden: false }
  ]
}
```

---

### 4. Connector (Attachment Point)

**Analogy**: LEGO studs and holes

**Key Properties**:

- **Point**: 3D coordinates (x, y, z)
- **Direction**: Which way it faces (like an arrow pointing outward)
- **t**: Position on a ring diagram (0-1, where 0 = top of circle)
- **Mandatory**: Must be connected or can stay free?
- **Interface**: What it's compatible with

**Real Example**: Window Frame Connectors

```typescript
{
  id: "window-top",
  point: { x: 0, y: 0, z: 1.5 },      // Top of window
  direction: { x: 0, y: 0, z: 1 },    // Points upward
  t: 0.0,                              // Top position in diagram
  mandatory: true,                     // Must connect to header
  interface: "header-groove",          // Compatible interface type

  props: [
    { key: "load-capacity", value: 500, unit: "N" }  // Can support 500 Newtons
  ]
}
```

---

### 5. Connection (How Pieces Link)

**Analogy**: Snapping two LEGO bricks together with specific positioning

**The 6 Degrees of Freedom**:

1. **Gap** (Y-axis translation): Distance forward/backward
   - Example: 0.05m gap between wall panels for sealant

2. **Shift** (X-axis translation): Distance left/right
   - Example: Offset a beam 0.1m to the side

3. **Rise** (Z-axis translation): Distance up/down
   - Example: Raise a beam 0.2m higher

4. **Rotation** (around Y-axis): Spinning like a door hinge
   - Example: 90° for a corner connection

5. **Turn** (around Z-axis): Rotating like a screw
   - Example: 45° for an angled roof beam

6. **Tilt** (around X-axis): Tilting forward/backward
   - Example: 30° slope for a solar panel

**Diagram Positioning**:

- **u**: Horizontal offset in 2D diagram (-1 to 1)
- **v**: Vertical offset in 2D diagram (-1 to 1)

**Real Example**: Corner Wall Connection

```typescript
{
  connected: {
    piece: "wall-north-1",
    connector: "right-edge"
  },
  connecting: {
    piece: "wall-east-1",
    connector: "left-edge"
  },

  // Translation (position adjustments)
  gap: 0.0,      // Tight fit, no gap
  shift: 0.0,    // No sideways shift
  rise: 0.0,     // Same height level

  // Rotation (angle adjustments)
  rotation: 90,  // 90° corner (perpendicular walls)
  turn: 0,       // No twist
  tilt: 0,       // No forward/back tilt

  // 2D diagram positioning
  u: 0.5,        // Slightly right in diagram
  v: 0.0,        // Centered vertically in diagram

  attributes: [
    { key: "joint-type", value: "tongue-and-groove" }
  ]
}
```

---

### 6. Quality (Measurement Standards)

**Analogy**: Nutrition labels on food (calories, protein, vitamins)

**Types of Qualities**:

- **General**: Applies to anything (cost, weight)
- **Type**: Applies to component types (load capacity)
- **Design**: Applies to complete designs (total energy consumption)
- **Piece**: Applies to individual instances (actual weight after scaling)
- **Connection**: Applies to joints (shear strength)
- **Connector**: Applies to attachment points (torque rating)

**Benchmarks**: Performance standards

**Real Example**: Energy Efficiency Quality

```typescript
{
  key: "energy-rating",
  name: "Energy Efficiency Rating",
  kind: "Design",  // Applies to complete buildings

  default: 50,     // Default value
  unit: "kWh/m²/year",  // Kilowatt-hours per square meter per year

  min: 0,
  max: 300,

  benchmarks: [
    {
      name: "Passive House",
      icon: "🏆",
      min: 0,
      max: 15,  // Ultra-efficient (uses <15 kWh/m²/year)
    },
    {
      name: "Low Energy",
      icon: "⭐",
      min: 15,
      max: 50,
    },
    {
      name: "Standard",
      icon: "✓",
      min: 50,
      max: 100,
    },
    {
      name: "Inefficient",
      icon: "⚠️",
      min: 100,
      max: 300,
    }
  ]
}
```

---

### 7. Interface (Compatibility Rules)

**Analogy**: USB-C ports (certain devices can only connect to compatible ports)

**Purpose**: Define which connectors can link together

**Real Example**: Electrical Connector Compatibility

```typescript
{
  guid: "electrical-plug-220v",
  name: "European 220V Plug",
  description: "Type C/E/F electrical plug",
  icon: "🔌",

  compatibleInterfaces: [
    "electrical-socket-220v-typeC",
    "electrical-socket-220v-typeE",
    "electrical-socket-220v-typeF"
  ],

  attributes: [
    { key: "voltage", value: 220, unit: "V" },
    { key: "max-current", value: 16, unit: "A" }
  ]
}
```

**How It Works**:

- A "220V plug" connector can ONLY link to compatible "220V socket" connectors
- A "110V plug" connector CANNOT link to "220V socket" connectors
- If no interface is specified = compatible with everything (default port)

---

## 🎨 How Users Interact with Semio

### Different Interfaces for Different Users

Think of Semio like a Swiss Army knife - same core tool, different interfaces:

#### 1. **Sketchpad (Web & Desktop)**

**Audience**: Architects, designers  
**Analogy**: Adobe Photoshop for buildings

**Features**:

- Drag-and-drop components onto a 3D canvas
- Visual connection tools (click connector A, click connector B)
- Real-time 3D preview
- Multi-window workspace (scene view, diagram view, property panel)

**Example Workflow**:

1. Open "Sustainable Housing Kit"
2. Create new design: "My Dream Home"
3. Drag wall panel to canvas → appears in 3D
4. Drag another wall → snap it to first wall's connector
5. Add windows, doors, solar panels
6. Export to .zip file or share link

#### 2. **Grasshopper Plugin (Rhino)**

**Audience**: Parametric designers, engineers  
**Analogy**: Visual programming (like Scratch for architects)

**Features**:

- Node-based workflow (connect boxes with wires)
- Live parameter adjustments
- Integration with Rhino's 3D modeling

**Example Workflow**:

```
[Load Kit] → [Select Type: Wall] → [Array: 10 units] → [Rotate: 15°] → [Preview]
```

#### 3. **VS Code Extension**

**Audience**: Developers  
**Analogy**: Microsoft Word with spell-check, but for code

**Features**:

- Real-time validation (red squiggles under errors)
- Quick fixes (click to auto-correct)
- Code navigation (jump to definitions)

#### 4. **Command Line (Repo CLI)**

**Audience**: Developers, automation  
**Analogy**: Text-based instructions to computer

**Example Commands**:

```bash
# Validate a kit file
repo analyze path/to/kit.json

# Apply automatic fixes
repo fix path/to/kit.json

# Open a development ticket
repo ticket open TASK-NAME "description"

# List all tickets from December 2025
repo ticket list 2025 12
```

---

## 🔄 How Data Flows Through the System

### Example: User Creates a New Wall Piece

Let's follow what happens step-by-step when you drag a wall onto the canvas:

#### Step 1: User Action (React Component)

```typescript
// User drags "wall-panel-2x3" type to canvas
onDrop(wallType, position) {
  // 👇 Sends event to state machine
  actor.send({
    type: "DESIGN.CREATE_PIECE",
    origin: "semio.sketchpad.canvas.drop-zone",  // Where this came from
    typeGuid: "wall-panel-2x3",
    position: { x: 5, y: 3, z: 0 }
  })
}
```

#### Step 2: State Machine (XState)

**Analogy**: Traffic controller deciding what happens next

```typescript
// State machine receives event and decides if it's valid
sketchpadMachine.states.navigation.design.on({
  "DESIGN.CREATE_PIECE": {
    guard: "hasDesignScope", // Check: are we in a design?
    actions: "handleCreatePiece", // If yes, execute this action
  },
});
```

#### Step 3: Command Execution (Business Logic)

```typescript
// Execute the actual logic
executeCommand(
  "semio.designApp.createPiece",
  "semio.sketchpad.canvas.drop-zone", // Origin (for logging)
  {
    typeGuid: "wall-panel-2x3",
    position: { x: 5, y: 3, z: 0 },
  },
);

// Inside command handler:
function createPiece(context, params) {
  // 1. Start transaction (group changes for undo/redo)
  store.startTransaction();

  // 2. Generate new piece
  const newPiece = {
    id: generateGuid(), // Create unique ID
    type: params.typeGuid,
    plane: positionToPlane(params.position),
    scale: { x: 1, y: 1, z: 1 },
    color: "#FFFFFF",
  };

  // 3. Calculate difference (diff)
  const pieceDiff = {
    added: [newPiece],
  };
  const kitDiff = {
    designs: {
      updated: [
        {
          id: currentDesignGuid,
          diff: {
            pieces: pieceDiff,
          },
        },
      ],
    },
  };

  // 4. Record for undo/redo
  store.recordEdit({
    do: { kitDiff },
    undo: { kitDiff: inverse(kitDiff) }, // Calculate reverse
  });

  // 5. Apply change
  applyKitDiff(currentKit, kitDiff);

  // 6. Finalize transaction
  store.finalizeTransaction();

  return newPiece;
}
```

#### Step 4: Data Persistence (Y.js)

**Analogy**: Auto-save feature in Google Docs

```typescript
// Y.js automatically syncs changes to:

// 1. IndexedDB (browser storage) - saves locally
indexeddbPersistence.update();

// 2. Remote server (if connected) - syncs with team
websocketProvider.send(changes);

// 3. Other users' browsers - real-time collaboration
// Their screens update automatically!
```

#### Step 5: UI Update (React Re-render)

```typescript
// React component subscribes to changes
const pieces = useSyncExternalStore(
  designStore.onChanged,  // Subscribe
  () => designStore.snapshot().pieces  // Get data
)

// When pieces change, component re-renders automatically
return (
  <Canvas>
    {pieces.map(piece => (
      <Piece3D key={piece.id} {...piece} />
    ))}
  </Canvas>
)
```

### Visual Flow Diagram

```
User Drag → React Event Handler → XState Machine → Command
                                        ↓
                                   Store Transaction
                                        ↓
                          Calculate Diff (what changed)
                                        ↓
                            Record Edit (undo/redo)
                                        ↓
                              Apply to Y.js Doc
                                        ↓
                    ┌───────────────────┼───────────────────┐
                    ↓                   ↓                   ↓
              IndexedDB           Remote Sync         Other Users
              (local save)        (team server)       (live update)
                    ↓                   ↓                   ↓
                    └───────────────────┴───────────────────┘
                                        ↓
                               React Re-renders
                                        ↓
                              User Sees New Wall
```

---

## 🗄️ Data Storage: Where Everything Lives

### Different Storage Types

Think of these like different filing systems:

#### 1. **SQLite Database** (.semio/kit.db inside .zip file)

**Analogy**: A digital filing cabinet with drawers and folders

**What it stores**: Complete kit information
**When it's used**: When exporting/importing kits, sharing kits

**Structure Example**:

```sql
-- Types table
CREATE TABLE types (
  guid TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  is_virtual BOOLEAN,
  can_scale BOOLEAN,
  parent_guid TEXT,
  ...
);

-- Models table
CREATE TABLE models (
  guid TEXT PRIMARY KEY,
  type_guid TEXT,
  name TEXT,
  file_path TEXT,
  ...
);
```

**Real Example**: When you export a kit

```
sustainable-housing-v1.zip
├── .semio/
│   └── kit.db (SQLite database with all metadata)
├── models/
│   ├── wall_panel.glb (3D model file)
│   ├── solar_panel.glb
│   └── window.glb
└── images/
    └── kit_preview.png
```

#### 2. **Y.js Document** (CRDT - Conflict-free Replicated Data Type)

**Analogy**: Google Docs magic (multiple people editing simultaneously)

**What it stores**: Live kit state during editing
**When it's used**: Real-time collaboration

**Technical Detail**: CRDT = Special data structure that automatically merges changes

- User A adds wall at position (1,0,0)
- User B adds window at position (2,0,0)
- Y.js automatically merges both changes without conflicts

**Example Structure**:

```typescript
yDoc.getMap("kit").set("name", "Sustainable Housing Kit").set("version", "1.0.0");

yDoc.getArray("types").push([wallType, windowType, doorType]);

yDoc.getArray("designs").push([house1, house2]);
```

#### 3. **IndexedDB** (Browser Storage)

**Analogy**: Your browser's personal storage closet

**What it stores**: Cached Y.js documents for offline use
**When it's used**: Auto-saving your work in browser

**Capacity**: ~50MB-1GB (depends on browser)

#### 4. **File System** (Your Computer's Hard Drive)

**Analogy**: Regular folders and files

**What it stores**:

- Tickets (development tasks)
- Reports (analysis results)
- Exports (kit .zip files)

**Example Structure**:

```
tickets/
├── 2025/
│   └── 12/
│       └── 24/
│           └── FEATURE-SOLAR-PANELS/
│               ├── ticket.md (task definition)
│               ├── plan.md (strategy)
│               ├── log.md (progress notes)
│               └── summary.md (results)

reports/
├── i18n.json (translation validation)
├── eslint.json (code quality)
└── typescript.json (type errors)
```

---

## ⚙️ Technical Deep Dive: Key Mechanisms

### 1. The Diff System (Change Tracking)

**Analogy**: Microsoft Word's "Track Changes" feature

**Purpose**: Record exactly what changed, so we can:

- Undo changes
- Redo changes
- See what's different between versions
- Merge multiple people's changes

**How it works**:

```typescript
// Original state
const before = {
  pieces: [
    { id: "wall-1", color: "#FFFFFF" },
    { id: "wall-2", color: "#FFFFFF" },
  ],
};

// User changes wall-1 color to blue
const after = {
  pieces: [
    { id: "wall-1", color: "#0000FF" }, // Changed!
    { id: "wall-2", color: "#FFFFFF" },
  ],
};

// Calculate diff
const diff = getDiff(before, after);
// Result:
{
  pieces: {
    updated: [
      {
        id: "wall-1",
        diff: { color: "#0000FF" },
      },
    ];
  }
}

// Calculate inverse (for undo)
const inverseDiff = inverseDiff(before, diff);
// Result:
{
  pieces: {
    updated: [
      {
        id: "wall-1",
        diff: { color: "#FFFFFF" }, // Restore original
      },
    ];
  }
}
```

**Why this matters**:

- **Undo/Redo**: Apply inverse diff to go backward
- **Collaboration**: Merge multiple users' diffs
- **History**: See exactly what changed over time
- **Efficiency**: Only send changes, not entire documents

### 2. Transactions (Grouping Changes)

**Analogy**: Banking transactions (either complete or nothing happens)

**Example**: Adding a room with furniture

```typescript
// Start transaction
store.startTransaction();

try {
  // Multiple operations
  createWall("wall-1"); // Edit #1
  createWall("wall-2"); // Edit #2
  createDoor("door-1"); // Edit #3
  createWindow("window-1"); // Edit #4

  // Success - commit all changes as ONE undo action
  store.finalizeTransaction();
} catch (error) {
  // Failure - undo ALL changes
  store.abortTransaction();
}
```

**Without transactions**:

- Undo → removes window
- Undo → removes door
- Undo → removes wall-2
- Undo → removes wall-1
  (4 undo operations)

**With transactions**:

- Undo → removes entire room
  (1 undo operation)

### 3. State Machine (XState)

**Analogy**: Traffic light system (clear rules for transitions)

**Purpose**: Control what's allowed when

**Example States**:

```
Sketchpad
├── navigation
│   ├── home (browsing kits)
│   ├── kit (viewing kit contents)
│   ├── design (editing a design)
│   │   ├── idle
│   │   ├── selecting
│   │   └── dragging
│   ├── type (editing a type)
│   └── docs (reading help)
└── settings
    ├── theme (light/dark)
    ├── language (en/de)
    └── expertise (beginner/normal/expert)
```

**Rules**:

- You can ONLY create pieces when in "design" state
- You can ONLY edit connectors when in "type" state
- You CANNOT access design commands when in "home" state

**Code Example**:

```typescript
const machine = createMachine({
  initial: "home",
  states: {
    home: {
      on: {
        OPEN_KIT: "kit", // Allowed transition
      },
    },
    kit: {
      on: {
        OPEN_DESIGN: "design",
        OPEN_TYPE: "type",
        BACK: "home",
      },
    },
    design: {
      on: {
        CREATE_PIECE: {
          guard: "hasSelection", // Only if something selected
          actions: "createPiece",
        },
        DELETE: {
          guard: "hasSelection",
          actions: "deletePiece",
        },
        BACK: "kit",
      },
    },
  },
});
```

### 4. Validation System

**Analogy**: Spell-check and grammar check

**Purpose**: Find errors and suggest fixes

**Constraint Types**:

1. **GUID Uniqueness**: Every ID must be unique

   ```typescript
   // ❌ BAD - duplicate IDs
   { id: "wall-1", ... }
   { id: "wall-1", ... }  // Duplicate!

   // Fix: Regenerate second ID
   { id: "wall-1", ... }
   { id: "wall-2", ... }  // Unique!
   ```

2. **Name Uniqueness (Scoped)**: Siblings must have unique names

   ```typescript
   // ❌ BAD - two walls with same name in same design
   pieces: [
     { id: "p1", name: "North Wall", ... },
     { id: "p2", name: "North Wall", ... }  // Duplicate!
   ]

   // Fix: Rename
   pieces: [
     { id: "p1", name: "North Wall", ... },
     { id: "p2", name: "North Wall 2", ... }  // Unique!
   ]
   ```

3. **Mandatory Connectors**: Required connections must exist

   ```typescript
   // ❌ BAD - door has mandatory "hinge" connector but no connection
   piece: {
     type: "door",
     connectors: [
       { id: "hinge", mandatory: true }  // Must be connected!
     ]
   }
   connections: []  // No connections!

   // Fix: Add connection
   connections: [{
     connected: { piece: "door-1", connector: "hinge" },
     connecting: { piece: "frame-1", connector: "left" }
   }]
   ```

**Validation Flow**:

```
Kit Modified
    ↓
Build Context (gather all data)
    ↓
Run Constraints (check rules)
    ↓
Generate Problems (list errors)
    ↓
Generate Fixes (create KitDiff to fix each problem)
    ↓
User Chooses:
  → Apply fix (apply KitDiff)
  → Ignore (mark as exception)
```

---

## 🌍 Cross-Platform Implementation

One of Semio's unique features: **The same domain model implemented in 4 different programming languages**

### Why?

**Different platforms need different languages**:

- **Web** = TypeScript (runs in browsers)
- **Backend** = Python (AI integration, databases)
- **Rhino/Grasshopper** = C# (required by Rhino API)
- **CLI Tools** = Go (fast, efficient command-line programs)

### How It Stays Synchronized

**1. JSON Schema (Central Source of Truth)**

```json
// jsonschema/kit.json
{
  "type": "object",
  "properties": {
    "guid": { "type": "string", "format": "uuid" },
    "name": { "type": "string", "minLength": 1 },
    "types": {
      "type": "array",
      "items": { "$ref": "#/definitions/Type" },
      "maxItems": 256
    }
  }
}
```

**2. Code Generation** (Automated)

```bash
# Run schema generator
tsx jsonschema/build.ts

# Generates:
# - jsonschema/kit.json (JSON Schema)
# - sql/sqlite/schema.sql (Database schema)
# - graphql/schema.graphql (GraphQL schema)
```

**3. Cross-Platform Tests**

```typescript
// Test file: kit_invalid.json (intentionally broken)
{
  guid: "duplicate-id",
  types: [
    { guid: "duplicate-id", ... },  // Error!
    { guid: "duplicate-id", ... }   // Duplicate!
  ]
}

// Expected output: validation.json
{
  problems: [
    {
      constraintId: "guid-unique",
      message: "Duplicate GUID found",
      entityGuid: "duplicate-id",
      fixes: [...]
    }
  ]
}
```

**All 4 implementations must produce identical validation.json**

**Test runner**:

```bash
# TypeScript
npm test -- validation.test.ts

# Python
pytest engine_test.py

# C#
dotnet test Semio.Tests

# Compare outputs
diff ts-validation.json py-validation.json
# Should output: (no differences)
```

---

## 🔥 Potential Problems (Technical Debt)

### "God Modules" (Too Much in One File)

**What's a God Module?** A file that does too many things

**Analogy**: Imagine a Swiss Army knife with 200 tools - technically it works, but it's hard to use and maintain.

**Current God Modules**:

1. **Sketchpad.tsx** (15,835 lines)
   - Contains: Store classes, state machine, all apps, kit management
   - **Problem**: Hard to find specific code, slow to load in editor
   - **Solution**: Split into separate files
     ```
     sketchpad/
     ├── Sketchpad.tsx (main shell only)
     ├── stores/
     │   ├── Store.ts
     │   ├── AppStore.ts
     │   ├── KitDiffAppStore.ts
     │   └── KitStore.ts
     ├── machines/
     │   └── sketchpadMachine.ts
     └── apps/
         ├── Home.tsx
         ├── Kit.tsx
         ├── Design.tsx
         └── Type.tsx
     ```

2. **repo.go** (10,110 lines)
   - Contains: All CLI commands, GraphQL, policies, tickets
   - **Problem**: Everything in one file
   - **Solution**: Split by domain
     ```
     repo/
     ├── cmd/
     │   ├── ticket.go
     │   ├── analyze.go
     │   └── fix.go
     ├── graph/
     │   └── executor.go
     └── policies/
         ├── code.go
         ├── sections.go
         └── contributors.go
     ```

### Tight Coupling (Components Depend on Each Other Too Much)

**Example Problem**: Y.js ↔ XState Bidirectional Sync

**What happens now**:

```
Y.js changes → triggers → XState update
                    ↓
               XState update → triggers → Y.js change
                                    ↓
                              Y.js change → triggers → XState update
                                              (infinite loop danger!)
```

**Why it's hard**:

- Must carefully track which updates came from where
- Easy to create infinite loops
- Hard to debug

**Better approach** (suggestion):

```
Single Source of Truth: Y.js
    ↓
XState reads Y.js (one-way)
    ↓
User actions → XState events → Y.js changes
```

### Manual Schema Synchronization

**Current process** (error-prone):

1. Update semio.ts (TypeScript)
2. Manually update engine.py (Python)
3. Manually update Semio.cs (C#)
4. Manually update SQL schema
5. Manually update JSON schema
6. Run tests and hope nothing broke

**Better approach** (recommendation):

```
Single source of truth: TypeScript with Zod
    ↓
Code generator
    ↓
Generates: Python, C#, SQL, JSON Schema, GraphQL
```

---

## 🛠️ Development Workflow

### How Developers Work on Semio

**1. Ticket System** (AI-First Development)

**Real Example**: Adding solar panel calculator feature

```bash
# Step 1: Create ticket
repo ticket open SOLAR-CALCULATOR "Add energy calculation for solar panels"

# This creates:
tickets/2026/01/12/SOLAR-CALCULATOR/
├── ticket.md      # Task definition
├── plan.md        # (AI writes plan here)
├── log.md         # (AI logs progress here)
└── summary.md     # (Final results here)
```

**ticket.md frontmatter**:

```yaml
---
slug: SOLAR-CALCULATOR
prompt: "Add energy calculation for solar panels"
status: open
author: John Doe <john@example.com>
date:
  created: 2026-01-12T10:30:00Z
model: claude-opus-4
commit: abc123def456 # Git commit when ticket opened
bundles: {} # Will be filled on close
---
```

**Step 2: AI writes plan**

```markdown
# plan.md

## Goal

Add a quality measurement for solar energy generation

## Steps

1. Create "solar-output" quality in semio.ts
2. Add formula: panels × 300W × sun-hours
3. Update UI to display energy calculations
4. Add tests for calculation accuracy
5. Update documentation
```

**Step 3: AI logs work**

```markdown
# log.md

## 2026-01-12 10:35

Started implementation of solar-output quality.

Created Quality definition:

- Key: "solar-output"
- Unit: "kWh/day"
- Formula: "sum(pieces where type=solar-panel) _ 300 _ sun-hours"

## 2026-01-12 11:20

Added tests. All passing.

## 2026-01-12 11:45

Updated Design app footer to show total solar output.
```

**Step 4: Close ticket**

```bash
repo ticket close SOLAR-CALCULATOR \
  --summary="Added solar energy calculation quality" \
  --files="js/semio/semio.ts,js/semio/sketchpad/Design.tsx"
```

**System automatically**:

- Calculates git diff (lines added/removed per file)
- Groups changes by Nx bundle
- Extracts affected sections/definitions
- Updates ticket frontmatter

**Final ticket.md frontmatter**:

```yaml
---
slug: SOLAR-CALCULATOR
status: closed
date:
  created: 2026-01-12T10:30:00Z
  finished: 2026-01-12T12:00:00Z
bundles:
  "@semio/js":
    files:
      "js/semio/semio.ts":
        sections:
          "Quality":
            definitions:
              - solarOutputQuality
            lines:
              added: 45
              removed: 2
      "js/semio/sketchpad/Design.tsx":
        sections:
          "Footer":
            lines:
              added: 23
              removed: 5
---
```

**2. CI/CD Pipeline** (Automated Checks)

**Every time you save a file**:

```bash
# Formatters (auto-fix)
prettier --write .
ruff format py/

# Linters (find problems)
tsx hooks/typescript.ts  # → reports/typescript.json
tsx hooks/eslint.ts      # → reports/eslint.json
tsx hooks/i18n.ts        # → reports/i18n.json
tsx hooks/code.ts        # → reports/code.json
```

**Before committing** (Husky pre-commit hook):

```bash
npm run preflight
  ↓
Run fix (formatters)
  ↓
Run analyze (linters)
  ↓
If problems found → Block commit
If clean → Allow commit
```

**3. Testing Strategy**

**Unit Tests** (Fast, isolated)

```typescript
// semio.test.ts
test("getDiff calculates correct piece diff", () => {
  const before = { pieces: [{ id: "p1", color: "#FFF" }] };
  const after = { pieces: [{ id: "p1", color: "#000" }] };

  const diff = getDiff(before, after);

  expect(diff.pieces.updated).toEqual([{ id: "p1", diff: { color: "#000" } }]);
});
```

**E2E Tests** (Slow, realistic)

```typescript
// design.spec.ts (Playwright)
test("user can create and connect wall pieces", async ({ page }) => {
  // 1. Open app
  await page.goto("http://localhost:5173");

  // 2. Create temporary kit
  await page.click("#semio\\.sketchpad\\.app\\.home\\.createTemporary");

  // 3. Create type
  await page.click("#semio\\.sketchpad\\.app\\.kit\\.createType");
  await page.fill("input[name=name]", "Wall Panel");

  // 4. Create design
  await page.click("#semio\\.sketchpad\\.app\\.kit\\.createDesign");

  // 5. Drag type to canvas
  await page.dragAndDrop("#type-wall-panel", "#canvas", { targetPosition: { x: 200, y: 300 } });

  // 6. Verify piece created
  await expect(page.locator(".piece")).toHaveCount(1);
});
```

---

## 📚 Glossary of Technical Terms

**For absolute beginners - every technical term explained:**

- **API** (Application Programming Interface): A menu of commands a program can use to talk to another program
  - _Example_: Restaurant menu (API) lets you order food (commands) from kitchen (program)

- **CRDT** (Conflict-free Replicated Data Type): Special data structure that automatically merges simultaneous edits
  - _Example_: Google Docs magic - two people typing at once, no conflicts

- **CLI** (Command Line Interface): Text-based way to control a computer (opposite of clicking icons)
  - _Example_: `dir` shows files (text), vs. opening File Explorer (visual)

- **Component**: Reusable piece of code or design
  - _Example_: LEGO brick = physical component, React component = code component

- **Coordinate System**: Way to describe positions in space using numbers
  - _Example_: Chess board uses letters (A-H) and numbers (1-8) to describe positions

- **Dependency**: When program A needs program B to work
  - _Example_: Your car (A) depends on gasoline (B)

- **Diff** (Difference): What changed between two versions
  - _Example_: Microsoft Word's "Track Changes" shows diffs

- **GUID** (Globally Unique Identifier): Special ID number guaranteed to be unique
  - _Example_: Like a social security number for digital objects
  - Format: `a1b2c3d4-e5f6-7890-abcd-ef1234567890`

- **JSON** (JavaScript Object Notation): Human-readable way to structure data

  ```json
  {
    "name": "John",
    "age": 30,
    "hobbies": ["reading", "coding"]
  }
  ```

- **Monorepo**: One big code repository containing many related projects
  - _Example_: One box containing all your LEGO sets vs. separate boxes for each set

- **React**: JavaScript library for building user interfaces
  - _Analogy_: Like building UI with LEGO-style components

- **REST API**: Way for programs to request data over the internet
  - _Example_: Order at drive-thru (request) → get food (response)

- **SQLite**: Small database stored in a single file
  - _Analogy_: Digital filing cabinet in one file

- **State**: Current condition of the program
  - _Example_: Light switch state = on or off

- **Three.js**: JavaScript library for 3D graphics in browsers
  - _Analogy_: Like a 3D game engine for websites

- **TypeScript**: JavaScript with type checking (catches errors before running)
  - _Analogy_: Spell-check for code

- **VSCode**: Visual Studio Code - popular code editor by Microsoft
  - _Analogy_: Microsoft Word, but for programmers

- **WebSocket**: Two-way real-time connection between browser and server
  - _Analogy_: Phone call (vs. HTTP = text messages)

- **Y.js**: Library for real-time collaborative editing
  - _Analogy_: Google Docs technology

---

## 🎓 Further Learning Paths

### If you want to understand more:

**1. For Visual Learners**

- Watch: "What is an API?" on YouTube (5 min)
- Watch: "How databases work" animations
- Interactive: Try Scratch (visual programming for kids)

**2. For Hands-On Learners**

- Try: Sketchpad demo (create a simple design)
- Try: Chrome DevTools (see how websites work)
- Build: Simple LEGO structure, photograph each step (that's "version control")

**3. For Theory Learners**

- Read: "Code" by Charles Petzold (how computers work, no prerequisites)
- Read: "The Architecture of Open Source Applications" (real-world examples)
- Course: CS50 (Harvard's intro to computer science, free online)

### Topics to Explore Next:

**Beginner Level:**

1. What is version control? (Git basics)
2. How do websites work? (HTML, CSS, JavaScript basics)
3. What is a database? (SQL basics)

**Intermediate Level:**

1. React tutorial (official React docs)
2. TypeScript handbook (typescript-lang.org)
3. REST API design principles

**Advanced Level:**

1. State management patterns (XState docs)
2. CRDT theory (Y.js research papers)
3. Compiler design (how code becomes programs)

---

## 🏁 Summary: The Essence of Semio

**In 50 words:**
Semio is digital LEGO for architects. Create reusable components (Types) with connection points (Connectors), combine them into designs (Designs), and collaborate in real-time. Works in browsers, desktop apps, and Rhino. The same core system runs in 4 programming languages for maximum compatibility.

**Key Innovation:**
Not just a 3D modeling tool, but a **design version control system** with:

- Semantic understanding of what changed (not just pixels)
- Automatic conflict resolution for collaboration
- Cross-platform compatibility (web, desktop, Rhino)
- AI-first development workflow

**Who Should Use It:**

- **Architects**: Design modular buildings
- **Designers**: Create furniture systems
- **Engineers**: Optimize structures parametrically
- **Developers**: Extend with plugins and integrations

**The Future Vision:**
Imagine an "App Store for building components" where:

- Manufacturers publish certified component kits
- Designers share innovative design patterns
- Engineers validate structural integrity automatically
- Builders generate fabrication instructions instantly

All from the same design data, in real-time collaboration, with AI assistance.

---

_Document created for absolute beginners. Questions? Open an issue or email hello@semio.dev_
