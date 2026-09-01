# Version Control Story: Demonstrator Apps End-to-End Analysis

**Date:** 2026-08-28  
**Scope:** Event-sourcing core, UI surfaces, persistence, undo/redo implementation, gaps blocking full version control across six demonstrator apps.

---

## 1. Event-Sourcing Core

### Location & Definitions
- **Core module:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs:1-200`
  - Generic version-graph algebra for Author/Change/Checkpoint/Alternative/ArtifactVcs
  - Pure data structures + pure functions (no live document mutation)
  - Document mutation handled by `store::ArtifactStore`, which depends on `vcs`

### Data Structures (Rust)
- **`Author`** (🧀️component.rs:76-81): User identity with id, name, optional avatar
- **`Change`** (🧀️component.rs:88-94): Ordered collection of edits with id, description, timestamp
- **`Checkpoint`** (🧀️component.rs:118-132): Version node with change_ids, parent_id, authors, message, composition_pins for composite artifacts
- **`Alternative`** (🧀️component.rs:136-140): Branch name + checkpoint sequence (enables parallel work)
- **`CompositionPin`** (🧀️component.rs:111-114): Captures child artifact checkpoint IDs when parent is checkpointed

### Content-Addressed IDs
All IDs deterministic (blake3-based, no global counter):
- **Edit ID** (🧀️component.rs:33-42): `mint_edit_id(actor, sequence, forwards_fingerprint)` → `edit-{hex16}`
- **Change ID** (🧀️component.rs:44-50): `mint_change_id(edit_ids[], description?)` → `change-{hex16}`
- **Alternative ID** (🧀️component.rs:52-58): `mint_alternative_id(name, checkpoint_ids[])` → `alternative-{hex16}`
- **Mutation ID** (🧀️component.rs:60-63): `mint_mutation_id(mutation_bytes)` → `mutation-{hex16}`
- **Checkpoint ID** (🧀️component.rs:16-24): Content-addressed via blake3(prefix || 0 || payload)

### History Ledger
- **Capacity:** `ARTIFACT_HISTORY_LEDGER_CAPACITY = 64` (🧀️component.rs:142)
- **Visibility coordination:** `ArtifactGroupVisibility` + `ArtifactGroupVisibilityOwner` (🧀️component.rs:145-202)
  - Allows multiple readers to see the same committed history state atomically
  - Used by `ArtifactHistoryLedger` for multi-member group synchronization
- **Tests:** Fixture-backed tests in `🧪️group-history.json` (🧀️component.rs:221-282)

### Edit & Mutation Format
- **Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8-36`
- **Envelope structure:** MutationEnvelope carries diff, inverse, baseVersion, undoPolicy
- **Inverse:** Precomputed undo operation stored with every forward mutation
- **UndoPolicy:** Strict ("exactBaseOnly") — undo only applies if base version matches exactly

---

## 2. UI Surfaces for Version Control

### History Panel (Framework-Auto-Injected)

**Registration & IDs:**
- **Panel Tab ID:** `framework.panel.history` (🔨️manifest/🟦️component.ts:)
- **Auto-injected:** Every app's manifest receives this read-only history panel
- **Icon:** `undo` icon (undo-2.svg as alternative)

