# Extent Budget Audit: puzzle3d Editor Actions

**Audit Date:** 2026-09-05  
**File Audited:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`  
**File mtime:** Sep 5 04:18  
**Test Document:** Nakagin Capsule Tower (128,755 bytes)

## Nakagin Entity Count

**DSL Structure Analysis**  
Counted from `.dsl.semio` fixture (lines 1-249):

- **Object instances:** 121
- **Object kinds (catalog):** 12
- **Vortex kinds (catalog):** 18
- **Cable kinds:** 1
- **Attraction kinds:** 1
- **Attraction instances:** 0
- **Target volume instances:** 0
- **Reference instances:** 0
- **Kind-compatibility rules:** 14

**Counting Method:** Parsed DSL sections by delimiter:
- Lines 63-243: object instances (one per line starting with UUID)
- Lines 5-17: catalog object/kind definitions (12 entries)
- Lines 20-37: vortex kind definitions (18 entries)
- Lines 46-59: compatibility entries (14 rules)
- Attraction, target-volume, reference sections empty

## Capacity Constants

- `PUZZLE_COMMAND_WORK_ITEMS` = 4,096
- `PUZZLE_COMMAND_DECODED_ITEMS` = 512
- `PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT` = 64

Source: `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs` line 11 and editor file line 4131.

## Verdict Table

| Work Type | Tool IDs | Extent Result | Verdict | Notes |
|-----------|----------|----------------|---------|-------|
| `Puzzle3dScalarConfigWork` | setCamera, setProjection, setLodAutomatic, setVortexShow, etc. (27 ids) | 2 | **SAFE** | Constant 2 |
| `Puzzle3dKindWeightWork` | setObjectKindWeight | 51 | **SAFE** | 12 kinds × 4 + 3 = 51 ≤ 4,096 |
| `Puzzle3dKindWeightWork` | setVortexKindWeight | 75 | **SAFE** | 18 kinds × 4 + 3 = 75 ≤ 4,096 |
| `Puzzle3dEngagementAbortWork` | engagementAbort | 4 | **SAFE** | Constant 4 |
| `Puzzle3dEngagementRepeatWork` | engagementRepeatLast | 2 | **SAFE** | Constant 2 |
| `Puzzle3dAddObjectKindWork` | addObjectKind | 143 | **SAFE** | 12 + 128 + 3 = 143 ≤ 4,096 |
| `Puzzle3dScaleWork` | translateSelection, rotateSelection, scaleSelection | 242 | **SAFE** | 121 + 0 + 121 + 0 = 242 ≤ 4,096 |
| `Puzzle3dPatchInspectorWork` | patchInspector | **None** | **DEAD** | Vortex entity: 121 × 512 = 61,952 > 4,096 |
| `Puzzle3dWorldRelocateWork` | worldRelocate | **None** | **EXCEEDS CAP** | 121 × 2 + 121 × 64 = 7,986 > 4,096 |
| `Puzzle3dCreateAttractionWork` | createAttraction | **None** | **EXCEEDS CAP** | 121 × 64 × 2 + 14 + 1 = 15,503 > 4,096 |
| `Puzzle3dSetActiveExampleWork` | setActiveExample | **?** | **CANNOT DETERMINE** | Depends on target document size from command args |
| `Puzzle3dAddBrushObjectWork` | addBrushObject | 144 | **SAFE** | 12 + 128 + 0 + 4 = 144 ≤ 4,096 |
| `Puzzle3dFocusSelectionWork` | focusSelection | 364 | **SAFE** | 121 + 242 + 1 = 364 ≤ 4,096 (all objects selected) |
| `Puzzle3dEngagementSubmitWork` | engagementSubmit | 370 | **SAFE** | 121 + 242 + 7 = 370 ≤ 4,096 (all objects selected) |
| `Puzzle3dRelocateVolumeWork` | relocateTargetVolume | 4 | **SAFE** | 0 + 4 = 4 ≤ 4,096 |
| `Puzzle3dAcceptSuggestionWork` | acceptSuggestion | **None** | **EXCEEDS CAP** | 121 × 64 + 12 + 128 + 0 + 4 = 7,888 > 4,096 |
| `Puzzle3dPrecomputeCommandWork` | cycleBrushCandidate, fillBuildTick, registerBrushMesh, setFillCount, suggestionsTick | 1 | **SAFE / UNKNOWN** | Returns 1 if args validate; depends on command `positions` and `indices` arrays ≤ 512 |
| `BoundedFirstStepCommandWork` (fallback) | addTargetVolume, openAddObjectDialog, and ~10 others | ~1 | **SAFE** | Default fallback; extent 1 for most unmapped actions |

## Dead Actions (Render-Blocking Issues)

### EXCEEDS CAP (3 actions)

1. **`worldRelocate`** → `Puzzle3dWorldRelocateWork`
   - **Extent Formula:** `121 × 2 + (121 × 64) = 7,986`
   - **Line:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:4223–4226`
   ```rust
   let object_vortices = document.objects.len().checked_mul(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT)?;
   let items = document.objects.len().checked_mul(2)?.checked_add(object_vortices)?.checked_add(document.attractions.len())?;
   (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
   ```
   - **Cap:** 4,096
   - **Surplus:** 3,890 items

