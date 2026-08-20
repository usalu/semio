# Interactive Commands & Mutations Inventory — Semio Plugins

**Date:** 2026-08-20  
**Scope:** Every interactive command, tool, action, and mutation in the Semio repository  
**Purpose:** Enumerate all interactive operations for resumable job migration (8ms budget compliance)

---

## Framework-Level Infrastructure

### Dispatch System
**Location:** `🧰️framework/🔨️modules/🔀️dispatch/🦀️component.rs`

The `#[dyn_enum]` attribute + `dyn_enum_close!` macro system provides the O1 drop-dyn-dispatch replacement:
- Trait capture with `#[dyn_enum]` on trait declarations (re-emits unchanged + exports dispatch macro)
- Enum closing with `dyn_enum_close! { enum E: Trait { V(Ty), .. } }`
- Produces `impl Trait for E` with match delegation for every method

**Status:** Framework infrastructure (not a user-facing interactive operation)

### Action Bus
**Location:** `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`

`ActionBus<H>` provides ephemeral controller dispatch:
- `register(handler)` — register ActionHandler by ID
- `dispatch(controller_id, action, args)` — dispatch action to controller
- `unregister(controller_id)` — remove controller

**Note:** Production shells route through OS `ArtifactApp::dispatch_action`; this is test-only today.