**UI Implementation:**
- **Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:5514-5540`
  - Line 5514: Panel tab lookup by kind id `FRAMEWORK_PANEL_TAB_HISTORY_ID`
  - Line 5523-5525: Tab registration with label resolution
  - Line 5533-5534: Undo/Redo buttons with disabled states (`!historyProjection.canUndo`, `!historyProjection.canRedo`)

**History Projection State:**
- **Location:** ShellHost.tsx:1116
- **Fields:** 
  ```typescript
  {
    cursor: number,                                           // Current position in history
    entries: Record<number, HistoryEntry>,                   // Seq → entry map
    canUndo: boolean,                                         // Undo button state
    canRedo: boolean,                                         // Redo button state
    currentCheckpointId: string | undefined                  // Last committed checkpoint
  }
  ```
- **Updates:** Via `applyHistoryPatch()` callback (ShellHost.tsx:1123-1128)
  - Patches are upserts of new/modified entries
  - Cursor moves forward only (prevents stale follow paints)

**History Entry Display:**
- **Location:** ShellHost.tsx:5521-5588
- **Rendering:** Sorted by seq (descending), showing mutation kind entries
- **Action:** Clicking entry dispatches history navigation

**Undo/Redo Commands:**
- **Dispatch:** `onAction({ controllerId: session.app.controllerId, action: "undo" })`
- **Dispatch:** `onAction({ controllerId: session.app.controllerId, action: "redo" })`
- **Button Behavior:** Disabled for viewers, enabled when canUndo/canRedo true

### Other History-Adjacent UI

**Checkpoint Checkin Dialog:**
- **Location:** ShellHost.tsx:5443-5458
- **Trigger:** `commitCheckpoint` action with message + authors
- **Effect:** Creates framework.history entry with actionId "commitCheckpoint"

**Uncommitted Edit Count:**
- **Location:** ShellHost.tsx:5355-5372
- **Derived from:** historyProjection.entries sorted by seq
- **Logic:** Count mutations since last commitCheckpoint entry

---

## 3. Persistence: Local-First Storage

### Storage Port Abstraction
- **Interface:** `StoragePort { get(key), set(key, value), remove(key) }` (🖥️platform/🟦️component.ts:238-242)
- **Key-value only:** No schema enforcement at port level

### Browser Storage Port
- **Implementation:** `createBrowserStoragePort()` (🖥️platform/🟦️component.ts:591-615)
- **Backend:** localStorage (with try-catch for quota/privacy errors)
- **Used by:** Demonstrator landing page, UI chrome, preferences

```typescript
export function createBrowserStoragePort(): StoragePort {
  return {
    get: (key) => localStorage.getItem(key),
    set: (key, value) => localStorage.setItem(key, value),
    remove: (key) => localStorage.removeItem(key)
  };
}
```

### Memory Storage Port (Ephemeral)
- **Implementation:** `createMemoryStoragePort()` (🖥️platform/🟦️component.ts:617-628)
- **Backend:** In-memory Map<string, string>
- **Used by:** Ephemeral branded shells (nothing survives refresh)

### Scoped Storage Port
- **Implementation:** `createScopedStoragePort(base, namespace)` (🖥️platform/🟦️component.ts:633-639)
- **Effect:** Prefixes all keys with `semio.shell.<namespace>.`
- **Used by:** Each FrameworkOsShell to isolate per-app storage

### Demonstrator Storage Architecture
- **Landing page:** `createBrowserStoragePort()` (demonstrator/📦️index.tsx:36)
  - Stores UI chrome appearance, layout, driver preferences
  - Persists across all six panes
  
- **Each pane shell:** Scoped storage with pane id (📦️index.tsx:421)
  - Pane id = "generator", "koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen"
  - Isolation: each pane's dock layout, window panes, preferences separate
  - **Backend:** Browser localStorage (via demonstrator's base port)

### OsShellConfig Storage
- **Persisted document:** `OsShellConfigSnapshot` (🖥️platform/🟦️component.ts:247-263)
- **Storage key:** `semio.os.config` (or `semio.shell.<namespace>.os.config` when scoped)
- **Fields:** preferences, namedLayouts, dockLayouts, dockUi, windowPanes
- **Layer strategy:** Global "os" defaults + per-app overrides

**No document data is persisted to localStorage.** Only UI chrome + preferences.

---

## 4. End-to-End Undo/Redo & History: Partial Implementation

### Undo/Redo Envelope Synthesis

**Location:** `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts:394-410`

**`rollbackEnvelope()` function:**
- Takes a speculative mutation envelope with precomputed `inverse`
- Synthesizes a new envelope with undoId = `${original.id}~undo`
- Inverse of the inverse = redoing the original operation
- Dependency: new envelope depends on original envelope id
- **Test:** Line 1205 "rollbackEnvelope synthesizes an undo from the original inverse"

```rust
// Rust twin in framework/plugin/rs/plugin_runtime.rs
fn rollback_envelope(envelope: MutationEnvelope) -> MutationEnvelope {
  let undo_id = format!("{}~undo", envelope.id);
  // Create new envelope with inverse diff as forward diff
  // Set dependencies to [original.id]
}
```

### Flow App Undo/Redo (Completed)

**Status:** Recently restored (cursor plan flow_undo_redo_catalogue_restore_4a7c7eb8.plan.md)

**Architecture:**
- **Core history:** `FlowHost::history` (flow/core/rs/lib.rs)
  - `FlowHistory` stack for change bookmarks
  - `begin_change()` wraps mutations
  - `undo()/redo()` navigate the stack

- **Plugin wiring:** flow/plugin/rs/lib.rs restores persistent FlowPlayApp
  - `host_for()` caches FlowHost per fixture
  - Commands route through `host.undo()/host.redo()`
  - Fixture patches preserve history (new `set_fixture_preserving_history()`)

**Tests:** cargo test passes for flow_core + flow-plugin; E2E verified in playground

**Verified:** Ctrl+Z/Ctrl+Shift+Z undo/redo add/connect/delete/move/rename/patch

### Other Apps: History Infrastructure Exists, Implementation Gaps

**ShellHost wiring (framework renderer):**
- `applyHistoryPatch()` dispatches history updates (ShellHost.tsx:1123)
- Undo/Redo buttons call `onAction(..., "undo"/"redo")` (ShellHost.tsx:5533-5534)
- `historyProjection` state tracks cursor, entries, can-undo/can-redo

**Plugin contract:**
- Plugins receive `HistoryPatch` wire messages from the store
- Plugins dispatch `"undo"` / `"redo"` command actions
- **Missing:** Each app must implement its own undo/redo handler

**Per-app implementation status (demonstrator apps):**
- ✓ Generator (flow-based, uses flow-core history)
- ✓ Koordinator (has coordinator/flow module, flow-core history)
- ? Aggregator (puzzle 3D, unclear if undo/redo implemented)
- ? Aussuchen (library browsing, may be read-only)
- ? Bearbeiten (CAD editing, unclear if history wired)
- ? Verfolgen (GIS tracking, unclear if history wired)

---

## 5. Document Persistence: Wave-1 Gap

### Documented Limitation
**Location:** demonstrator/📦️index.tsx:208

```typescript
/**
 * 🎪️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT: a booted pane that's fully offscreen or the
 * tab is backgrounded releases its live shell (plugin worker, WASM instances, WebGL contexts — see
 * the framework's teardown-on-unmount path) and shows a static poster instead, revived instantly on
 * hover/focus. Only PRISTINE panes (never interacted with) are ever suspended: there is no document
 * round-trip yet (`readAppDocument`/`loadAppDocument` are an unimplemented, documented Wave-1 gap in
 * the framework core), so suspending a pane the user actually used would silently discard their work.
 */
