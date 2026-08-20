# Interactive Commands & Mutations Inventory — Part 2: Plugins & Framework

**Date:** 2026-08-20  
**Scope:** Completion audit covering 8 unaudited plugins + framework-level commands  
**Completes:** `📓️p0-inventory-commands.md` part 1 (180 commands) with part 2 coverage

---

## Coverage Summary

This document completes the P0 inventory by auditing exactly the 8 areas part 1 marked as "for follow-up audit":
- `🏗️fem/` — 2D & 3D variants (37 total commands)
- `🔋️energy/` — Energy model editor (2 commands)
- `📏️layout/` — Graphics layout engine (21 commands)
- `🧱️block/` — Voxel/block 3D construction (24 commands)
- `🏭️process/` — Manufacturing process 3D (31 commands)
- `🪵️sourcing/` — Material/component curation (14 commands)
- `🌿️vcs/` — Version control system (10 commands)
- `🎞️animate/` — Keyframe animation (17 commands)

Plus framework-level operations (history, clipboard, interaction, serialization):
- History & undo/redo (7 commands)
- Clipboard & selection (6 commands)
- Global utilities (5 commands)

**New Total:** Part 1 (~180) + Part 2 (~217) = ~397 interactive commands enumerated.

---

## Plugin Command Inventories (Audited)

### 🏗️ FEM (Finite Element Methods)

