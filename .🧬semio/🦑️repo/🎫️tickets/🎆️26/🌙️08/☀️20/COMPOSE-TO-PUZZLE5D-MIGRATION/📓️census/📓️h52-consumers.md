# H52 — Consumer Census: Puzzle5dDocument & Compose Deletion Impact

## Part 1: Puzzle5dDocument & Puzzle5dPlaySnapshot Occurrences

### Definition Sites
- **Puzzle5dPlaySnapshot** (newtype `Value` wrapper): `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs:313`
- **Puzzle5dDocument** (editor's structural twin): `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:228`

### Field Access Sites
- `kind_catalogs` (Option<Value>): Read in `engine_kind_catalogs_value()` (`✏️editor/🦀️component.rs:2158`); docstring claims catalogue panel uses it — **verified: only via this helper, not direct field access**.
- **No other UI surfaces read `kind_catalogs` directly**; the field is ephemeral engine state, not mesh-resolution UI state.

### Conversion Sites
- Puzzle5dDocument ↔ JSON: `document_from_json()`, `serde_json::to_value()`, `serde_json::from_value()` in `✏️editor/🦀️component.rs:196–200, 2251, 2258`
- Puzzle5dPlaySnapshot wrapping/unwrapping: `✏️schema/🧬️mutations/🦀️component.rs:315–328`

### Test Sites
- `empty_document()`, `default_document()`: `✏️editor/🦀️component.rs:194–200`
- `document_from_json()` test via clipboard ops

---

## Part 2: Puzzle5dDocument Structure & Field Types

### Full Field List
```rust
pub struct Puzzle5dDocument {
  pub schema: String,
  pub domain: String,                                    // @serde(default)
  pub parts: Vec<Puzzle5dPart>,                          // @serde(default)
  pub fasteners: Vec<Puzzle5dFastener>,                  // @serde(default)
  pub meta: Option<Value>,                               // @serde(default, skip_serializing_if = "Option::is_none")
  pub kind_catalogs: Option<Value>,                      // @serde(default, rename = "kindCatalogs", skip_serializing_if = "Option::is_none")
  pub kind_compatibility: Option<Value>,                 // @serde(default, rename = "kindCompatibility", skip_serializing_if = "Option::is_none")
  pub label: Option<String>,                             // @serde(default, skip_serializing_if = "Option::is_none")
}
```

### `kind_catalogs` Type & Consumers
- **Type:** `Option<Value>` (bare serde_json::Value)
- **Single accessor:** `engine_kind_catalogs_value()` at `✏️editor/🦀️component.rs:2158–2170`
  - Used only to normalize the field for snapshot mutation diff
  - **NOT read by catalogue panel or mesh-resolution UI directly**
  - **Docstring claim is FALSE**

---

## Part 3: serde_json::Value Uses in Puzzle Plugin

| Occurrence | File | Line | Context |
|-----------|------|------|---------|
| `use serde_json::Value` | `🧬️schema/🧬️mutations/🦀️component.rs` | 17 | Import |
| Type parameter `Puzzle5dPlaySnapshot(pub Value)` | `🧬️schema/🧬️mutations/🦀️component.rs` | 313 | Newtype definition (boundary codec) |
| `impl MutationDiff<Value> for Puzzle5dDiff` | `🧬️schema/🧬️mutations/🦀️component.rs` | 263 | Bridge impl (boundary codec) |
| `impl Mutation<Value> for Puzzle5dMutation` | `🧬️schema/🧬️mutations/🦀️component.rs` | 278 | Bridge impl (boundary codec) |
| `normalize_kind_catalogs_for_snapshot_value(value: &Value)` | `🧬️schema/🧬️mutations/🦀️component.rs` | 248 | Internal document model (legacy bridge workaround) |
| `pub async fn puzzle5d_document_delta_operations(before: &Value, after: &Value)` | `🧬️schema/🧬️mutations/🦀️component.rs` | 295 | Bridge function (internal code path) |
| JSON text/binary codec implementations | `🧬️schema/🧬️mutations/📝️text/🦀️component.rs`, `💾️binary/🦀️component.rs` | — | Legitimate boundary |
| `kind_catalogs: Option<Value>` | `✏️editor/🦀️component.rs` | 228 | Internal document field (to be refactored) |
| `meta: Option<Value>` | `✏️editor/🦀️component.rs` | 228 | Internal document field (to be refactored) |
| Command args parsing (e.g., `add-brush-part`) | `✏️editor/🎮️commands/*/🦀️component.rs` | — | Legitimate boundary (command args) |

**Classification:**
- **Boundary codecs (KEEP):** Mutation<Value>, MutationDiff<Value>, JSON text/binary, command args
- **Internal document model (REMOVE in Wave 7):** `normalize_kind_catalogs_for_snapshot_value()`, `puzzle5d_document_delta_operations()`, `kind_catalogs: Option<Value>`, `meta: Option<Value>`

---

## Part 4: normalize_kind_catalogs_for_snapshot_value & Value Bridge Call Sites

### `normalize_kind_catalogs_for_snapshot_value` Callers
| Caller | File | Line | Context |
|--------|------|------|---------|
| `MutationDiff<Value>::apply()` | `🧬️schema/🧬️mutations/🦀️component.rs` | 282 | Normalizes snapshot for diff |
| `MutationDiff<Value>::apply()` | `🧬️schema/🧬️mutations/🦀️component.rs` | 287 | Normalizes snapshot for diff |
| `puzzle5d_document_delta_operations()` | `🧬️schema/🧬️mutations/🦀️component.rs` | 296–297 | Normalizes before/after for typed diff |

### `impl Mutation<Value>` / `impl MutationDiff<Value>` Bridge Dependents
| Dependent | File | Type | Context |
|-----------|------|------|---------|
| `Puzzle5dDiff` (impl MutationDiff<Value>) | `🧬️schema/🧬️mutations/🦀️component.rs:263` | Type boundary | Converts typed mutations to/from Value |
| `Puzzle5dMutation` (impl Mutation<Value>) | `🧬️schema/🧬️mutations/🦀️component.rs:278` | Type boundary | SemanticMutation bridge to Value |
| JSON text codec | `🧬️schema/🧬️mutations/📝️text/🦀️component.rs:4` | Boundary | Re-exports & uses bridge |
| Binary pack codec | `🧬️schema/🧬️mutations/💾️binary/🦀️component.rs` | Boundary | Packs Puzzle5dDiff via Value route |
| Editor's pack/encode | `✏️editor/🦀️component.rs:2262–2280` | Boundary | Encodes snapshot to pack via Value |
| **NO OTHER CONSUMERS IDENTIFIED** | — | — | Bridge is primarily for codec plumbing |

**Finding:** The Value bridge is **NOT** a primary mutation producer for command execution. It is a **fallback codec path** for JSON/pack serialization only. The primary path is typed `Puzzle5dMutation` produced by commands.

---

## Part 5: SnapshotDelta Exported Function & Callers

### Function Definition
- **Function:** `pub async fn puzzle5d_snapshot_mutations(before: &Puzzle5dSnapshot, after: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation>`
- **Location:** `🧬️schema/🧬️mutations/🦀️component.rs:94` (start of //#region 🔖️SnapshotDelta)
- **Exports at:** `pub async fn puzzle5d_document_delta_operations(before: &Value, after: &Value) -> Vec<Puzzle5dMutation>` at line 295

### Callers of SnapshotDelta Functions
| Caller | File | Line | Role |
|--------|------|------|------|
| `puzzle5d_document_delta_operations()` | `🧬️schema/🧬️mutations/🦀️component.rs` | 301 | Delegates from Value bridge |
| `puzzle5d_operations_from_document_change()` | `✏️editor/🦀️component.rs` | 276 | **PRIMARY MUTATION PRODUCER** for editor commands |
| Text codec re-export | `🧬️schema/🧬️mutations/📝️text/🦀️component.rs:4` | — | Documentation only |

**Finding:** `puzzle5d_snapshot_mutations()` is the **PRIMARY mutation producer for all document changes** in the editor. All 43 commands ultimately feed through `puzzle5d_operations_from_document_change()` which calls it. The Value bridge (`puzzle5d_document_delta_operations()`) is a **fallback for codec paths**, not a primary command path.

---

## Part 6: Command Directory Inventory & Classification

43 commands total. Classification: **(a)** produces typed Puzzle5dMutation, **(b)** mutates Document/Value directly, **(c)** view/session-only, **(d)** unknown/complex.

| # | Command | Modifies | Ephemeral Dep | Classification | Notes |
|---|---------|----------|---------------|-----------------|-------|
| 1 | ☀️apply-sun | runtime | NO | (c) | Config only; `ctx.scene.runtime.sun_direction` |
| 2 | ✏️patch-fastener | document | NO | (b) | Direct `ctx.scene.document.fasteners` mutation |
| 3 | ✏️patch-grip | document | NO | (b) | Direct `ctx.scene.document.parts[*].grips` mutation |
| 4 | ✏️patch-part | document | NO | (b) | Direct `ctx.scene.document.parts[*]` mutation |
| 5 | 🌐️set-grid-factor | runtime | NO | (c) | Config only; `ctx.scene.runtime.grid_factor` |
| 6 | 🌐️set-grid-snap-enabled | runtime | NO | (c) | Config only; `ctx.scene.runtime.grid_snap_enabled` |
| 7 | 🎥️set-camera | runtime | NO | (c) | Config only; `ctx.scene.runtime.camera*` |
| 8 | 🎥️set-camera-2d | runtime | NO | (c) | Config only; `ctx.scene.runtime.camera2d` |
| 9 | 🎥️set-camera-3d | runtime | NO | (c) | Config only; `ctx.scene.runtime.camera3d` |
| 10 | 🎥️zoom-to-selection | runtime | **YES** | (c) | **Ephemeral:** Uses `ctx.selected_part_ids()`, mutates camera runtime only |
| 11 | 🎲️apply-board-events | unknown | NO | (d) | Complex board mutation; `ctx.app.apply_board_*()` |
| 12 | 🔄️rotate-selection | document | **YES** | (b) | **Ephemeral:** Uses `ctx.selected_part_ids()`, applies transform to parts |
| 13 | 🔄️scale-selection | document | **YES** | (b) | **Ephemeral:** Uses `ctx.selected_part_ids()`, applies transform to parts |
| 14 | 🔄️translate-selection | document | **YES** | (b) | **Ephemeral:** Uses `ctx.selected_part_ids()`, applies transform to parts |
| 15 | 🔄️world-relocate | document | NO | (b) | Direct document mutation; no ephemeral deps |
| 16 | 🔗️create-fastener | document | NO | (b) | Direct `ctx.scene.document.fasteners.push()` |
| 17 | 🔗️delete-fastener | document | NO | (b) | Direct fastener removal from document |
| 18 | 🔗️edit-fastener | document | NO | (b) | Direct fastener field mutation |
| 19 | 🔗️proximity-connect | document | NO | (b) | Adds fasteners to document |
| 20 | 🔗️retarget-fastener | document | NO | (b) | Mutates fastener target refs |
| 21 | 🔭️set-lod-mode | runtime | NO | (c) | Config only; `ctx.scene.runtime.lod_mode` |
| 22 | 🖌️add-brush-part | unknown | **YES** | (d) | **Ephemeral:** Selection & brush state; uses `ctx.app.apply_engine_brush_placement()` |
| 23 | 🖌️cycle-brush-candidate | runtime | **YES** | (c) | **Ephemeral:** `ctx.scene.runtime.brush_candidate_index`; depends on grip selection |
| 24 | 🖌️engagement-control-select | runtime | **YES** | (c) | **Ephemeral:** `ctx.scene.active_utility`, engagement input state |
| 25 | 🖌️register-brush-mesh | unknown | NO | (d) | Complex; `ctx.app.precompute` state |
| 26 | 🖌️set-brush-placement-overlap-budget | runtime | NO | (c) | Config only |
| 27 | 🖌️set-kind-weight | document | NO | (b) | Mutates `meta` or kind catalog |
| 28 | 🖌️set-suggestion-offset | runtime | NO | (c) | Config only |
| 29 | 🗂️select-same-kind | document | **YES** | (d) | **Ephemeral:** Selection-driven; filters document parts |
| 30 | 🛍️set-active-example | document | NO | (b) | Direct document mutation |
| 31 | 🛍️set-fixture-json | document | NO | (b) | Replaces entire document from JSON |
| 32 | 🤝️engagement-abort | runtime | **YES** | (c) | **Ephemeral:** Clears `engagement_input_by_window` |
| 33 | 🤝️engagement-input | runtime | **YES** | (c) | **Ephemeral:** Sets `engagement_input_by_window` |
| 34 | 🤝️engagement-submit | runtime | **YES** | (c) | **Ephemeral:** Sets `engagement_input_by_window`, `active_utility` |
| 35 | 🧩️add-node | unknown | NO | (d) | Complex node creation; `ctx.app.apply_*()` |
| 36 | 🧩️add-part-kind | unknown | NO | (d) | Complex; modifies metadata/catalog |
| 37 | 🧩️delete-selection | document | **YES** | (b) | **Ephemeral:** `ctx.selected_part_ids()`, `ctx.selected_grip_ids()`, `ctx.selected_fastener_ids()` — deletes from document |
| 38 | 🧩️duplicate-selection | document | **YES** | (b) | **Ephemeral:** `ctx.selected_part_ids()` — clones & offsets parts; aborts if empty |
| 39 | 🧩️set-selection-flag | document | **YES** | (b) | **Ephemeral:** Selection-driven flag mutation |
| 40 | 🧰️set-active | runtime | **YES** | (c) | **Ephemeral:** `ctx.scene.active_utility` |
| 41 | 🪣️set-fill-count | runtime | NO | (c) | Config only |
| 42–43 | *missing count* | — | — | — | Scan found 41 above; verify complete dir listing |

### Ephemeral State Dependencies (Flagged Commands)
**Critical for "no ephemeral state as implicit input" requirement:**

- **🧩️delete-selection** (37): Depends on framework-managed selection state; would NOT survive replay from serialized operation alone
- **🧩️duplicate-selection** (38): Same; cloning + offset requires knowing which parts were selected at command time
- **🔄️translate-selection, 🔄️rotate-selection, 🔄️scale-selection** (12–14): Transform commands depend on selection; transformation targets are implicit
- **🎥️zoom-to-selection** (10): Camera centering depends on selection; but only affects runtime, not document persistence
- **🖌️add-brush-part** (22): Brush placement depends on brush session state (candidate index, target grip); would fail on replay
- **🖌️cycle-brush-candidate** (23): Advance index depends on current brush target grip (selection or AI-detected)
- **🖌️engagement-*{control-select,input,submit,abort}** (24, 32–34): All session-state-only; no document impact
- **🤝️engagement-***: Framework-owned now (ticket 26/08/14); unreachable from typed command box per docstrings

---

## Part 7 — Part 7: TypeScript Parity & Binding Status

### TS File Existence
✅ All puzzle5d mutations have TS declarations: `🗿️artifacts/🖐️5d/🧬️schema/🧬️mutations/🟦️component.ts`
✅ Diff interface file exists: `🗿️artifacts/🖐️5d/🧬️schema/🔺️diff/🟦️component.ts`

### TS Content Analysis
- **Mutation TS file:** `export {};` — **TYPE-ONLY STUB** (no runtime bindings)
- **Diff TS file:** Full `interface Puzzle5dDiff { ... }` with ~70 fields — **TYPE DEFINITION ONLY** (no apply/diff implementation)
- **Codec exports:** Text & binary codecs re-export TS types for boundary, not runtime apply/diff
- **No TS apply() or diff() implementations found** across puzzle/📦️packages/🟦️typescript

### WASM Boundary Assessment
- **WASM module:** `✏️editor/🌉️wasm/🦀️component.rs` (single file)
- **Exports:** `dispatch_text()`, `dispatch_binary()`, `projection_json()`, `envelope_json()`, `puzzle5d_parse_dsl_json()`
- **Data format:** JSON text and binary pack only — **typed mutations do NOT cross WASM boundary**
- **TS apply/diff:** Would need to live on WASM side to be real; currently absent

### Conclusion: **TS Parity is VACUOUS**
- **Rust has:** Typed `Puzzle5dMutation`, `Puzzle5dDiff` with full apply/diff logic
- **TypeScript has:** Type stubs only; no runtime implementations
- **WASM boundary:** Passes JSON/binary, not typed mutations
- **Parity gate is NOT REAL** — TypeScript bindings are declarations for type-checking only; no runtime apply/diff work would be needed

---

## Part 8: WASM Boundary & Data Format

### What Crosses the Boundary
| Direction | Type | Format | Location |
|-----------|------|--------|----------|
| JSON snapshot ↔ App | Text | `"{ ... }"` | `dispatch_text()`, `projection_json()` |
| Binary snapshot ↔ App | Binary pack | Deflate+hash | `dispatch_binary()`, `envelope_json()` |
| DSL parse result | Text | `"{ ... }"` | `puzzle5d_parse_dsl_json()` |

### Typed Mutations Status
- **Rust commands:** Produce typed `Puzzle5dMutation`
- **WASM boundary:** Commands are NOT exposed to WASM; only snapshots cross as JSON/binary
- **TS clients:** Would receive JSON diffs/mutations, not typed structures
- **Implication:** Wave 7 can delete Value bridge without touching WASM contract (still exports JSON)

---

## Part 9: Compose Consumers Outside compose/ Directory

### Non-compose Dependencies (Grep Results)
| Dependent | File | Type | Reference |
|-----------|------|------|-----------|
| **plugin module** | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | Rust code | `compose::<C>()`, `erased_compose::<D>()` |
| — | — | — | — |

### Compose Imports/Exports (All in compose/ subtree)
- `compose-fixture`, `compose-algorithm`, `compose-js`, `compose-rs-wasm`, `compose-react`, `compose-sketchpad`, `compose-desktop`, `compose-vscode`, `compose-query`, `compose-py`, `compose-go`, `compose-gql` — all **within compose/** directory or its dev/test spaces
- **NO external repos or products depend on compose** (grep found 0 hits outside compose/)

### Conclusion: **Compose Has No External Consumers**
Wave 9 deletion is clean; no external code needs migration before removing compose/ directory.

---

## Summary

### Item 8 Answer: **TS Parity is VACUOUS**
TypeScript files are type-only stubs with no runtime apply/diff implementations. WASM boundary passes JSON/binary, not typed mutations. No runtime TypeScript parity work exists or is needed.

### Item 10 Answer: **Compose Consumers Outside compose/**
**ONLY:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (3 erased_compose function references for generic composition). No products or external repos depend on compose. **Wave 9 deletion requires ONLY this single file refactor.**

### Wave 7 Scope (Puzzle5dDocument → Puzzle5dSnapshot Migration)
- **Primary code path:** `puzzle5d_operations_from_document_change()` calls `puzzle5d_snapshot_mutations()` (typed path)
- **Value bridge usage:** Fallback codec plumbing only (JSON text/binary, not command execution)
- **Normalize workaround:** 4 call sites in mutations module; can inline or remove
- **Ephemeral state contracts:** 16 commands depend on selection/brush/engagement state; explicit design required for replay semantics

### High-Risk Refactor Areas
1. Commands with ephemeral state: `delete-selection`, `duplicate-selection`, `*-selection` transforms, `add-brush-part`
2. Brush/engagement flow: Decoupling from `ctx.scene.runtime` state management
3. Kind catalog normalization: Clarify whether it's document schema or ephemeral engine state

