# Generic Fallback Arm Audit: `build_tool_job` (Line 6715)

## Verdict Summary

**UNSAFE IDs (MUST NOT migrate via generic arm):**
- `setFillCountStep` → **SILENT NO-OP** at runtime

**SAFE IDs (can migrate):**
- `openAddObjectDialog` ✓
- `worldPointerDown` ✓
- `addTargetVolume` ✓
- `deleteAttraction` ✓
- `deleteSelection` ✓
- `deleteTargetVolume` ✓
- `duplicateSelection` ✓
- `openVortexSuggestions` ✓
- `selectSameKindSelection` ✓
- `setFixtureJson` ✓
- `setSelectionFlag` ✓
- `setTargetVolumeFlag` ✓

---

## Step 1: Exact Set of IDs Falling Through to Generic Arm

IDs declared in `.action_interactive_job` (lines 7252–7316) but **not** explicitly matched in `build_tool_job`'s match arms (lines 6666–6714):

| ID | In build_tool_job | Falls through |
|----|----|-----|
| acceptSuggestion | ✓ line 6678 | No |
| addBrushObject | ✓ line 6672 | No |
| addObjectKind | ✓ line 6673 | No |
| addTargetVolume | ✗ | **Yes** |
| closeVortexSuggestions | ✓ line 6708 | No |
| createAttraction | ✓ line 6670 | No |
| cycleBrushCandidate | ✓ line 6679 | No |
| cycleBrushCandidateBack | ✓ line 6680 | No |
| deleteAttraction | ✗ | **Yes** |
| deleteSelection | ✗ | **Yes** |
| deleteTargetVolume | ✗ | **Yes** |
| duplicateSelection | ✗ | **Yes** |
| engagementAbort | ✓ line 6674 | No |
| engagementControlSelect | ✓ line 6710 | No |
| engagementInput | ✓ line 6711 | No |
| engagementRepeatLast | ✓ line 6675 | No |
| engagementSubmit | ✓ line 6676 | No |
| fillBuildTick | ✓ line 6681 | No |
| focusSelection | ✓ line 6685 | No |
| hoverSuggestion | ✓ line 6709 | No |
| openAddObjectDialog | ✗ | **Yes** |
| openVortexSuggestions | ✗ | **Yes** |
| patchInspector | ✓ line 6668 | No |
| registerBrushMesh | ✓ line 6682 | No |
| relocateTargetVolume | ✓ line 6686 | No |
| rotateSelection | ✓ line 6667 | No |
| scaleSelection | ✓ line 6667 | No |
| selectSameKindSelection | ✗ | **Yes** |
| setActiveExample | ✓ line 6671 | No |
| setBrushPlacementOverlapBudget | ✓ line 6707 | No |
| setCamera | ✓ line 6687 | No |
| setChunkSize | ✓ line 6702 | No |
| setFillCount | ✓ line 6683 | No |
| setFillCountStep | ✗ | **Yes** |
| setFixtureJson | ✗ | **Yes** |
| setGridSnapEnabled | ✓ line 6698 | No |
| setGridSpacing | ✓ line 6699 | No |
| setGridVisible | ✓ line 6697 | No |
| setLocale | ✓ line 6712 | No |
| setLodAutomatic | ✓ line 6694 | No |
| setLodDepthVariable | ✓ line 6695 | No |
| setLodManual | ✓ line 6696 | No |
| setObjectKindWeight | ✓ line 6677 | No |
| setProjection | ✓ line 6688 | No |
| setProjectionParam | ✓ line 6689 | No |
| setProximityRadius | ✓ line 6701 | No |
| setSelectableKind | ✓ line 6700 | No |
| setSelectionFlag | ✗ | **Yes** |
| setSunAzimuth | ✓ line 6691 | No |
| setSunElevation | ✓ line 6692 | No |
| setSunIntensity | ✓ line 6693 | No |
| setTargetVolumeFlag | ✗ | **Yes** |
| setTerminology | ✓ line 6712 | No |
| setTransformGumballFlag | ✓ line 6704 | No |
| setVortexDirection | ✓ line 6706 | No |
| setVortexKindWeight | ✓ line 6677 | No |
| setVortexShow | ✓ line 6705 | No |
| setVoxelDims | ✓ line 6703 | No |
| suggestionsTick | ✓ line 6684 | No |
| toggleSun | ✓ line 6690 | No |
| transformBegin | ✓ line 6714 | No |
| transformEnd | ✓ line 6714 | No |
| translateSelection | ✓ line 6667 | No |
| worldPointerDown | ✓ line 6714 | No |
| worldRelocate | ✓ line 6669 | No |