2. **`createAttraction`** → `Puzzle3dCreateAttractionWork`
   - **Extent Formula:** `(121 × 64 × 2) + 0 + 14 + 1 = 15,503`
   - **Line:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:4497–4500`
   ```rust
   let endpoint_scans = document.objects.len().checked_mul(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT)?.checked_mul(2)?;
   let items = document.attractions.len().checked_add(endpoint_scans)?.checked_add(document.meta.kind_compatibility.len())?.checked_add(1)?;
   (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
   ```
   - **Cap:** 4,096
   - **Surplus:** 11,407 items

3. **`acceptSuggestion`** → `Puzzle3dAcceptSuggestionWork`
   - **Extent Formula:** `(121 × 64) + 12 + 128 + 0 + 4 = 7,888`
   - **Line:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:5710–5715`
   ```rust
   let target_scans = document.objects.len().checked_mul(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT)?;
   let items = target_scans
       .checked_add(catalogs.objects.len())?
       .checked_add(PUZZLE3D_RELOCATE_VORTICES_PER_OBJECT.checked_mul(2)?)?
       .checked_add(document.attractions.len())?
       .checked_add(4)?;
   (items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
   ```
   - **Cap:** 4,096
   - **Surplus:** 3,792 items

### RETURNS NONE (1 action)

4. **`patchInspector`** → `Puzzle3dPatchInspectorWork`
   - **Extent Formula (vortex case):** `object_selection + (121 × 512) = ~61,952`
   - **Line:** `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:3986–3995`
   ```rust
   let scan = match Self::entity(command) {
       "object" => document.objects.len(),
       "vortex" => document.objects.len().checked_mul(crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS)?,
       ...
   };
   let items = source.checked_add(scan)?;
   (source <= crate::retained_command::PUZZLE_COMMAND_DECODED_ITEMS && items <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS).then_some(items)
   ```
   - **Cap:** 4,096
   - **Actual (vortex):** 121 × 512 ≈ 61,952
   - **Multiplier Error:** 512× over budget. This is a **design error**, not precision edge case — the vortex scan multiplies by `DECODED_ITEMS` when it should scale differently.

## Summary of Findings

**Dead Actions (Cannot Run on Nakagin):**
- `worldRelocate` — 7,986 items (194% over cap)
- `createAttraction` — 15,503 items (379% over cap)
- `acceptSuggestion` — 7,888 items (193% over cap)
- `patchInspector` (vortex mode) — ~61,952 items (1,514% over cap)

**Safe Actions:**
- All scalar config (setCamera, setProjection, etc.) — constant budget
- All simple toggles (engagementAbort, engagementRepeatLast) — constant budget
- Kind weight setters (setObjectKindWeight, setVortexKindWeight) — linear in catalog size, well under cap
- Scale transformations (translateSelection, rotateSelection, scaleSelection) — scales with object selection (max 121, fits)
- Selection focus (focusSelection, engagementSubmit) — scales with selection, under cap
- Volume and brush management — small constant or fits

**Uncertain:**
- `setActiveExample` — depends on target document size (command argument)
- `Puzzle3dPrecomputeCommandWork` — depends on command-provided arrays (positions, indices)

## Root Cause Analysis

The dead actions share a common pattern: **vortex-per-object scaling (× 64) or decoded-items scaling (× 512) applied to object lists**. 

On Nakagin's 121 objects:
- 121 × 64 = 7,744 (already 189% of cap before adding anything else)
- 121 × 512 = 61,952 (exceeds by 15×)

These actions work fine on small documents (< 50 objects), but fail hard on realistic architectural models. The cap of 4,096 assumes small working sets and does not scale to puzzle3d's use case of placing 100+ objects in a single scene.

## Recommendations

1. Increase `PUZZLE_COMMAND_WORK_ITEMS` to at least 65,536 (16× current), or
2. Redesign the extent formulas to avoid full object-scan multipliers, or
3. Implement streaming/pagination for large document operations

The three EXCEEDS CAP actions could potentially be fixed by reducing the vortex scan budget or splitting work into multiple steps. The patchInspector vortex case is a design error requiring architectural fix.
