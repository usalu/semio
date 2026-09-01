# Canonical Plugin Shape Analysis for 💠️lowpoly

## Governance

No `TEMPLATE.md` found. The canonical shape is defined by **emoji-based directory taxonomy** that must be rigorously followed across all plugins. The structure enforces event-driven, schema-first design with multi-implementation support.

### Key Paths
- Governing rules: `/Users/ueli/Documents/semio/AGENTS.md` (lines 23-47 define multi-implementation, schema-first, domain-driven taxonomy requirements)
- Reference plugins: 
  - **📐️cad** — most complete implementation (all facets populated)
  - **🧱️block** — three artifacts (◻2d, 🖐️5d, 🧊️3d), well-structured
  - **🪐️space**, **🧩️puzzle** — partial implementations
  - **🌀️procedural** — three variants (🌀️procedural2d, 🧊️procedural3d, 🧩️assembly)

---

## Structural Hierarchy

Every plugin follows:
```
✏️s/🔌️plugins/{PLUGIN}/🗿️artifacts/{ARTIFACT}/🏅️standards/🔖️1/🪆️subsets/✳️any/
├── ✏️editor/                              # Editing interface
│   ├── ⚙️engine/                          # Core implementation (Rust component)
│   ├── 🎚️config/                          # Artifact-level config schema
│   │   └── 🧬️schema/                      # MultiFormat schema (graphql,json,ts,rs,proto)
│   ├── 🎭️modes/{MODE}/                    # Mode-specific implementations (e.g., ✏️edit, 🎨️paint)
│   │   ├── 🎚️config/                      # Mode config (EMPTY in lowpoly)
│   │   ├── 🎮️commands/                    # Mode-level commands (EMPTY in lowpoly edit mode)
│   │   ├── 👥️presence/                    # Presence schema (EMPTY in lowpoly edit mode)
│   │   ├── 🫧️transient/                   # Session ephemeral state (EMPTY in lowpoly edit mode)
│   │   └── 🪟️windows/{WINDOW}/            # Window-specific state
│   │       ├── 🎚️config/                  # Window config (EMPTY)
│   │       ├── 🎚️options/                 # Window-level options (EMPTY)
│   │       ├── 🎬️actions/                 # Window-specific action handlers (EMPTY)
│   │       ├── 👥️presence/                # Window presence (EMPTY)
│   │       ├── 🪛️utilities/               # Helper functions (EMPTY)
│   │       └── 🫧️transient/               # Window ephemeral state (EMPTY)
│   ├── 🎮️commands/{COMMAND}/              # Artifact-level commands
│   │   └── 🦀️component.rs                 # Command handler
│   ├── 👥️presence/                        # Shared editor presence
│   │   ├── 🧬️schema/                      # Collaboration state schema
│   │   └── 🔗️component.graphql, 🔣️component.json, etc.
│   └── 🫧️transient/                       # Editor session state (EMPTY)
├── 👁️viewer/                              # Read-only viewing interface  
│   ├── 🎚️config/                          # Viewer config (EMPTY in lowpoly)
│   ├── 🎭️modes/👁️view/                   # Viewer-specific mode
│   │   ├── 🎮️commands/                    # Viewer commands (EMPTY)
│   │   ├── 👥️presence/                    # Viewer presence (EMPTY)
│   │   ├── 🫧️transient/                   # Viewer session (EMPTY)
│   │   └── 🪟️windows/{WINDOW}/            # Viewer windows (EMPTY)
│   ├── 🎮️commands/                        # Viewer-level commands (EMPTY)
│   ├── 👥️presence/                        # Viewer presence (EMPTY)
│   └── 🫧️transient/                       # Viewer ephemeral (EMPTY)
├── 📚️examples/                            # Demo sessions and test fixtures
├── 🚪️io/                                  # Import/export serializers
└── 🧪️oracle/                              # Test suite

```

---

## 💠️lowpoly Structural Gaps

### Summary
**34 empty/missing directories** across editor and viewer facets. The plugin has basic infrastructure but lacks complete command system integration, presence/transient state management, and viewer support.

### Critical Gaps (Ranked by Impact)

#### 1. **Mode-Level Commands** (Editor Modes)
**Missing:** `✏️editor/🎭️modes/✏️edit/🎮️commands/` and paint mode equivalent
**Impact:** Commands at mode level are shadowed by artifact-level commands but need explicit mode bindings.
**Current State:** Empty (📌️empty.md)
**Should Contain:** Mode-specific command variant handlers or re-exports
**Example File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️sun/🦀️component.rs` (lines 1-30)

#### 2. **Window-Level Config/Actions** (Edit & Paint Modes)
**Missing:** 
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🎚️config/`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🎬️actions/`
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🎚️options/`
- Similar for paint mode `🖼️uv` window