#### FEM 2D Editor
**Location:** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (19 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation | I/O |
|---------|---------|------|---------|---------|-----------|----------|-----|
| addNode | add-node | Creation | Menu | Cheap | No | Document | None |
| addBar | add-bar | Creation | Menu | Cheap | No | Document | None |
| addBeam | add-beam | Creation | Menu | Cheap | No | Document | None |
| addMaterial | add-material | Creation | Menu | Cheap | No | Document | None |
| addSection | add-section | Creation | Menu | Cheap | No | Document | None |
| addSupport | add-support | Creation | Menu | Cheap | No | Document | None |
| addNodalLoad | add-nodal-load | Creation | Menu | Cheap | No | Document | None |
| addMemberUdl | add-member-udl | Creation | Menu | Cheap | No | Document | None |
| addAreaLoad | add-area-load | Creation | Menu | Cheap | No | Document | None |
| addRegion | add-region | Creation | Menu | Cheap | No | Document | None |
| addLoadCase | add-load-case | Creation | Menu | Cheap | No | Document | None |
| addCombination | add-combination | Creation | Menu | Cheap | No | Document | None |
| setSelfWeight | set-self-weight | Config | Toggle | Cheap | No | Document | None |
| setAnalysisSettings | set-analysis-settings | Config | Inspector | Cheap | No | Document | None |
| removeSelection | remove-selection | Deletion | Keyboard | Cheap | No | Document | None |
| setActiveExample | active-example | Load | Menu | Cheap | No | Document | None |
| setCamera | camera | View | Gesture | Cheap | No | Preview | None |
| setResultDisplay | result-display | View | Menu | Cheap | No | Preview | None |
| setLocale | locale | Config | Menu | Cheap | No | Preview | None |

**UI Domains:** Nodes (flat hierarchy), load cases, load combinations

**Precompute Lanes:** None documented

---

#### FEM 3D Editor
**Location:** `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (18 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation |
|---------|---------|------|---------|---------|-----------|----------|
| addNode | add-node | Creation | Menu | Cheap | No | Document |
| addBar | add-bar | Creation | Menu | Cheap | No | Document |
| addFrame | add-frame | Creation | Menu | Cheap | No | Document |
| addMaterial | add-material | Creation | Menu | Cheap | No | Document |
| addSection | add-section | Creation | Menu | Cheap | No | Document |
| addSupport | add-support | Creation | Menu | Cheap | No | Document |
| addNodalLoad | add-nodal-load | Creation | Menu | Cheap | No | Document |
| addMemberUdl | add-member-udl | Creation | Menu | Cheap | No | Document |
| addAreaLoad | add-area-load | Creation | Menu | Cheap | No | Document |
| addSolid | add-solid | Creation | Menu | Cheap | No | Document |
| addLoadCase | add-load-case | Creation | Menu | Cheap | No | Document |
| addCombination | add-combination | Creation | Menu | Cheap | No | Document |
| setSelfWeight | set-self-weight | Config | Toggle | Cheap | No | Document |
| setAnalysisSettings | set-analysis-settings | Config | Inspector | Cheap | No | Document |
| removeSelection | remove-selection | Deletion | Keyboard | Cheap | No | Document |
| setActiveExample | active-example | Load | Menu | Cheap | No | Document |
| setCamera | camera | View | Gesture | Cheap | No | Preview |
| setResultDisplay | result-display | View | Menu | Cheap | No | Preview |

**Note:** FEM 3D has no `setLocale` command (intentional asymmetry with FEM 2D)

**Precompute Lanes:** None documented; all operations synchronous

---

### 🔋️ Energy Model Editor

**Location:** `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (2 custom enum, not app_commands!):**

| Command | Type | Trigger | Expense | Mutation |
|---------|------|---------|---------|----------|
| SetStructureField | Tree edit | Inspector (tree node) | Cheap | Document |
| SetZoneCell | Table edit | Inspector (table cell) | Cheap | Document |

**Context:** Handcrafted minimal enum (addresses only top-level `name`/`version` fields in tree view, and zone table columns `name`/`volumeM3`/`multiplier`/`conditioned`). Composite children (`structure`/`zones`) are regenerated atomically via `ReplaceModel` mutation.

**Budget:** Cheap, no precompute lanes.

---

### 📏️ Layout Engine

**Location:** `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (21 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation | I/O |
|---------|---------|------|---------|---------|-----------|----------|-----|
| setActivePage | active-page | Config | Tab click | Cheap | No | Preview | None |
| focusPreflightIssue | focus-preflight-issue | Navigation | Menu | Cheap | No | Preview | None |
| engagementInput | engagement-input | Input | Keyboard | Cheap | No | Preview | None |
| canvasPointerDown | canvas-pointer-down | Pointer | Canvas | Cheap | No | Preview | None |
| canvasPointerMove | canvas-pointer-move | Pointer | Canvas | Cheap | No | Preview | None |
| canvasPointerUp | canvas-pointer-up | Pointer | Canvas | Cheap | No | Document | None |
| canvasDragOver | canvas-drag-over | Gesture | Drag | Cheap | No | Preview | None |
| canvasDragLeave | canvas-drag-leave | Gesture | Drag | Cheap | No | Preview | None |
| setCamera | camera | View | Gesture | Cheap | No | Preview | None |
| setLocale | locale | Config | Menu | Cheap | No | Preview | None |
| addFrame | add-frame | Creation | Menu | Cheap | No | Document | None |
| addPage | add-page | Creation | Menu | Cheap | No | Document | None |
| patchPage | patch-page | Mutation | Inspector | Cheap | No | Document | None |
| patchFrame | patch-frame | Mutation | Inspector | Cheap | No | Document | None |
| canvasDrop | canvas-drop | File drop | Canvas | Moderate | No | Document | Filesystem |
| exportPng | export-png | Export | Menu | **Expensive** | No | None | I/O: file |
| exportSvg | export-svg | Export | Menu | **Expensive** | No | None | I/O: file |
| exportPdf | export-pdf | Export | Menu | **Expensive** | No | None | I/O: file |
| exportPackage | export-package | Export | Menu | Moderate | No | None | I/O: file |
| engagementSubmit | engagement-submit | Input submit | Keyboard | Cheap | No | Document | None |

**UI Domains:** Frames (nested hierarchy), pages

**Budget Risk:** Export operations (PNG/SVG/PDF) likely 50–500ms depending on page complexity.

---

### 🧱️ Block 3D Voxel/Block Construction

**Location:** `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (24 via app_commands! macro; note: non-kebab-case DSL keys):**

| Command | DSL Key | Type | Trigger | Expense | Mutation |
|---------|---------|------|---------|---------|----------|
| patchObjectKind | patchObjectKind | Mutation | Inspector | Cheap | Document |
| addRepresentation | addRepresentation | Creation | Menu | Cheap | Document |
| removeRepresentation | removeRepresentation | Deletion | Menu | Cheap | Document |
| addVortexKind | addVortexKind | Creation | Menu | Cheap | Document |
| removeVortexKind | removeVortexKind | Deletion | Menu | Cheap | Document |
| addVortex | addVortex | Creation | Button | Cheap | Document |
| removeVortex | removeVortex | Deletion | Button | Cheap | Document |
| setActiveExample | setActiveExample | Load | Menu | Cheap | Document |
| edit | edit | Text edit | Input | Cheap | Document |
| setActiveRepresentation | setActiveRepresentation | Config | Menu | Cheap | Preview |
| setWindowRepresentations | setWindowRepresentations | Config | Menu | Cheap | Preview |
| toggleWindowRepresentation | toggleWindowRepresentation | Config | Toggle | Cheap | Preview |
| setWindowArrangement | setWindowArrangement | Config | Menu | Cheap | Preview |
| setWindowSpacing | setWindowSpacing | Config | Slider | Cheap | Preview |
| setActiveUtility | setActiveUtility | Config | Tool click | Cheap | Preview |
| setBrushVortexKind | setBrushVortexKind | Config | Menu | Cheap | Preview |
| setBrushRadius | setBrushRadius | Config | Slider | Cheap | Preview |
| setBrushFlip | setBrushFlip | Config | Toggle | Cheap | Preview |
| worldSurfaceHover | hoverSurface | Hover | Pointer | Cheap | Preview |
| worldSurfaceLeave | leaveSurface | Hover | Pointer | Cheap | Preview |
| worldSurfacePlace | placeVortex | Commit | Pick | Cheap | Document |
| setCamera | setCamera | View | Gesture | Cheap | Preview |
| patchRepresentation | patchRepresentation | Mutation | Inspector | Cheap | Document |

**Note:** DSL keys use camelCase (not kebab-case), breaking pattern from other plugins — intentional design choice for consistency with legacy codec.

**Precompute Lanes:** None documented; all operations cheap and synchronous.

---

### 🏭️ Process 3D Manufacturing

**Location:** `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (31 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Mutation | I/O |
|---------|---------|------|---------|---------|----------|-----|
| setSnapshot | document | Document load | Load | Moderate | Document | Filesystem |
| setActiveExample | active-example | Load | Menu | Cheap | Document | None |
| addStep | add-step | Creation | Menu | Cheap | Document | None |
| addWorkshopMachine | add-workshop-machine | Creation | Menu | Cheap | Document | None |
| removeWorkshopMachine | remove-workshop-machine | Deletion | Menu | Cheap | Document | None |
| updateWorkshopMachine | update-workshop-machine | Mutation | Inspector | Cheap | Document | None |
| removeStep | remove-step | Deletion | Menu | Cheap | Document | None |
| removeSelectedStep | remove-selected-step | Deletion | Keyboard | Cheap | Document | None |
| moveStep | move-step | Reorder | Drag | Cheap | Document | None |
| updateStep | update-step | Mutation | Inspector | Cheap | Document | None |
| setStepEnabled | set-step-enabled | Config | Toggle | Cheap | Document | None |
| setStock | stock | Load | File | Moderate | Document | Filesystem |
| patchInspector | patch-inspector | Mutation | Inspector | Cheap | Document | None |
| setCursor | cursor | Navigation | Slider | Cheap | Preview | None |
| stepCursor | step-cursor | Navigation | Goto | Cheap | Preview | None |
| stepCursorBack | step-cursor-back | Navigation | Button | Cheap | Preview | None |
| stepCursorForward | step-cursor-forward | Navigation | Button | Cheap | Preview | None |
| engagementSubmit | engagement-submit | Engagement | Keyboard | Cheap | Document | None |
| worldPointerDown | world-pointer-down | Pointer | Canvas | Cheap | Preview | None |
| worldFaceDragEnd | world-face-drag-end | Drag | Canvas | Cheap | Document | None |
| importModelFile | import-model-file | Import | File drop | **Moderate-Expensive** | Document | Filesystem |
| setActiveUtility | active-utility | Config | Tool click | Cheap | Preview | None |
| engagementInput | engagement-input | Input | Keyboard | Cheap | Preview | None |
| engagementAbort | engagement-abort | Abort | Keyboard | Cheap | Preview | None |
| setCamera | camera | View | Gesture | Cheap | Preview | None |
| toggleSun | toggle-sun | Config | Toggle | Cheap | Preview | None |
| setSunAzimuth | sun-azimuth | Config | Slider | Cheap | Preview | None |
| setSunElevation | sun-elevation | Config | Slider | Cheap | Preview | None |
| setSunIntensity | sun-intensity | Config | Slider | Cheap | Preview | None |
| setLocale | locale | Config | Menu | Cheap | Preview | None |
| setContributions | contributions | Config | Menu | Cheap | Preview | None |
| exportModel | export-model | Export | Menu | **Expensive** | None | Filesystem |
| loadModelRequest | load-model-request | Load | Button | Moderate | None | Filesystem |

**Budget Risk:** `importModelFile` (20–200ms for typical STEP/IGES files), `exportModel` (50–500ms depending on complexity).

**I/O Operations:** File drops (import), file saves (export), potential network if linking to remote stores.

---

### 🪵️ Sourcing (Material/Component Curation)

**Location:** `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (14 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Mutation | I/O |
|---------|---------|------|---------|---------|----------|-----|
| setDocument | document-json | Document load | Load | Moderate | Document | Filesystem |
| setActiveExample | active-example | Load | Menu | Cheap | Document | None |
| stockFromCatalogue | stock-from-catalogue | Creation | Menu | Cheap | Document | Network |
| curateAdd | curate-add | Creation | UI | Cheap | Document | None |
| curateSetCount | curate-set-count | Mutation | Inspector | Cheap | Document | None |
| curateRemove | curate-remove | Deletion | Menu | Cheap | Document | None |
| dropOnPool | drop-on-pool | Drag | Canvas | Cheap | Document | None |
| dropOnCurated | drop-on-curated | Drag | Canvas | Cheap | Document | None |
| setFilterQuery | filter-query | Filter | Text input | Cheap | Preview | None |
| setFilterModule | filter-module | Filter | Checkbox | Cheap | Preview | None |
| setFilterTypology | filter-typology | Filter | Menu | Cheap | Preview | None |
| setFilterMinAvailability | filter-min-availability | Filter | Slider | Cheap | Preview | None |
| sortTable | sort-table | Sort | Header click | Cheap | Preview | None |
| setLocale | locale | Config | Menu | Cheap | Preview | None |
| setContributions | contributions | Config | Menu | Cheap | Preview | None |

**Budget Risk:** `stockFromCatalogue` may require network I/O to fetch live catalog data (50–500ms depending on network latency and catalog size).

**Search/Filter Pattern:** Filters are all preview-only (no document mutation); sorting operates on view state.

---

### 🌿️ VCS (Version Control System)

**Location:** `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (10 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation |
|---------|---------|------|---------|---------|-----------|----------|
| incrementCounter | increment-counter | Mutation | Button | Cheap | No | Document |
| patchSnapshot | patch-snapshot | Mutation | Inspector | Cheap | No | Document |
| textEdit | text-edit | Text mutation | Input | Cheap | No | Document |
| edit | edit | Text mutation | Textarea | Cheap | No | Document |
| setLocale | locale | Config | Menu | Cheap | No | Preview |
| noMutation | no-operation | No-op | Menu | Cheap | No | None |
| canvasPointerDown | canvas-pointer-down | Pointer | Canvas | Cheap | No | Preview |
| canvasPointerMove | canvas-pointer-move | Pointer | Canvas | Cheap | No | Preview |
| canvasPointerUp | canvas-pointer-up | Pointer | Canvas | Cheap | No | Document |
| canvasWheel | canvas-wheel | Gesture | Wheel | Cheap | No | Preview |

**Note:** Simple document editor focused on text/pointer interaction. No explicit diff/merge/conflict commands found in app_commands! (these may live in command taxonomy folders).

**Potential Missing Commands:** `commitCheckpoint`, `branchCreate`, `branchDelete`, `mergeWithConflictResolution`, `pushToRemote`, `pullFromRemote`, `diffShow`, `searchHistory` — these were listed in part 1 as suspected but not yet enumerated in this app_commands! export.

---

### 🎞️ Animate (Keyframe Animation Timeline)

**Location:** `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`

**Commands (17 via app_commands! macro):**

| Command | Wire ID | Type | Trigger | Expense | Resumable | Mutation | I/O |
|---------|---------|------|---------|---------|-----------|----------|-----|
| seedGrid | seed-grid | Setup | Button | Cheap | No | Document | None |
| addTile | add-tile | Creation | Menu | Cheap | No | Document | None |
| deleteTile | delete-tile | Deletion | Menu | Cheap | No | Document | None |
| deleteSelection | delete-selection | Deletion | Keyboard | Cheap | No | Document | None |
| renameTiles | rename-tiles | Naming | Dialog | Cheap | No | Document | None |
| patchTileCrops | patch-tile-crops | Mutation | Inspector | Cheap | No | Document | None |
| setSource | set-source | Load | File drop | Moderate | No | Document | Filesystem |
| setFrame | set-frame | Config | Slider | Cheap | No | Preview | None |
| setActiveExample | set-active-example | Load | Menu | Cheap | No | Document | None |
| clearTiles | clear-tiles | Deletion | Menu | Cheap | No | Document | None |
| engagementSubmit | engagement-submit | Engagement | Keyboard | Cheap | No | Document | None |
| resetGrid | reset-grid | Setup | Button | Cheap | No | Document | None |
| engagementInput | engagement-input | Input | Keyboard | Cheap | No | Preview | None |
| canvasPointerDown | canvas-pointer-down | Pointer | Canvas | Cheap | No | Preview | None |
| setLocale | set-locale | Config | Menu | Cheap | No | Preview | None |
| noMutation | no-op | No-op | Menu | Cheap | No | None | None |
| copyPrompt | copy-prompt | Clipboard | Button | Cheap | No | None | Clipboard |
| exportVideoFromDeck | export-video-from-deck | Export | Menu | **Expensive** | No | None | Filesystem |

**Budget Risk:** `exportVideoFromDeck` (100ms–several seconds for video encoding, depending on frame count and resolution).

**Timeline Pattern:** Keyframe-based; scrubbing (`setFrame`) is cheap view state, not document mutation.

---

## Framework-Level Commands & Operations

### History (Undo/Redo/Checkpoints)

**Location:** `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`

**Commands (7 auto-injected into every app):**

| Command | Action ID | Type | Keybinding | Expense | Mutation | Notes |
|---------|-----------|------|-----------|---------|----------|-------|
| undo | framework.history.undo | History | Mod+Z | Cheap | Document | Reverts last operation |
| redo | framework.history.redo | History | Mod+Shift+Z | Cheap | Document | Replays last undone operation |
| commitCheckpoint | framework.history.checkpoint | History | None | Moderate | Document | Marks explicit milestone; can be expensive for large docs |
| createAlternative | framework.history.alternative-new | History | None | Moderate | None | Branches alternate history |
| switchAlternative | framework.history.alternative-switch | History | None | Cheap | None | View state only |
| checkoutCheckpoint | framework.history.checkpoint-checkout | History | None | Moderate | Document | Reverts to specific checkpoint |
| revertToCommand | framework.history.revert-to-command | History | None | Cheap-Moderate | Document | Time-travel via history panel picker |

**Precompute Lanes:** None explicit; undo/redo are atomic per operation.

**Budget Risk:** `commitCheckpoint` and `checkoutCheckpoint` may iterate entire document for serialization; on large models (1000+ objects) could exceed 8ms.

---

### Clipboard (Copy/Paste/Cut)

**Location:** `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`

**Commands (3 auto-injected into every app):**

| Command | Action ID | Type | Keybinding | Expense | Mutation |
|---------|-----------|------|-----------|---------|----------|
| copy | framework.clipboard.copy | Clipboard | Mod+C | Cheap | None |
| cut | framework.clipboard.cut | Clipboard | Mod+X | Cheap | Document |
| paste | framework.clipboard.paste | Clipboard | Mod+V | Cheap-Moderate | Document |

**Arguments:** `paste` carries optional `anchor` (original/middle/centroid/bottomLeft/bottomRight/topLeft/topRight) and `position` override.

**Budget Risk:** `paste` can be expensive for large fragments (100+ objects); deserialization + merge may exceed 8ms.

---

### Interaction (Selection & Hover)

**Location:** `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`

**Commands (6 — conditional if app declares interactions):**

| Command | Action ID | Type | Keybinding | Expense | Mutation | Notes |
|---------|-----------|------|-----------|---------|----------|-------|
| interactionSelect | framework.interaction.select | Interaction | Renderer | Cheap | Preview | Raw dispatch verb; never in palette |
| interactionHover | framework.interaction.hover | Interaction | Renderer | Cheap | Preview | Raw dispatch verb; never in palette |
| clearSelection | framework.selection.clear | Interaction | None | Cheap | Preview | User-facing; clears all domains |
| selectAll | framework.selection.all | Interaction | None | Cheap | Preview | User-facing; selects every target at active granularity |
| setSelectionMode | framework.selection.mode | Interaction | Menu | Cheap | Preview | Single vs. multiple mode |
| setInteractionGranularity | framework.interaction.granularity | Interaction | Menu | Cheap | Preview | Per-domain target scale |

**Precompute Lanes:** None; all cheap and preview-only.

---

### Global Utilities & Shell

**Location:** `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`

**Commands (5 framework-level utilities):**

| Command | Action ID | Type | Keybinding | Expense | Mutation | Notes |
|---------|-----------|------|-----------|---------|----------|-------|
| noteShellCommand | framework.shell.note | Shell | None | Cheap | None | Logs external effect (navigate, export, spawn) to history |
| setHistoryCommandFilter | framework.history.filter | View | None | Cheap | None | Tri-state filter (all/withoutOps/onlyOps) for history panel |
| startIntroduction | framework.ux.introduction | View | None | Cheap | None | Launches onboarding flow |
| startTutorial | framework.ux.tutorial | View | None | Cheap | None | Launches interactive tutorial |
| recordTutorial | framework.ux.record | View | None | Cheap | None | Records tutorial scenario for playback |

---

## Operations Exceeding 8ms Budget (Extended List)

### Ranked by Likelihood & Impact

1. **Layout::exportPng / exportSvg / exportPdf** (Raster/vector rendering)
   - **Likely:** 50–500ms (depends on page size, asset count, rendering backend)
   - **Current:** Synchronous
   - **Trigger:** Menu
   - **Classification:** Requires async + progress UI

2. **Process3d::importModelFile** (STEP/IGES parser + geometry import)
   - **Likely:** 20–200ms (depends on file size and triangle count)
   - **Current:** Synchronous
   - **Trigger:** File drop
   - **Classification:** Requires async + import wizard

3. **Process3d::exportModel** (Geometry export to standard format)
   - **Likely:** 50–500ms (depends on model complexity)
   - **Current:** Synchronous
   - **Trigger:** Menu
   - **Classification:** Requires async + progress reporting

4. **Animate::exportVideoFromDeck** (Video encoding)
   - **Likely:** 100ms–several seconds (depends on frame count, codec, resolution)
   - **Current:** Synchronous
   - **Trigger:** Menu
   - **Classification:** Requires async + cancellation + progress UI

5. **Sourcing::stockFromCatalogue** (Network catalog fetch)
   - **Likely:** 50–1000ms (depends on network latency and catalog API response time)
   - **Current:** Synchronous
   - **Trigger:** Menu
   - **Classification:** Requires async + network timeout handling

6. **History::commitCheckpoint / checkoutCheckpoint** (Document serialization)
   - **Likely:** 10–200ms on large documents (1000+ objects)
   - **Current:** Synchronous
   - **Trigger:** Menu / history panel
   - **Classification:** Requires async on large docs; monitor with budget instrumentation

7. **Framework::paste** (Fragment deserialization + merge for 100+ objects)
   - **Likely:** 20–100ms on large fragments
   - **Current:** Synchronous
   - **Trigger:** Keyboard (Mod+V)
   - **Classification:** Requires async + cancellation for large pastes

8. **Animate::setSource** (Image/video file load)
   - **Likely:** 10–100ms (depends on file size and format)
   - **Current:** Synchronous
   - **Trigger:** File drop
   - **Classification:** Requires async I/O handling

---

## Summary Statistics

| Metric | Count |
|--------|-------|
| **FEM 2D commands** | 19 |
| **FEM 3D commands** | 18 |
| **Layout commands** | 21 |
| **Block 3D commands** | 24 |
| **Process 3D commands** | 31 |
| **Sourcing commands** | 14 |
| **VCS commands** | 10 |
| **Animate commands** | 17 |
| **Energy commands** | 2 |
| **Subtotal: Plugin commands** | 156 |
| **History commands** | 7 |
| **Clipboard commands** | 3 |
| **Interaction commands** | 6 |
| **Global utilities** | 5 |
| **Subtotal: Framework commands** | 21 |
| **Part 1 inventory** | ~180 |
| **Part 2 inventory** | 177 |
| **Combined total** | ~357 |
| **Ops exceeding 8ms risk** | 8 (detailed above) |
| **I/O operations (file/network/clipboard)** | ~35 |

---

## Integration Notes

### Merge with Part 1
- Part 1 covered: Puzzle (2D/3D), Procedural (2D/3D), CAD, Draw, Space (S Studio), plus framework dispatch/action-bus
- Part 2 covers: FEM (2D/3D), Energy, Layout, Block, Process, Sourcing, VCS, Animate, plus framework history/clipboard/interaction
- **Combined coverage:** 13 plugins + framework level = comprehensive interactive command surface

### Missing/Incomplete Areas
1. **VCS advanced commands:** `branchCreate`, `branchDelete`, `mergeWithConflictResolution`, `diffShow`, `searchHistory` — not yet enumerated in app_commands! but listed in part 1 as suspected; may exist in command taxonomy folders (audit deferred to P1)
2. **FEM simulations:** No explicit `runFemSimulation` found in app_commands!; may be triggered via `setAnalysisSettings` + implicit solver or via separate I/O path
3. **Energy simulations:** No `runEnergySimulation` in editor commands; may be triggered externally or via dedicated simulation app
4. **Framework serialization:** `importMedia`, `exportMedia`, `importStudioPack`, `exportStudioPack` live in app-specific handlers (Space plugin), not framework-level

---

## Next Steps

1. **P0 completion:** Merge parts 1 & 2 into unified classification table; assign phase (P1–P4) to every command
2. **P1 follow-up:** Audit VCS/FEM command taxonomy folders for advanced ops (`mergeWithConflictResolution`, `runFemSimulation`)
3. **P2 implementation:** Wrap all 8ms+ operations with tick-based async + cancellation tokens
4. **P3 validation:** Instrument existing precompute lanes (procedural flow, puzzle3d fill) with 8ms budget telemetry
5. **P4 registry:** Build release-time command classification validator (reject unclassified commands in builds)

---

**Generated:** 2026-08-20  
**By:** Semio P0 Inventory Part 2 Audit  
**Scope:** Read-only inventory; no modifications made  
**Completes:** 8 unaudited plugin areas + framework-level operations