**IDs falling through (13 total):**
1. `addTargetVolume`
2. `deleteAttraction`
3. `deleteSelection`
4. `deleteTargetVolume`
5. `duplicateSelection`
6. `openAddObjectDialog`
7. `openVortexSuggestions`
8. `selectSameKindSelection`
9. `setFixtureJson`
10. `setSelectionFlag`
11. `setTargetVolumeFlag`
12. `setFillCountStep` ← Note: `"setFillCountStep"` is the value of `set_fill_count::STEP_ACTION_ID` (defined at line 13 of `🧮️set-fill-count/🦀️.rs`)
13. `worldPointerDown` (already explicitly matched, so doesn't fall through)

Corrected list (12 ids actually fall through):
1. `addTargetVolume`
2. `deleteAttraction`
3. `deleteSelection`
4. `deleteTargetVolume`
5. `duplicateSelection`
6. `openAddObjectDialog`
7. `openVortexSuggestions`
8. `selectSameKindSelection`
9. `setFixtureJson`
10. `setSelectionFlag`
11. `setTargetVolumeFlag`
12. `setFillCountStep`

---

## Step 2: Reducer & Extent Analysis

### `puzzle3d_retained_reduce` (lines 2548–2590)

```rust
fn puzzle3d_retained_reduce(
    command: &Puzzle3dCommand,
    snapshot: &Puzzle3dPlaySnapshot,
    config: &Puzzle3dConfig,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
) -> Result<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>, Fault> {
    // Line 2560: explicit handler
    if command.action_id() == "openAddObjectDialog" {
        return Ok(Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(120), dialog_id: "addObject".into(), args: None }));
    }
    // Line 2563: explicit handler
    if command.action_id() == "worldPointerDown" {
        return Ok(Emit::default());
    }
    // Line 2566: explicit handler
    if command.action_id() == "addTargetVolume" {
        let Some(origin) = command.args().and_then(|args| args.get("origin")).and_then(value_as_vec3) else { return Ok(Emit::default()) };
        let options = command.window_id().and_then(|window_id| config.window_options.get(window_id));
        let grid_spacing = options.map_or(config.grid_spacing, |options| options.grid_spacing).max(0.1);
        let voxel_dims = options.map_or(config.voxel_dims, |options| options.voxel_dims);
        let snapped = [(origin[0] / grid_spacing).round() * grid_spacing, (origin[1] / grid_spacing).round() * grid_spacing, (origin[2] / grid_spacing).round() * grid_spacing];
        let scale = crate::artifacts::puzzle3d::Puzzle3dScale::Vec3([voxel_dims[0] as f64 * grid_spacing, voxel_dims[1] as f64 * grid_spacing, voxel_dims[2] as f64 * grid_spacing]);
        let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let volume = crate::artifacts::puzzle3d::Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false };
        return Ok(Emit { artifact_mutations: vec![crate::artifacts::puzzle3d::mutations::create_target_volume(volume, None)], ui_scope: UiDirtyScope::Full, ..Default::default() });
    }
    let empty_selection = protocol::DomainSelection::default();
    let selection = interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).unwrap_or(&empty_selection);
    Ok(with_puzzle3d_app_for(config, |app| {
        // Line 2580: explicit handler
        if command.action_id() == "fillBuildTick" {
            if let Some(emit) = fill_build_tick::fill_build_tick_cached(app, config) {
                return emit;
            }
        }
        // Line 2585: explicit handler
        if command.action_id() == "setFillCount" {
            let mut precompute = app.precompute.borrow_mut();
            if !config.fill_checkpoint.is_empty() {
                precompute.restore_persisted_fill(&config.fill_checkpoint);
            }
            precompute.set_fill_applied_count(config.fill_applied_count);
            return set_fill_count::begin(&mut precompute, config, command.args());
        }
        // Line 2588: DEFAULT: falls through to handle_action_impl
        app.handle_action_impl(command.action_id(), command.args(), command.window_id(), snapshot, config, selection)
    }))
}
```

### `puzzle3d_retained_extent` (lines 2533–2546)

```rust
fn puzzle3d_retained_extent(command: &Puzzle3dCommand, snapshot: &Puzzle3dPlaySnapshot, interaction: &protocol::InteractionState) -> Option<usize> {
    // Line 2539: explicit special case
    if matches!(command.action_id(), "addTargetVolume" | "openAddObjectDialog" | "worldPointerDown") {
        return Some(1);
    }
    let selection = interaction.selection.get(PUZZLE3D_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len());
    let document = snapshot.typed();
    // Lines 2544–2549: targeted document_items calculations for specific actions
    let document_items = match command.action_id() {
        "focusSelection" | "patchInspector" | "translateSelection" | "rotateSelection" | "scaleSelection" | "transformEnd" => document.objects.len().checked_add(document.target_volumes.len())?,
        "createAttraction" | "worldRelocate" => document.objects.len().checked_add(document.attractions.len())?,
        "addObjectKind" | "setObjectKindWeight" | "setVortexKindWeight" => document.meta.kind_catalogs.as_ref().map_or(0, |catalogs| catalogs.objects.len().saturating_add(catalogs.vortices.len())),
        // Line 2548: DEFAULT for all others
        _ => 1,
    };
    selection.checked_add(document_items).filter(|items| *items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS)
}
```

### Per-ID Analysis

| ID | In puzzle3d_retained_reduce | Route | In dispatch_puzzle3d_action | Verdict |
|----|----|----|----|----|
| `openAddObjectDialog` | ✓ line 2560 | Direct | N/A | **SAFE** — special-cased in reducer, produces `Emit::effect(OpenDialog)` |
| `worldPointerDown` | ✓ line 2563 | Direct | N/A | **SAFE** — special-cased in reducer, produces `Emit::default()` |
| `addTargetVolume` | ✓ line 2566 | Direct | N/A | **SAFE** — special-cased in reducer, creates target volume mutation |
| `deleteAttraction` | ✗ | Fallback→dispatch (2588) | ✓ line 2461: `delete_attraction::delete_attraction(ctx, args)` | **SAFE** — handled in dispatch |
| `deleteSelection` | ✗ | Fallback→dispatch (2588) | ✓ line 2456: `delete_selection::delete_selection(ctx)` | **SAFE** — handled in dispatch |
| `deleteTargetVolume` | ✗ | Fallback→dispatch (2588) | ✓ line 2463: `delete_target_volume::delete_target_volume(ctx, args)` | **SAFE** — handled in dispatch |
| `duplicateSelection` | ✗ | Fallback→dispatch (2588) | ✓ line 2457: `duplicate_selection::duplicate_selection(ctx)` | **SAFE** — handled in dispatch |
| `openVortexSuggestions` | ✗ | Fallback→dispatch (2588) | ✓ line 2489: `open_vortex_suggestions::open_vortex_suggestions(ctx, args)` | **SAFE** — handled in dispatch |
| `selectSameKindSelection` | ✗ | Fallback→dispatch (2588) | ✓ line 2453: `select_same_kind::select_same_kind(ctx)` | **SAFE** — handled in dispatch |
| `setFixtureJson` | ✗ | Fallback→dispatch (2588) | ✓ line 2451: `set_fixture_json::set_fixture_json(ctx, args)` | **SAFE** — handled in dispatch |
| `setSelectionFlag` | ✗ | Fallback→dispatch (2588) | ✓ line 2458: `set_selection_flag::set_selection_flag(ctx, args)` | **SAFE** — handled in dispatch |
| `setTargetVolumeFlag` | ✗ | Fallback→dispatch (2588) | ✓ line 2464: `set_target_volume_flag::set_target_volume_flag(ctx, args)` | **SAFE** — handled in dispatch |
| `setFillCountStep` | ✗ | Fallback→dispatch (2588) | ✗ line 2506: `_ => {}` (default case) | **UNSAFE** — NOT handled, falls to default case |

### Key Finding: `setFillCountStep` Silent No-Op

`setFillCountStep` is the step-continuation variant of `setFillCount` (defined as `"setFillCountStep"` at line 13 of `🧮️set-fill-count/🦀️.rs`). 

**In the legacy path** (`ArtifactEditor::handle` at line 6844):
```rust
if matches!(command.action_id(), "setFillCount" | set_fill_count::STEP_ACTION_ID) {
    // ... handled at line 6853
    return if command.action_id() == "setFillCount" { 
        set_fill_count::begin(&mut precompute, &cfg.snapshot, command.args()) 
    } else { 
        set_fill_count::step(&mut precompute, &cfg.snapshot, command.args()) 
    };
}
```

**In the retained command path** via `puzzle3d_retained_reduce`:
- Not explicitly handled (no special-case at line 2560, 2563, 2566, 2580, or 2585)
- Falls through to `app.handle_action_impl` (line 2588)
- Which calls `dispatch_puzzle3d_action` (line 2393 in handle_action_impl)
- Which has NO handler for `setFillCountStep` — falls to line 2506: `_ => {}` (silent no-op)

---

## Step 3: `BoundedFirstStepCommandWork` Behavior

From `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:59–95`:

```rust
pub struct BoundedFirstStepCommandWork<A: ArtifactApp> {
    tool_id: &'static str,
    reducer: PuzzleCommandReducer<A>,
    extent: PuzzleCommandExtent<A>,
    consumed: bool,
}

impl<A: ArtifactApp> PuzzleCommandWork<A> for BoundedFirstStepCommandWork<A> {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &A::Command, snapshot: &A::Snapshot, interaction: &protocol::InteractionState) -> Option<usize> {
        // Line 78: directly forwards extent function result
        (self.extent)(command, snapshot, interaction)
    }

    fn step(
        &mut self,
        command: &A::Command,
        snapshot: &A::Snapshot,
        config: &A::Config,
        interaction: &protocol::InteractionState,
        hover: &InteractionHoverState,
    ) -> Result<PuzzleCommandWorkStep<A>, Fault> {
        if self.consumed {
            return Err(Fault::from("puzzle-command-bounded-work-repeated"));
        }
        // Line 92: reducer is called
        let emit = (self.reducer)(command, snapshot, config, interaction, hover)?;
        self.consumed = true;
        // Line 94: result is returned as Complete
        Ok(PuzzleCommandWorkStep::Complete(emit))
    }
}
```

### Behavior When Reducer Returns Nothing or Errors

1. **If `puzzle3d_retained_reduce` returns `Err(Fault)`:** The `?` operator at line 92 propagates it — the step returns the error, and the job faults.

2. **If `puzzle3d_retained_reduce` returns `Ok(Emit::default())`:** The step completes successfully with an empty `Emit`. The framework then:
   - Issues no mutations
   - Emits no effects
   - Silently completes

3. **If `puzzle3d_retained_extent` returns `None`:** The extent is unknown. The framework may reject the job or use a default limit, depending on caller logic.

### For `setFillCountStep` Specifically

1. `puzzle3d_retained_extent("setFillCountStep", ...)` returns `Some(1)` (line 2548 default case)
2. `puzzle3d_retained_reduce("setFillCountStep", ...)` falls to `dispatch_puzzle3d_action` (line 2588)
3. `dispatch_puzzle3d_action` has no handler — executes `_ => {}` (line 2506)
4. The context's effects/mutations remain empty
5. `app.handle_action_impl` returns `Emit { artifact_mutations: Vec::new(), config_mutations: Vec::new(), effects: Vec::new(), ... }` (derived from the empty context)
6. `BoundedFirstStepCommandWork::step` completes with this empty `Emit`
7. **Result: Silent no-op — no mutations, no errors, no visible indication that the action failed**

---

## Step 4: Legacy Dispatch Cross-Check

Legacy dispatch table: `dispatch_puzzle3d_action` (lines 2449–2507) and `puzzle3d_action_document_intent`/`handle_action_impl` (line 2344 onward).

### Legacy (Non-Retained) Behavior

For all 12 IDs that fall through to the generic arm:

| ID | Legacy dispatch (line) | Legacy handler | Retained path via generic arm | Match? |
|----|----|----|----|----|
| `openAddObjectDialog` | 2347 in handle_action_impl | Explicit handler: `Emit::effect(OpenDialog)` | Same behavior (line 2560 in reducer) | ✓ Yes |
| `worldPointerDown` | 2350 in handle_action_impl | Explicit handler: `Emit::default()` | Same behavior (line 2563 in reducer) | ✓ Yes |
| `addTargetVolume` | 2462 in dispatch | Calls `add_target_volume::add_target_volume(ctx, args)` | Same behavior (line 2566 in reducer) | ✓ Yes |
| `deleteAttraction` | 2461 in dispatch | Calls `delete_attraction::delete_attraction(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| `deleteSelection` | 2456 in dispatch | Calls `delete_selection::delete_selection(ctx)` | Fallback→dispatch→same handler | ✓ Yes |
| `deleteTargetVolume` | 2463 in dispatch | Calls `delete_target_volume::delete_target_volume(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| `duplicateSelection` | 2457 in dispatch | Calls `duplicate_selection::duplicate_selection(ctx)` | Fallback→dispatch→same handler | ✓ Yes |
| `openVortexSuggestions` | 2489 in dispatch | Calls `open_vortex_suggestions::open_vortex_suggestions(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| `selectSameKindSelection` | 2453 in dispatch | Calls `select_same_kind::select_same_kind(ctx)` | Fallback→dispatch→same handler | ✓ Yes |
| `setFixtureJson` | 2451 in dispatch | Calls `set_fixture_json::set_fixture_json(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| `setSelectionFlag` | 2458 in dispatch | Calls `set_selection_flag::set_selection_flag(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| `setTargetVolumeFlag` | 2464 in dispatch | Calls `set_target_volume_flag::set_target_volume_flag(ctx, args)` | Fallback→dispatch→same handler | ✓ Yes |
| **`setFillCountStep`** | **NOT in dispatch** | **Default case: `_ => {}`** | **NOT in dispatch→default case: `_ => {}`** | ✗ No handler in either path |

### Critical Discrepancy for `setFillCountStep`

- **Declared** as an interactive job (line 7285, classified as `Migrated`)
- **Never handled** in `dispatch_puzzle3d_action`
- **Manually handled** ONLY in `ArtifactEditor::handle` (line 6844) — the legacy non-retained path
- **When dispatched through the retained path**, it silently does nothing

This is a **migration hazard**: `setFillCountStep` was added to the retained tool IDs list but never wired into the retained reducer's dispatch table. It was pre-implemented only for the legacy `handle()` path.

---

## Summary

### SAFE to migrate (11 IDs):
These have genuine handlers in either `puzzle3d_retained_reduce` directly or via `dispatch_puzzle3d_action`:
1. `openAddObjectDialog` (line 2560 in reducer)
2. `worldPointerDown` (line 2563 in reducer)
3. `addTargetVolume` (line 2566 in reducer)
4. `deleteAttraction` (line 2461 in dispatch)
5. `deleteSelection` (line 2456 in dispatch)
6. `deleteTargetVolume` (line 2463 in dispatch)
7. `duplicateSelection` (line 2457 in dispatch)
8. `openVortexSuggestions` (line 2489 in dispatch)
9. `selectSameKindSelection` (line 2453 in dispatch)
10. `setFixtureJson` (line 2451 in dispatch)
11. `setSelectionFlag` (line 2458 in dispatch)
12. `setTargetVolumeFlag` (line 2464 in dispatch)

### UNSAFE to migrate (1 ID):
**`setFillCountStep`** — Falls through to default case in `dispatch_puzzle3d_action` (line 2506) and produces a silent no-op. It needs either:
- An explicit arm in `puzzle3d_retained_reduce` that calls `set_fill_count::step()` (like line 6853 in the legacy handle path), OR
- An arm in `dispatch_puzzle3d_action` (like the legacy `setFillCount` handler), OR
- To remain explicitly matched in `build_tool_job` with custom work (current safe state at line 6683)

---

## Required Fix for `setFillCountStep`

Add to `dispatch_puzzle3d_action` (before the default case at line 2506):
```rust
"setFillCountStep" => {
    // Calls set_fill_count::step, which continues fill materialization
    // (matches behavior in ArtifactEditor::handle at line 6853)
}
```

OR add special case to `puzzle3d_retained_reduce` before the fallback (before line 2588):
```rust
if command.action_id() == "setFillCountStep" {
    let mut precompute = app.precompute.borrow_mut();
    precompute.restore_persisted_fill(&config.fill_checkpoint);
    precompute.set_fill_applied_count(config.fill_applied_count);
    return set_fill_count::step(&mut precompute, config, command.args());
}
```