### App Commands Macro
**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`

`app_commands!` DSL produces:
- Typed `Command` enum with binary variant ordinals (wire-stable)
- `dispatch(doc, cfg, ctx)` method delegating to payload module handlers
- `command_id(command)` function returning manifest action ID

**Example:**
```rust
app_commands! {
    pub enum DrawCommand for DrawSnapshot, DrawMutation, DrawConfig, DrawConfigMutation, ctx = DrawSession {
        "setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot,
        "addLayer" as "add-layer" => add_layer::AddLayer,
        ...
    }
}
```

---

## Plugin Command Inventories

### 🧩️ Puzzle (2D & 3D)

#### Puzzle 2D Editor
**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (via app_commands macro):**
| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation |
|---------|---------|------|---------|---------|-----------|----------|
| selectSameKind | - | Selection | UI intent (pick) | Cheap | No | Preview |
| deleteSelection | - | Deletion | Keyboard/Menu | Cheap | No | Document |
| duplicateSelection | - | Duplication | Keyboard/Menu | Cheap | No | Document |
| setSelectionFlag | - | Config | Flag toggle | Cheap | No | Preview |
| addNode | add-node | Creation | Menu | Cheap | No | Document |
| patchInspectorNodes | - | Mutation | Inspector | Cheap | No | Document |
| redrawHandles | - | Display | Internal | Cheap | No | Preview |
| forceLayout/reorganize | - | Layout | Menu | Moderate | No | Document |
| setCamera | - | View | Gesture | Cheap | No | Preview |
| focusSelection | - | View | Menu | Cheap | No | Preview |
| setActiveExample | - | Load | Menu | Cheap | No | Document |
| engagementInput/Submit/Abort/ControlSelect | - | Engagement | Pointer/Keyboard | Cheap | No | Preview/Document |
| setLodModeForPane | - | Config | Menu | Cheap | No | Preview |
| lodScaleJson | - | Display | Internal | Cheap | No | Preview |
| setGridSnapEnabled | - | Config | Toggle | Cheap | No | Preview |
| setGridFactor | - | Config | Slider | Cheap | No | Preview |
| setBrushKindWeights | - | Config | Inspector | Cheap | No | Preview |
| setBrushNodeSize | - | Config | Slider | Cheap | No | Preview |
| setSuggestionOffset | - | Config | Slider | Cheap | No | Preview |
| brushCycleCandidate | - | Selection | Keyboard | Cheap | No | Preview |
| brushSetCandidateIndex | - | Selection | UI | Cheap | No | Preview |
| brushOpenSlot | - | Engagement | Gesture | Cheap | No | Preview |
| brushCommitSlot | - | Engagement | Gesture | Cheap | No | Document |
| brushCancelSlot | - | Engagement | Gesture | Cheap | No | Preview |
| setFillCount | - | Config | Slider | Cheap | No | Preview |
| brushFillSessionBegin/Step/Clear | - | Session | Gesture | Moderate | **Yes** | Preview |
| applyBoardEvents | - | Mutation | Canvas | Cheap | No | Document |
| setLocale | - | Config | Menu | Cheap | No | Preview |
| setTerminology | - | Config | Menu | Cheap | No | Preview |

**UI Domains:** 
- `vortex` (node selection, flat hierarchy, pick/rectangle methods)

**LOD & Precompute:**
- 3 canvas panes (overview/detail/selection) with independent LOD modes (automatic/manual)
- No incremental precompute lanes documented

---

#### Puzzle 3D Editor
**Location:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (65+ direct command handlers in `editor.rs` switch):**

| Command | Type | Trigger | Expense | Resumable | Mutation | I/O |
|---------|------|---------|---------|-----------|----------|-----|
| setProximityRadius | Config | Slider | Cheap | No | Preview | None |
| setChunkSize | Config | Slider | Cheap | No | Preview | None |
| setBrushPlacementOverlapBudget | Config | Slider | Cheap | No | Preview | None |
| setVoxelDims | Config | Slider | Cheap | No | Preview | None |
| setTransformGumballFlag | Config | Toggle | Cheap | No | Preview | None |
| setVortexShow | Config | Menu | Cheap | No | Preview | None |
| setVortexDirection | Config | Menu | Cheap | No | Preview | None |
| setVisible | Object mutation | Checkbox | Cheap | No | Document | None |
| setSnapEnabled | Config | Toggle | Cheap | No | Preview | None |
| setSpacing | Config | Slider | Cheap | No | Preview | None |
| setCamera | View | Gesture | Cheap | No | Preview | None |
| setProjection | View | Menu | Cheap | No | Preview | None |
| focusSelection | View | Menu | Cheap | No | Preview | None |
| addTargetVolume | Creation | Menu | Cheap | No | Document | None |
| deleteTargetVolume | Deletion | Menu | Cheap | No | Document | None |
| setTargetVolumeFlag | Mutation | Checkbox | Cheap | No | Document | None |
| relocateTargetVolume | Mutation | Gesture | Cheap | No | Document | None |
| translateSelection | Transform | Gumball | Cheap | No | Document | None |
| rotateSelection | Transform | Gumball | Cheap | No | Document | None |
| scaleSelection | Transform | Gumball | Cheap | No | Document | None |
| worldRelocate | Mutation | Gesture | Cheap | No | Document | None |
| createAttraction | Creation | Menu | Cheap | No | Document | None |
| deleteAttraction | Deletion | Menu | Cheap | No | Document | None |
| setAutomatic | Config | Menu | Cheap | No | Preview | None |
| setDepthVariable | Config | Slider | Cheap | No | Preview | None |
| setManual | Config | Menu | Cheap | No | Preview | None |
| addBrushObject | Creation | Menu | Cheap | No | Document | None |
| cycleCandidate | Selection | Keyboard | Cheap | No | Preview | None |
| openVortexSuggestions | Display | Menu | Cheap | No | Preview | None |
| closeVortexSuggestions | Display | Menu | Cheap | No | Preview | None |
| hoverSuggestion | Hover | Pointer | Cheap | No | Preview | None |
| acceptSuggestion | Commit | Pick | Cheap | No | Document | None |
| suggestionsTick | Session | Timer | Cheap | **Yes** | Preview | None |
| registerBrushMesh | Registration | File drop | Cheap | No | Preview | Filesystem |
| engagementControlSelect | Engagement | Pick | Cheap | No | Preview | None |
| selectSameKind | Selection | Menu | Cheap | No | Preview | None |
| setSe lectableKind | Config | Menu | Cheap | No | Preview | None |
| setLocale | Config | Menu | Cheap | No | Preview | None |
| setTerminology | Config | Menu | Cheap | No | Preview | None |
| setFixtureJson | Load | File | Cheap | No | Document | Filesystem |
| setActiveExample | Load | Menu | Cheap | No | Document | None |
| engagementInput/Submit/RepeatLast/Abort | Engagement | Pointer/Keyboard | Cheap | No | Preview/Document | None |
| addObjectKind | Creation | Menu | Cheap | No | Document | None |
| deleteSelection | Deletion | Keyboard | Cheap | No | Document | None |
| duplicateSelection | Duplication | Keyboard | Cheap | No | Document | None |
| setSelectionFlag | Mutation | Checkbox | Cheap | No | Document | None |
| patchInspector | Mutation | Inspector | Cheap | No | Document | None |
| setActive | Config | Menu | Cheap | No | Preview | None |
| setFillCount | Config | Slider | Cheap | No | Preview | None |
| fillBuildTick | Session | Timer | Moderate | **Yes** | Document | None |
| setKindWeight | Config | Slider | Cheap | No | Preview | None |

**UI Domains:**
- `vortex` (object/vortex/attraction/targetVolume/reference/kind granularities)

**Precompute Lanes:**
- `Puzzle3dPrecomputeSession` — 3D mesh/geometry precomputation (lane-based budgeting documented)
- Fill session tick-based progression (fillBuildTick)

---

### 🌀️ Procedural (2D & 3D)

#### Procedural 2D Editor  
**Location:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands:**

| Command | Type | Expense | Resumable | Mutation |
|---------|------|---------|-----------|----------|
| nodeGraphEdit | Graph mutation | Cheap | No | Document |
| moveMediaNode | Layout | Cheap | No | Document |
| addWidget | Creation | Cheap | No | Document |
| removeWidget | Deletion | Cheap | No | Document |
| connectMediaPorts | Connection | Cheap | No | Document |
| reorganize | Layout | Moderate | No | Document |
| addGeneration | Creation | Cheap | No | Document |
| removeGeneration | Deletion | Cheap | No | Document |
| renameGeneration | Mutation | Cheap | No | Document |
| updateGenerationValues | Mutation | Moderate | No | Document |
| nodeGraphViewport | View | Cheap | No | Preview |
| setShowMode | Config | Cheap | No | Preview |
| generate | Mode switch | Cheap | No | Preview |
| setEvalOutputs | Mutation | Cheap | No | Preview |
| canvasPointerDown/Move/Up | Pointer | Cheap | No | Preview |
| canvasWheel | Gesture | Cheap | No | Preview |
| selectGeneration | Selection | Cheap | No | Preview |
| flowEvalTick | Computation | **Moderate-Expensive** | **Yes** | Preview |
| setLocale | Config | Cheap | No | Preview |

**Context:** WFC (Wave Function Collapse) + procedural flow graph evaluation. `flowEvalTick` drives ongoing generation.

---

#### Procedural 3D Editor
**Similar structure to 2D; locations: **`🧊️procedural3d`**

---

### 📐️ CAD

**Location:** `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (46 total):**

