# Puzzle3d Lane Audit

**File analyzed:** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`  
**Fixture:** `✏️s/🔌️plugins/🧩️puzzle/🧪️publication-authority/🔣️.json`  
**Runtime gate:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22897-22904`

## Findings

### Confirmed Mismatches (Runtime Failures)

1. **addBrushObject** (Group 5, declared `["Artifact","Config"]`)
   - Work type: `Puzzle3dAddBrushObjectWork` (line 5004)
   - Emission (line 5151-5155): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations

2. **addObjectKind** (Group 5, declared `["Artifact","Config"]`)
   - Work type: `Puzzle3dAddObjectKindWork` (line 3312)
   - Emission (line 3413-3417): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations

3. **rotateSelection** (Group 5, declared `["Artifact","Config"]`)
   - Work type: `Puzzle3dScaleWork` (line 3550), shared with translateSelection/scaleSelection
   - Emission (line 3623-3628): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations

4. **scaleSelection** (Group 5, declared `["Artifact","Config"]`)
   - Work type: `Puzzle3dScaleWork` (line 3550)
   - Emission (line 3623-3628): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations

5. **translateSelection** (Group 5, declared `["Artifact","Config"]`)
   - Work type: `Puzzle3dScaleWork` (line 3550)
   - Emission (line 3623-3628): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations

6. **engagementAbort** (Group 6, declared `["Config"]`)
   - Work type: `Puzzle3dEngagementAbortWork` (line 3076)
   - Emission (line 3126-3131): `config_mutations` + `effects`
   - Issue: Emits effects but declares no HostOnly lane (only Config)
   - Error at runtime: "typed-operation emitted a store lane absent from its exact factory publication contract" (line 22904)

7. **engagementSubmit** (Group 6, declared `["Config"]`)
   - Work type: `Puzzle3dEngagementSubmitWork` (line 5388)
   - Emission (line 5464): Delegates to FocusSelectionWork and collects result
   - Accumulated emissions (line 5440, 5446, 5452, 5458): `config_mutations` + `effects`
   - Issue: Emits effects but declares no HostOnly lane (only Config)
   - Error at runtime: Same as above

8. **relocateTargetVolume** (Group 6, declared `["Config"]`)
   - Work type: `Puzzle3dRelocateVolumeWork` (line 5539)
   - Emission (line 5527-5531): `artifact_mutations` only
   - Issue: Declares Config lane but emits no config_mutations; emits Artifact instead

### Detailed Work Type Analysis

| Work Type | Tool IDs | Emits | File:Line |
|-----------|----------|-------|-----------|
| Puzzle3dScalarConfigWork | setCamera, setProjection, toggleSun, setLodAutomatic, setGridVisible, setBrushPlacementOverlapBudget, setSelectableKind, setChunkSize, setVoxelDims, setTransformGumballFlag, setVortexShow, setVortexDirection, hoverSuggestion, engagementControlSelect, engagementInput, setLocale, setTerminology, closeVortexSuggestions | config_mutations | 2787 |
| Puzzle3dKindWeightWork | setObjectKindWeight, setVortexKindWeight | config_mutations | 3010-3014 |
| Puzzle3dEngagementAbortWork | engagementAbort | config_mutations + effects | 3127-3128 |
| Puzzle3dEngagementRepeatWork | engagementRepeatLast | effects only | 3212 |
| Puzzle3dAddObjectKindWork | addObjectKind | artifact_mutations | 3413-3417 |
| Puzzle3dScaleWork | translateSelection, rotateSelection, scaleSelection | artifact_mutations | 3623-3628 |
| Puzzle3dPatchInspectorWork | patchInspector | artifact_mutations | 3793 (complete method) |
| Puzzle3dWorldRelocateWork | worldRelocate | artifact_mutations | 4204-4208 |
| Puzzle3dCreateAttractionWork | createAttraction | artifact_mutations | 4576-4592 |
| Puzzle3dSetActiveExampleWork | setActiveExample | artifact_mutations + config_mutations | 4854-4858 |
| Puzzle3dAddBrushObjectWork | addBrushObject | artifact_mutations | 5151-5155 |
| Puzzle3dFocusSelectionWork | focusSelection | config_mutations | 5317-5321 |
| Puzzle3dEngagementSubmitWork | engagementSubmit | config_mutations + effects | 5440, 5446, 5452, 5458, 5464 |
| Puzzle3dRelocateVolumeWork | relocateTargetVolume | artifact_mutations | 5527-5531 |
| Puzzle3dAcceptSuggestionWork | acceptSuggestion | artifact_mutations + config_mutations | 5879-5883 |
| Puzzle3dPrecomputeCommandWork | cycleBrushCandidate, cycleBrushCandidateBack, setFillCount, fillBuildTick, suggestionsTick, registerBrushMesh | config_mutations (conditional, see below) | 6099-6124 |
| NoopPuzzleCommandWork | worldPointerDown, transformBegin, transformEnd | (empty Emit) | Line 131 in retained/🦀️.rs |
| BoundedFirstStepCommandWork | Fallback | Depends on puzzle3d_retained_reduce | 2548+ |

### Puzzle3dPrecomputeCommandWork Lane Breakdown (conditional)

Tool ID dispatch at publication stage (line 6099-6122):
- **cycleBrushCandidate**: config_mutations (line 6106)
- **cycleBrushCandidateBack**: config_mutations (line 6106)
- **setFillCount**: config_mutations (line 6110)
- **fillBuildTick**: ui_scope only (line 6117)
- **suggestionsTick**: ui_scope only (line 6119)
- **registerBrushMesh**: ui_scope only (line 6120)

### Runtime Validation Gate

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22897-22904`

```rust
if (!emit.artifact_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Artifact))
    || (!emit.config_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Config))
    || (!emit.draft_mutations.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Draft))
    || (!ephemeral.presence.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Presence))
    || (!ephemeral.transient.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Transient))
    || (!emit.child_emits.is_empty() && !publication_lanes.contains(&ArtifactToolPublicationLane::Child))
{
    return Err(plugin_sdk_fault("typed-operation emitted a store lane absent from its exact factory publication contract"));
}
```

This gate executes for every completion. If any mutation type is emitted that isn't declared in the lane contract, the operation fails silently at runtime with the error message shown above.

## Summary

Eight actions have mismatched lane declarations:
- **5 emit Artifact without Config**: addBrushObject, addObjectKind, rotateSelection, scaleSelection, translateSelection
- **2 emit effects outside HostOnly lane**: engagementAbort, engagementSubmit
- **1 emits Artifact instead of Config**: relocateTargetVolume

All 8 will fail at runtime when their completions are published, triggering the "typed-operation emitted a store lane absent from its exact factory publication contract" fault.