**Impact:** Window state and actions not declaratively defined; impossible for multi-user sync or state persistence.
**Current State:** Empty (📌️empty.md)
**Should Contain:** Component files for window-specific config schema and action handlers
**Example:** See 📐️cad window structures at `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/`

#### 3. **Editor & Mode Presence (Collaboration)**
**Missing:**
- `✏️editor/🎭️modes/✏️edit/👥️presence/` (mode-level presence)
- `✏️editor/🎭️modes/🎨️paint/👥️presence/`

**Impact:** No per-mode collaboration state; cannot track mode-specific selections/cursors/presence.
**Current State:** Empty (📌️empty.md)
**Should Contain:** Schema files (json/graphql/ts/rs/proto) in 🧬️schema subdirectory
**Example File:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🔣️component.json` (lines 1-40)
```json
{
  "$id": "https://semio.tech/schema/app/cad/cad/presence.json",
  "title": "CadPresence",
  "type": "object",
  "required": ["selectedObjectIds", "selectedNodeIds", "cameraPosition", "cameraTarget"],
  "properties": {
    "selectedObjectIds": {"type": "array", "items": {"type": "string"}, "x-semio-state": "presence"},
    "selectedNodeIds": {"type": "array", "items": {"type": "string"}, "x-semio-state": "presence"},
    "cameraPosition": {"type": "array", "items": {"type": "number"}},
    "cameraTarget": {"type": "array", "items": {"type": "number"}}
  }
}
```

#### 4. **Editor & Mode Transient State**
**Missing:**
- `✏️editor/🎭️modes/✏️edit/🫧️transient/` (mode session state)
- `✏️editor/🎭️modes/🎨️paint/🫧️transient/`
- `✏️editor/🫧️transient/` (editor-level ephemeral)

**Impact:** Temporary UI state (undo stack, drag state, tooltips) not explicitly modeled.
**Current State:** Empty (📌️empty.md)
**Should Contain:** Component.rs with transient state mutations
**Pattern:** Similar to presence but marked `x-semio-state: transient`

#### 5. **Viewer Interface (👁️viewer)**
**Missing:** Nearly all viewer facets:
- `👁️viewer/🎚️config/` — Viewer artifact config
- `👁️viewer/🎭️modes/👁️view/🎚️config/` — View mode config
- `👁️viewer/🎭️modes/👁️view/🎮️commands/` — View commands
- `👁️viewer/🎭️modes/👁️view/👥️presence/` — View presence
- `👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/{🎚️config,🎬️actions,🪛️utilities,🫧️transient}/`
- `👁️viewer/🎮️commands/` — Viewer-level commands
- `👁️viewer/👥️presence/` — Viewer presence
- `👁️viewer/🫧️transient/` — Viewer ephemeral

**Impact:** No read-only viewing mode; collaboration on shared views impossible.
**Current State:** Empty (📌️empty.md)
**Should Contain:** Full mirror of editor structure with read-only semantics
**Example Template:** 📐️cad viewer at `/Users/ueli/Documents/semio/✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/`

#### 6. **Window Utilities & Options** (All Windows)
**Missing:** 
- `🪛️utilities/` in all window paths (14 directories)
- `🎚️options/` in all window paths (6 directories)

**Impact:** Helper functions, dropdown options not centralized or reusable.
**Current State:** Empty (📌️empty.md)
**Should Contain:** 
  - **utilities**: Component.rs with pure functions (transform calculations, filters)
  - **options**: Config enum values or dropdown data
**Example Pattern:** 📐️cad has same gaps — likely not populated yet in ANY plugin at window level

#### 7. **Artifact-Level Config** (Editor)
**Missing:** `👁️viewer/🎚️config/` only
**Current State:** ✏️editor has it populated; viewer does not
**Impact:** Viewer cannot configure rendering/display preferences independently

---

## Populated Areas in 💠️lowpoly (Reference)

✓ **Artifact-level commands** (14 command variants): patch-object, add-primitive, sun, camera, chrome, engagement, fixture, mesh-edit, paint, selection, utility, transform, uv
✓ **Artifact-level presence schema** (6 multiformat files) 
✓ **Editor engine** (Rust component)
✓ **Editor config schema** (5 multiformat)
✓ **Two editor modes** (edit, paint) with window definitions
✓ **I/O serializers/deserializers** (9 artifact formats: las, ply, txt, png, json, dwg, stl, gltf, obj)
✓ **Oracle test suite**
✓ **Panels** (artifact, inspection, layers, catalogue)
✓ **Session tracking**
✓ **Terminology** (localization)
✓ **View options** (sun, show-edges, paint params, select, snap)

---

## Comparison with 📐️cad (Canonical Reference)

### 📐️cad Status
- **Empty count:** ~40 directories (mostly window-level actions/transient/utilities, same as lowpoly)
- **Difference:** 📐️cad has viewer structure populated (viewer config, modes, presence schema, window defs)
- **Node:** 📐️cad defines window names explicitly: `🏛️structure-classic`, `🏢️building`, `📐️shape`, `🔥️energy` (modes reference these)
- **Lowpoly Window Names:** `🌐️model` (edit mode), `🖼️uv` (paint mode) — correctly structured

### 🧱️block Status  
- **Multiple artifact variants:** ◻2d (2D blocks), 🖐️5d (5D handles), 🧊️3d (3D objects)
- **Same gaps:** Window-level actions, utilities, transient; viewer mostly empty
- **Better presence:** Defines presence at mode level in all artifacts

---

## Implementation Checklist

### Phase 1: Presence & Transient (CRITICAL for CQRS)
Needed for event-sourcing, undo/redo, and multi-user:

1. **`✏️editor/🎭️modes/✏️edit/👥️presence/🧬️schema/`**
   - Define EditModePresence (selection, tool state, cursor)
   - Files: component.json, component.ts, component.rs, component.graphql, component.proto

2. **`✏️editor/🎭️modes/🎨️paint/👥️presence/🧬️schema/`**
   - Define PaintModePresence (brush params, uv state, selections)

3. **`✏️editor/🎭️modes/{MODE}/🫧️transient/`** (both modes)
   - Define ephemeral state (drag-in-progress, hover targets, tooltips)

4. **`✏️editor/🫧️transient/🧬️schema/`**
   - Define editor-level transient (viewport zoom, scroll position, dialogs open)

### Phase 2: Mode-Level Commands
1. **`✏️editor/🎭️modes/✏️edit/🎮️commands/`**
   - Bind edit-mode-specific command handlers (should mostly re-export artifact commands)

2. **`✏️editor/🎭️modes/🎨️paint/🎮️commands/`**
   - Paint-mode-specific bindings

### Phase 3: Window Configuration
1. **`✏️editor/🎭️modes/{MODE}/🪟️windows/{WINDOW}/🎚️config/🧬️schema/`** (4 paths total)
   - Define window state schema per window type

2. **`✏️editor/🎭️modes/{MODE}/🪟️windows/{WINDOW}/🎬️actions/`** (4 paths)
   - Implement window-level action handlers

3. **`✏️editor/🎭️modes/{MODE}/🪟️windows/{WINDOW}/🪛️utilities/🦀️component.rs`** (4 paths)
   - Helper functions for window operations

### Phase 4: Mode-Level Config
1. **`✏️editor/🎭️modes/✏️edit/🎚️config/🧬️schema/`**
   - Edit mode config (tool settings, etc.)

2. **`✏️editor/🎭️modes/🎨️paint/🎚️config/🧬️schema/`**
   - Paint mode config

### Phase 5: Viewer Mirror (Optional but Recommended)
1. Replicate editor structure under `👁️viewer/` with read-only semantics
2. Populate viewer config, modes, presence, commands, transient
3. Define viewer-specific windows (likely same windows as editor in view-only mode)

---

## Key Files to Reference

| Category | Example Path | Lines | Purpose |
|----------|--------------|-------|---------|
| Artifact Presence Schema | `📐️cad/👥️presence/🧬️schema/component.json` | 1-50 | Multi-user selection tracking |
| Command Implementation | `📐️cad/🎮️commands/🌞️sun/component.rs` | 1-30 | Handler pattern for sun commands |
| Artifact Config | `💠️lowpoly/🎚️config/🧬️schema/component.json` | — | Config structure (already populated) |
| Editor Engine | `💠️lowpoly/⚙️engine/component.rs` | — | Core Rust implementation |

---

## Critical Constraints (from AGENTS.md)

- **Schema-first:** Always define 🧬️schema/ multiformat before implementation
- **CQRS + Event Sourcing:** All state changes flow through Emit<Artifact Mutation, Config Mutation>
- **No CRUDs:** Events only (apply, amend, retire)
- **Multi-language:** Schemas support graphql, json, typescript, rust, protobuf simultaneously
- **Local-first:** Ephemeral state (🫧️transient) separate from persisted presence (👥️presence)
- **Multi-user:** Presence schema enables concurrent editing without CRDTs

---

## Artifact Root for lowpoly

`/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/`

All relative paths in the diff are from this root.