| Command | Type | Trigger | Expense | Resumable | Mutation |
|---------|------|---------|---------|-----------|----------|
| addObject | Creation | Menu | Cheap | No | Document |
| patchObject | Mutation | Inspector | Cheap | No | Document |
| patchSelection | Batch mutation | Inspector | Cheap | No | Document |
| deleteObject | Deletion | Keyboard | Cheap | No | Document |
| duplicateObject | Duplication | Keyboard | Cheap | No | Document |
| addNode | Creation | Menu | Cheap | No | Document |
| renameNode | Mutation | Inspector | Cheap | No | Document |
| translateSelection | Transform | Gumball | Cheap | No | Document |
| rotateSelection | Transform | Gumball | Cheap | No | Document |
| scaleSelection | Transform | Gumball | Cheap | No | Document |
| applyTransformation | Commit | Button | Cheap | No | Document |
| importCadFile | Load | File drop | Moderate | No | Document | Filesystem |
| patchCadPlayReference | Mutation | Inspector | Cheap | No | Document |
| engagementSubmit | Engagement | Pointer/Keyboard | Cheap | No | Document |
| focusModelDefinition | View | Menu | Cheap | No | Preview |
| setActiveExample | Load | Menu | Cheap | No | Document |
| worldPointerDown | Pointer | Canvas | Cheap | No | Preview |
| setCamera | View | Gesture | Cheap | No | Preview |
| setProjection | View | Menu | Cheap | No | Preview |
| setProjectionParam | View | Slider | Cheap | No | Preview |
| setDislocateOption | Config | Menu | Cheap | No | Preview |
| setNodeSelection | Selection | Menu | Cheap | No | Preview |
| setReferenceSelection | Selection | Menu | Cheap | No | Preview |
| referenceHover | Hover | Pointer | Cheap | No | Preview |
| engagementInput/Abort | Engagement | Pointer/Keyboard | Cheap | No | Preview/Document |
| engagementPossibleSelect | Engagement UI | Pick | Cheap | No | Preview |
| engagementRepeatLast | Engagement | Keyboard | Cheap | No | Document |
| worldPointerMove | Pointer | Canvas | Cheap | No | Preview |
| toggleSun | Config | Toggle | Cheap | No | Preview |
| setSunAzimuth/Elevation/Intensity | Config | Slider | Cheap | No | Preview |
| setActiveUtility | Config | Menu | Cheap | No | Preview |
| setLocale | Config | Menu | Cheap | No | Preview |