```

### Infrastructure Exists But Incomplete
- **WIT exports available:**
  - `readAppDocument(instanceId) → JSON` (ShellHost.tsx: calls on line 5465)
  - `loadAppDocument(instanceId, json) → void` (ShellHost.tsx: calls on line 5471)
  - `loadAppDocumentPack(instanceId, packBytes, sprBytes) → void` (ShellHost.tsx: line 5479)

- **Wired call sites:**
  - Tutorial base loading (ShellHost.tsx:5465)
  - Snapshot restore (ShellHost.tsx:5471)
  - Pack deserialization on snapshotReplaced event (ShellHost.tsx:5479)
  - Backwards navigation (ShellHost.tsx:5598)

- **Gap:** Apps must implement WIT export handlers; not all do
  - Flow: ✓ (flux document serialization)
  - Puzzle: ? (needs verification)
  - Others: ? (unclear)

---

## 6. Six Demonstrator Apps: Architecture & Status

### Pane Specifications
**Location:** demonstrator/🟦️brand.ts:789-796

| ID | Variant | Label | Tagline | Icon | Runtime | Manifest |
|----|---------|-------|---------|------|---------|----------|
| generator | generator | Generator | Parametrische Abläufe | workflow | procedural3d | generator |
| koordinator | koordinator | Koordinator | Modelle koordinieren | cad-shape | koordinator | koordinator |
| aggregator | aggregator | Aggregator | Bestand zusammensetzen | puzzle | puzzle | aggregator |
| aussuchen | aussuchen | Aussuchen | Bestand sichten | library | aussuchen | aussuchen |
| bearbeiten | bearbeiten | Bearbeiten | Bauteile anpassen | hammer | bearbeiten | bearbeiten |
| verfolgen | verfolgen | Verfolgen | Herkunft verfolgen | gis2d | verfolgen | verfolgen |

### Storage per Pane
**Location:** demonstrator/📦️index.tsx:421

Each pane uses `storageNamespace={pane.id}`, creating isolated localStorage buckets:
- `semio.shell.generator.os.config`
- `semio.shell.koordinator.os.config`
- ... etc

**Persisted:** Dock layouts, window panes, preferences — NOT document content

---

## 7. Gaps Blocking Full Version Control for All Six Apps

### Critical Gaps

1. **Document Persistence (All Apps)**
   - **Issue:** No cross-session document round-trip (`readAppDocument`/`loadAppDocument` unimplemented in most apps)
   - **Impact:** Suspending a pane with unsaved edits discards work; users cannot reload and continue
   - **Blocker:** Demonstrator cannot show reliable undo/redo UI if work is lost on refresh
   - **Fix required:** Each app must implement WIT export handlers + serialize document state

2. **Undo/Redo Wiring (Most Apps)**
   - **Issue:** History infrastructure exists in framework but apps don't dispatch `"undo"`/`"redo"` commands
   - **Status:** Flow ✓, others unknown/incomplete
   - **Impact:** History panel shows past edits but buttons don't work for non-Flow apps
   - **Fix required:** Each app must implement undo/redo handlers routing through AppStore mutations

3. **History Patch Generation (Store Layer)**
   - **Issue:** Store must emit `HistoryPatch` messages on every mutation
   - **Current:** Core infrastructure in place, but per-app mutation tracking incomplete
   - **Impact:** History panel entries sparse or missing for some operations
   - **Fix required:** Audit store dispatch paths; ensure all mutations generate history entries

4. **Checkpoint/Commit UI (All Apps)**
   - **Issue:** "Save version" / "Checkpoint" concept absent from six demonstrator panes
   - **Impact:** History shows edits but no user-initiated checkpoints with messages
   - **Note:** Framework supports commitCheckpoint action + checkin dialog, just not exposed in pane UIs
   - **Fix required:** Add checkpoint buttons to each pane; wire commitCheckpoint action

### Secondary Gaps

5. **Tutorial Recording (Aggregator Only)**
   - **Issue:** `TutorialDocumentEventKind::Edit` ops stub (hand-authored, not captured from live session)
   - **Note:** `.cursor/plans/flow_undo_redo_catalogue_restore_4a7c7eb8.plan.md` lines 114-128 document this
   - **Impact:** Tutorial playback edits document, but stubs don't reflect real user actions
   - **Fix required:** Run tutorial recorder on live session to capture real document mutations

6. **Presence & Collaboration Indicators**
   - **Issue:** Presence peers shown but no "edited by" annotations in history
   - **Impact:** History shows who made changes, but unclear which peer did what in multi-user sessions
   - **Note:** Infrastructure exists (`UiPresence`, `ArtifactPresencePeer`), needs UI integration

### Testing Gaps

7. **No E2E Verification**
   - **Issue:** No automated test for "edit → undo → redo → refresh → see same state"
   - **Currently:** Flow has unit tests, but not all six apps tested together
   - **Fix required:** Integration test across all panes; verify document persistence + undo/redo round-trip

---

## 8. Implementation Checklist for Full Version Control

**Per-app (×6 panes):**
- [ ] Implement `readAppDocument(instanceId) → JSON` WIT export
- [ ] Implement `loadAppDocument(instanceId, json)` WIT export
- [ ] Route undo/redo commands through app mutation dispatch
- [ ] Emit HistoryPatch updates on every mutation
- [ ] Add "Save Version" / "Checkpoint" button to UI
- [ ] Add "Undo" / "Redo" keyboard shortcuts (Ctrl+Z / Ctrl+Shift+Z)
- [ ] Test: edit → undo → redo → refresh → verify state persisted

**Framework (core):**
- [ ] Audit HistoryPatch generation in store dispatch paths
- [ ] Verify envelope.inverse precomputation for all mutation types
- [ ] Document per-app WIT export requirements

**Demonstrator shell:**
- [ ] Test pane suspension with dirty panes (should warn, not discard)
- [ ] Verify history panel UI mirrors backend projection accurately
- [ ] Add integration test: six-pane round-trip workflow

---

## References

- **VCS core:** `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- **Store:** `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- **Backbone:** `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`
- **Renderer:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- **Platform/Storage:** `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts`
- **Demonstrator:** `♻️mit-bestand/🧺️demonstrator/📦️index.tsx`, `🟦️brand.ts`
- **Flow plan:** `.cursor/plans/flow_undo_redo_catalogue_restore_4a7c7eb8.plan.md`