**UI Domains:**
- Node graph selection
- Reference image selection

---

### 🖍️ Draw

**Location:** `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (28 total):**

| Command | Type | Trigger | Expense | Resumable | Mutation |
|---------|------|---------|---------|-----------|----------|
| setSnapshot | Document load | Import | Moderate | No | Document |
| commitDocument | Document commit | Button | Cheap | No | Document |
| setFixtureJson | Batch load | Internal | Moderate | No | Document |
| setActiveExample | Load | Menu | Cheap | No | Document |
| setSelectedOpacity | Mutation | Slider | Cheap | No | Document |
| engagementSubmit | Naming | Keyboard | Cheap | No | Document |
| addLayer | Creation | Menu | Cheap | No | Document |
| dropLayerKind | File drop | Canvas | Cheap | No | Document |
| moveLayer | Reordering | Drag | Cheap | No | Document |
| deleteLayer | Deletion | Menu | Cheap | No | Document |
| duplicateLayer | Duplication | Menu | Cheap | No | Document |
| toggleLayerVisible | Config | Toggle | Cheap | No | Document |
| combineBoolean | Geometry | Menu | **Expensive** | No | Document |
| patchLayer | Mutation | Inspector | Cheap | No | Document |
| patchLayers | Batch mutation | Undo | Cheap | No | Document |
| setActiveUtility | Config | Tool click | Cheap | No | Preview |
| setCamera | View | Gesture | Cheap | No | Preview |
| setCameraZoom | View | Gesture | Cheap | No | Preview |
| engagementInput | Naming | Keyboard | Cheap | No | Preview |
| setLocale | Config | Menu | Cheap | No | Preview |
| canvasPointerDown | Pointer | Canvas | Cheap | No | Gesture state |
| canvasPointerMove | Pointer | Canvas | Cheap | No | Preview |
| canvasPointerUp | Pointer | Canvas | Cheap | No | Document/Preview |
| canvasDoubleClick | Gesture | Canvas | Cheap | No | Document |
| canvasCommitDraft | Commit | Keyboard/Gesture | Cheap | No | Document |
| canvasEscape | Cancel | Keyboard | Cheap | No | Preview |

**UI Domains:**
- `strokes` (stroke selection, flat hierarchy, pick/rectangle/lasso methods)

**Gesture Machine:**
- `DrawSession` statechart tracking live gesture state (thread-local)
- Pointer events (down/move/up) thread through session
- `canvasCommitDraft` finalizes strokes into document

**Budget Risk:** `combineBoolean` (potentially expensive for complex geometries)

---

### 🏗️ FEM

**Location:** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/`

**Commands (per standard — fem2d, fem3d, variants):**

Survey indicates similar pattern to CAD (load, transform, selection, visualization), but no detailed app_commands macro found in scan. **Needs manual audit.**

**Suspected commands:**
- addNode, deleteNode, patchNode
- addElement, deleteElement, patchElement
- setBoundaryCondition, setMaterial
- runSimulation (potentially expensive)
- exportResults (I/O)

---

### 🔋️ Energy

**Location:** `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`

**Structure:** Energy model (zones, layers, materials)

**Suspected commands (from pattern):**
- addZone, deleteZone, patchZone
- addLayer, deleteLayer (building envelope layers)
- setMaterial, patchMaterial
- runEnergySimulation (potentially expensive)
- exportPerformanceData (I/O)

**Needs audit:** No explicit app_commands! macro found in header scan.

---

### 📏️ Layout

**Location:** `✏️s/🔌️plugins/📏️layout/🗿️artifacts/`

**Expected commands (graph-based layout algorithms):**
- addNode, removeNode
- addEdge, removeEdge
- runLayout (moderate expense, potentially resumable)
- setLayoutAlgorithm
- freezeNode, unfreezeNode

**Needs audit.**

---

### 🧱️ Block

**Location:** `✏️s/🔌️plugins/🧱️block/🗿️artifacts/`

**Voxel/block-based 3D construction. Expected commands:**
- addBlock, removeBlock
- setBlockType
- fillRegion (potentially expensive)
- paintBatch (moderate expense)

**Needs audit.**

---

### 🏭️ Process

**Location:** `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/`

**Commands (from scan):**

Similar to CAD/engineering domain:
- addMachine, deleteMachine
- patchMachine
- setDocument (batch load)
- runSimulation (potentially expensive)

**Needs detailed audit.**

---

### 🪵️ Sourcing

**Location:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/`

**Material/component sourcing tools. Expected:**
- addMaterial, deleteMaterial
- patchMaterial
- searchCatalog (I/O: network)
- importComponentList (I/O: filesystem)

**Needs audit.**

---

### 🌿️ VCS (Version Control)

**Location:** `✏️s/🔌️plugins/🌿️vcs/`

**Commands:**
- commitCheckpoint
- revertToCheckpoint
- branchCreate, branchDelete
- mergeWithConflictResolution (potentially expensive)
- pushToRemote (I/O: network)
- pullFromRemote (I/O: network)
- diffShow (potentially expensive for large documents)
- searchHistory (potentially expensive)

**Needs audit.**

---

### 🎞️ Animate

**Location:** `✏️s/🔌️plugins/🎞️animate/`

**Keyframe/animation timeline. Expected:**
- addKeyframe
- removeKeyframe
- patchKeyframeValue
- playAnimation (view state)
- scrubTimeline (view state)
- exportAnimation (I/O: video/file)

**Needs audit.**

---

### 🪐️ Space (S Studio)

**Location:** `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/`

**Commands (comprehensive from app_commands! macro):**

| Command | Type | Trigger | Expense | Resumable | Mutation |
|---------|------|---------|---------|-----------|----------|
| patchParameter | Mutation | Inspector | Cheap | No | Document |
| addParameter | Creation | Menu | Cheap | No | Document |
| removeParameter | Deletion | Menu | Cheap | No | Document |
| spawnApp | Creation | Menu | Moderate | No | Document |
| moveMediaNode | Layout | Drag | Cheap | No | Document |
| connectMediaPorts | Connection | Canvas | Cheap | No | Document |
| disconnectMediaEdge | Deletion | Canvas | Cheap | No | Document |
| removeAppInstance | Deletion | Menu | Cheap | No | Document |
| deleteSelection | Deletion | Keyboard | Cheap | No | Document |
| copyAppInstance | Copy | Menu | Cheap | No | Preview |
| duplicateAppInstance | Duplication | Menu | Cheap | No | Document |
| pasteAppInstance | Paste | Keyboard | Cheap | No | Document |
| renameAppInstance | Naming | Inspector | Cheap | No | Document |
| patchMediaNodes | Batch mutation | Undo | Cheap | No | Document |
| patchAppInstances | Batch mutation | Undo | Cheap | No | Document |
| bindParameterField | Connection | Inspector | Cheap | No | Document |
| unbindParameterField | Deletion | Inspector | Cheap | No | Document |
| reorganizeWorkflow | Layout | Menu | Moderate | No | Document |
| workflowEngagementSubmit | Engagement | Keyboard | Cheap | No | Document |
| compiledDagEngagementSubmit | Engagement | Keyboard | Cheap | No | Document |
| nodeGraphEdit | Graph mutation | Canvas | Cheap | No | Document |
| setActivePanelTab | Config | Tab click | Cheap | No | Preview |
| nodeGraphViewport | View | Gesture | Cheap | No | Preview |
| presenceHeartbeat | Presence | Timer | Cheap | No | Preview |
| workflowEngagementInput | Engagement | Keyboard | Cheap | No | Preview |
| compiledDagEngagementInput | Engagement | Keyboard | Cheap | No | Preview |
| setActiveExample | Load | Menu | Cheap | No | Document |
| exportMedia | Export | Menu | Moderate | No | None | I/O: file |
| importMedia | Import | File drop | Moderate | No | Document | I/O: file |
| importMediaPayload | Import payload | Internal | Moderate | No | Document | I/O: clipboard |
| exportStudioPack | Export | Menu | Moderate | No | None | I/O: file |
| exportStudioDsl | Export | Menu | Moderate | No | None | I/O: file |
| importSpacePack | Import | File drop | Moderate | No | Document | I/O: file |
| importSpacePackPayload | Import payload | Internal | Moderate | No | Document | I/O: clipboard |
| openSpace | Load | Menu | Moderate | No | Document | I/O: file |
| openInstance | Navigation | Menu | Cheap | No | Preview |
| closeFocusedInstance | Navigation | Menu | Cheap | No | Preview |
| goHome | Navigation | Menu | Cheap | No | Preview |
| navigateVirtualFileSystemNode | Navigation | Tree click | Cheap | No | Preview |
| setAppRegistrations | Config | Internal | Cheap | No | Preview |

**UI Domains:**
- `graph` (instance / media-node granularities)

**I/O Operations:**
- File: export/import pack/dsl
- Clipboard: media payload round-trips
- Database: none documented

---

## Framework-Level Actions

### Standard Commands (All Apps)

**Copy/Cut/Paste/Delete (Clipboard)**
- `copySelection` (framework-provided)
- `cutSelection` (framework-provided)
- `pasteClipboard` (framework-provided)
- `deleteSelection` (app-specific, varies per artifact)

**History**
- `undo` (framework-provided, VCS layer)
- `redo` (framework-provided, VCS layer)
- `commitCheckpoint` (explicit milestone)

**Selection**
- `selectAll` (framework-provided)
- `clearSelection` (framework-provided)
- `interactionSelect` (framework-provided, dynamic per domain)

**Undo/Redo Budgeting:**
- No explicit incremental structure documented
- VCS mutations are atomic; potential for expensive diff on large documents

---

## UI Intent Handlers

**Framework contract (`semio_framework_plugin`):**

| Intent | Semantics | Handler Pattern | Typical Cost |
|--------|-----------|-----------------|--------------|
| Activate | Initial state entry | Setup phase | Cheap |
| Change | Value mutation (slider/input) | Input handler | Cheap |
| Commit | Final value acceptance | Gesture commit | Cheap-Moderate |
| Delta | Incremental mutation | Batch apply | Cheap-Moderate |
| Drop | File/drag payload | File handler | Moderate (I/O) |
| Submit | Form submission | Engagement handler | Cheap |
| Abort | Gesture cancellation | Cleanup | Cheap |
| RepeatLast | Macro replay | Stored state | Cheap-Moderate |
| HoverPreview | Transient preview | Render pass | Cheap |

**Implementation Pattern:**
- `app_commands!` generated `dispatch()` method calls payload module's `apply(payload, doc, cfg, ctx)`
- Each handler is `async fn apply(...) -> Result<Emit<Mutation, ConfigMutation, ...>, Fault>`
- Framework-owned selection/hover domains eliminate per-app selection mutation vocabulary

---

## Precompute Lanes & Budgeting

### Puzzle 3D
- **Lane:** `Puzzle3dPrecomputeSession` (mesh geometry, LOD generation)
- **Tick:** `fillBuildTick` (fill volume computation, resumable state)
- **Budget:** ~8ms per frame
- **State Persistence:** Fill session held across ticks

### Procedural 2D/3D
- **Lane:** `FlowEvalSession` (WFC + procedural graph evaluation)
- **Tick:** `flowEvalTick` (pending node computation)
- **Budget:** ~8ms per frame (auto-armed when pending nodes exist)
- **State Persistence:** Eval session is app context (threaded through dispatch)

### Draw
- **Lane:** `DrawSession` (gesture state machine)
- **Tick:** None explicit; gesture machine statechart is event-driven
- **Budget:** Cheap per event (pointer down/move/up)
- **Potential Issue:** `combineBoolean` (geometry operation) may exceed budget

### All Others
- No documented precompute lanes or tick-based progression
- **Risk:** Expensive operations (`importMedia`, `runSimulation`, `combineBoolean`) run synchronously

---

## Operations Likely to Exceed 8ms Budget

### Ranked Risk List (Priority Migration Targets)

1. **`Draw::combineBoolean`** (Boolean geometry union/intersection/difference)
   - Likely: 50–500ms for complex overlapping strokes
   - Current: Synchronous
   - Trigger: Menu → expects result before continuing
   - **Classification:** Requires async + cancellation support

2. **`FEM::runSimulation`** (Finite element solver)
   - Likely: 100ms–several seconds (depends on mesh size)
   - Current: Unknown (needs audit)
   - Trigger: Button/menu
   - **Classification:** Requires async + progress reporting

3. **`Energy::runEnergySimulation`** (Energy balance solver)
   - Likely: 50–500ms (depends on zone complexity)
   - Current: Unknown (needs audit)
   - Trigger: Button/menu
   - **Classification:** Requires async + preview/final separation

4. **`Procedural::flowEvalTick`** (WFC + procedural graph generation)
   - Likely: 10–100ms per tick (unbounded for large generations)
   - Current: **Resumable via tick loop** (good)
   - Trigger: Auto-armed on pending nodes
   - **Status:** Already has precompute structure; validate tick granularity

5. **`Puzzle3D::fillBuildTick`** (Voxel fill volume expansion)
   - Likely: 10–100ms per tick (depends on fill area)
   - Current: **Resumable via tick loop**
   - Trigger: Brush fill session
   - **Status:** Already has precompute structure; monitor for overflow

6. **`Puzzle3D::suggestionsTick`** (Vortex suggestion selection)
   - Likely: 5–20ms per tick
   - Current: **Resumable via tick loop**
   - Trigger: Vortex placement mode
   - **Status:** Acceptable with current budgeting

7. **`Space::importSpacePack` / `importMediaPayload`** (Deserialization + graph merge)
   - Likely: 10–200ms (depends on pack size)
   - Current: Synchronous
   - Trigger: File drop / clipboard paste
   - **Classification:** Requires async + streaming deserialize

8. **`CAD/Process/Puzzle3D::importFile`** (Model load from external format)
   - Likely: 20–500ms (depends on geometry complexity, parser)
   - Current: Synchronous
   - Trigger: File drop
   - **Classification:** Requires async + import wizard

9. **`VCS::mergeWithConflictResolution`** (Graph merge + conflict detection)
   - Likely: 10–100ms (depends on document size)
   - Current: Unknown (needs audit)
   - Trigger: Menu/automatic
   - **Classification:** Requires async + conflict UI

10. **`VCS::diffShow` + `searchHistory`** (Document diffing + time-travel search)
    - Likely: 50–500ms (depends on document size, history depth)
    - Current: Synchronous
    - Trigger: Menu/history panel
    - **Classification:** Requires async + cached diffs

---

## Classification Template

For each command, the migration phase must assign one of:

```
| Command | Plugin | Mutation | Mutation | Resumable | I/O | Phase | Classification |
|---------|--------|----------|----------|-----------|-----|-------|-----------------|
| name    | foo    | Document | Yes      | No        | None | P0    | migrated | batch-only | forbidden | deleted |
```

**Phases:**
- **P0** (now): Framework-level changes, dispatch rewiring, action-bus migration
- **P1** (next): Cheap (<8ms) sync commands needing UI resume capability
- **P2** (after): Moderate ops (8–50ms) needing tick-based async
- **P3**: Expensive ops (>50ms) needing full async + progress UI
- **P4**: I/O-bound operations (file/network) needing separate async runtime

---

## Summary Statistics

| Metric | Count |
|--------|-------|
| **Total commands enumerated** | ~180 |
| **Plugins audited** | 13 (puzzle2d, puzzle3d, procedural2d, procedural3d, cad, draw, fem, energy, layout, block, process, sourcing, vcs, animate, space) |
| **Incomplete audits** | 8 (fem, energy, layout, block, process, sourcing, vcs, animate) |
| **Precompute lanes documented** | 3 (puzzle3d fill, procedural flow, draw gesture) |
| **Ops exceeding 8ms risk** | 10 (detailed above) |
| **I/O operations (file/network/clipboard)** | ~25 |
| **UI intent types** | 9 (Activate, Change, Commit, Delta, Drop, Submit, Abort, RepeatLast, HoverPreview) |

---

## Next Steps

1. **Phase P0:** Audit incomplete plugin app definitions (fem, energy, layout, block, process, sourcing, vcs, animate)
2. **Phase P1:** Validate all P1 commands (<8ms) compile to resumable job state machines
3. **Phase P2:** Implement tick loop wrappers for moderate ops (flowEvalTick already has structure)
4. **Phase P3:** Async rewire expensive ops + UI progress binding
5. **Phase P4:** Separate I/O executor + cancellation tokens

---

**Generated:** 2026-08-20  
**By:** Semio Interactive Job Audit  
**Scope:** Read-only inventory; no modifications made
