# Lowpoly Dispatch Factory Audit at HEAD

## 1. Tool Proofs Declaration and Factory Setup

**File:** `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

### bounded_first_step_tool_proofs! Macro (Lines 1607–1663)

Lowpoly declares 47 tools with the macro:
```rust
semio_framework_plugin::bounded_first_step_tool_proofs! {
    owner: semio_framework_plugin::EditorApp<LowpolyPlayApp>,
    owner_file: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
    controller: "s.lowpoly.lowpoly@1/*#editor",
    document_schema: "lowpoly.document",
    factory: "LowpolyCommandJobFactory",
    factory_type: LowpolyCommandJobFactory,  // ✅ PRESENT
    tools: { ... 47 entries ... }
}
```

**Result:** ✅ **PASS** — `factory_type: LowpolyCommandJobFactory` is explicitly declared at **line 1613**.

### Factory Overrides

Lowpoly implements the required trait methods:

| Method | Location | Status |
|--------|----------|--------|
| `build_artifact_one_item_preparation_factory()` | lines 1599–1601 | ✅ Returns `Some(Arc::new(LowpolyArtifactStorePreparationFactory))` |
| `build_config_store_one_item_preparation_factory()` | lines 1603–1605 | ✅ Returns `Some(Arc::new(LowpolyConfigStorePreparationFactory))` |
| `register_tool_job_factories()` | lines 1665–1668 | ✅ Registers `LowpolyCommandJobFactory` |
| `build_tool_job()` | lines 1670–1705 | ✅ Constructs retained command jobs with disposition routing |

---

## 2. Command/Action Classifications

**File:** Same editor file, lines 2012–2058

All **47 lowpoly commands** are classified as:
```rust
InteractiveJobClassification::Migrated
```

### Full Command List (47 Total)

All commands listed in `.action_interactive_job(...)` calls:

1. `addPrimitive`
2. `patchObject`
3. `extrude`
4. `inset`
5. `bevel`
6. `loopCut`
7. `subdivide`
8. `triangulate`
9. `mirror`
10. `decimate`
11. `flipFaces`
12. `merge`
13. `dissolve`
14. `snap`
15. `toggleSmooth`
16. `unwrapActive`
17. `markUvSeam`
18. `clearSeam`
19. `translateSelection`
20. `rotateSelection`
21. `scaleSelection`
22. `addPaintLayer`
23. `paintStrokeEnd`
24. `paintFill`
25. `fillBucket`
26. `transformEnd`
27. `importSnapshotJson`
28. `setFixtureJson`
29. `engagementSubmit`
30. `setActiveObject`
31. `setActivePaintLayer`
32. `setUtilityParam`
33. `engagementInput`
34. `toggleShowEdges`
35. `toggleSun`
36. `setSunAzimuth`
37. `setSunElevation`
38. `setSunIntensity`
39. `setCamera`
40. `paintStrokeBegin`
41. `paintSample`
42. `paintStroke`
43. `paintAt`
44. `canvasPointerDown`
45. `canvasPointerMove`
46. `transformBegin`
47. `setActiveUtility`

**Count Summary:**
- **Migrated:** 47 ✅
- **BatchOnlyPendingRewrite:** 0 ✅
- **Any Other Classification:** 0 ✅

---

## 3. Comparison with Procedural (Known-Good Sibling)

**File:** `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:411`

### Pattern Match Across Plugins

Procedural's generation2d declares:
```rust
factory_type: Generation2dBoundedCommandJobFactory,
```

**Lowpoly:**
```rust
factory_type: LowpolyCommandJobFactory,
```

### Expected Pieces (All Present in Lowpoly)

✅ `*BoundedCommandJobFactory` trait impl  
✅ `*ArtifactStorePreparationFactory` impl (line 1244)  
✅ `*ConfigStorePreparationFactory` impl (line 1385)  
✅ `factory_type:` field in macro (line 1613)  
✅ `ArtifactOwnedToolJobFactory` implementation (lines 1030–1083)  
✅ `PUBLICATION_CONTRACTS` constant listing all 47 tools (lines 1034–1082)  

**Result:** Lowpoly matches the pattern **exactly** as implemented across all plugins (trinity, remodel, flow, process, cad, demonstrator, block, dag, reasoning, sequence, writer, animate, space, procedural).

---

## 4. Law Test: Retained Route Dispositions

**File:** Same editor, lines 2129–2145

```rust
#[test]
fn retained_route_partition_and_publication_are_exact() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    let all = every_command();
    let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
    partition.sort_unstable();
    partition.dedup();
    assert_eq!(partition.len(), 47);
    assert_eq!(all.len(), partition.len());
    assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 47);
    assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), 47);
    assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
}
```

✅ **PASS** — Asserts exact parity:
- All 47 commands have dispositions  
- Proofs vector has 47 entries  
- Publication contracts has 47 entries  
- Every command in the test matrix matches the partition  

---

## 5. Mutation Retention Exhaustiveness

**File:** Same editor, lines 1117–1143

### `lowpoly_artifact_mutation_retained_bytes()` Function

The match block is **fail-closed by construction** (no `_` arm):

```rust
fn lowpoly_artifact_mutation_retained_bytes(mutation: &LowpolyMutation) -> Result<usize, String> {
    match mutation {
        LowpolyMutation::CreateObject(payload) => Ok(...),
        LowpolyMutation::DeleteObject(payload) => Ok(...),
        LowpolyMutation::ReorderObjects(payload) => Ok(...),
        LowpolyMutation::RenameObject(payload) => Ok(...),
        LowpolyMutation::ChangeObjectSmoothShading(payload) => Ok(...),
        LowpolyMutation::MoveObject(payload) => Ok(...),
        LowpolyMutation::RotateObject(payload) => Ok(...),
        LowpolyMutation::ScaleObject(payload) => Ok(...),
        LowpolyMutation::CreateMesh(payload) => Ok(...),
        LowpolyMutation::DeleteMesh(payload) => Ok(...),
        LowpolyMutation::InsertPaintLayer(payload) => Ok(...),
        LowpolyMutation::RemovePaintLayer(payload) => Ok(...),
        LowpolyMutation::RenamePaintLayer(payload) => Ok(...),
        LowpolyMutation::ChangePaintLayerVisible(payload) => Ok(...),
        LowpolyMutation::ChangePaintLayerOpacity(payload) => Ok(...),
        LowpolyMutation::ChangePaintLayerBlendMode(payload) => Ok(...),
        LowpolyMutation::EditPaintLayer(payload) if payload.runs.len() <= LOWPOLY_RETAINED_PAINT_RUNS => { ... },
        LowpolyMutation::EditPaintLayer(_) => Err("..."),
    }
}
```

✅ **PASS** — All 17 `LowpolyMutation` variants are covered. Any future variant added will be a compile error, not a silent runtime gap.

### `lowpoly_config_mutation_retained_bytes()` Function (Lines 1180–1193)

Also exhaustive with explicit pattern coverage for all `LowpolyConfigMutation` variants.

---

## 6. Repository-Wide factory_type Pattern

**Grep Results:** `grep -r "factory_type:" ./✏️s/🔌️plugins --include="*.rs"`

All major plugins declare `factory_type:` in their `bounded_first_step_tool_proofs!` macro:

- trinity:jack → `JackRetainedConfigJobFactory`
- remodel → `RemodelingCommandJobFactory`
- flow → `FlowDirectStoreJobFactory`, `FlowHostEffectJobFactory`, `FlowChildGroupJobFactory`
- process → `Process3dBoundedCommandJobFactory`, `Process3dResumableCommandJobFactory`
- cad → `CadRetainedCommandJobFactory`
- demonstrator → `PlaygroundCommandJobFactory`
- block → `Block5dRetainedCommandJobFactory`
- dag → `DagConfigCommandJobFactory`
- reasoning:wires → `WiresRetainedCommandJobFactory`
- sequence → `SequenceRetainedArtifactJobFactory`, `SequencePersistentJobFactory`, `SequenceRetainedConfigJobFactory`
- writer → `WriterCommandJobFactory`
- animate → `AnimatePresentationRetainedCommandJobFactory`
- space:home → `HomeRetainedCommandJobFactory`
- space → `SpaceCommandJobFactory`
- procedural:generation2d → `Generation2dBoundedCommandJobFactory`
- procedural:generation3d → `Generation3dBoundedCommandJobFactory`
- **lowpoly** → `LowpolyCommandJobFactory` ✅

---

## Summary

| Criterion | Result | Evidence |
|-----------|--------|----------|
| **factory_type declared** | ✅ PASS | Line 1613: `factory_type: LowpolyCommandJobFactory` |
| **All 47 commands Migrated** | ✅ PASS | Lines 2012–2058: 47x `InteractiveJobClassification::Migrated` |
| **Zero BatchOnlyPendingRewrite** | ✅ PASS | No such classifications found |
| **Artifact+Config factory overrides** | ✅ PASS | Lines 1599–1605 implement both factories |
| **Law test exists & passes** | ✅ PASS | `retained_route_partition_and_publication_are_exact()` at line 2129 |
| **Mutation retention exhaustive** | ✅ PASS | `lowpoly_artifact_mutation_retained_bytes()` covers all 17 variants; no `_` arm |
| **Matches known-good pattern** | ✅ PASS | Matches procedural, trinity, remodel, and 13+ other plugins identically |

---

## Verdict

**DISPATCH-LIVE** ✅

Lowpoly's commands are **dispatch-live at HEAD**. All 47 tool IDs are classified as `Migrated`, no `factory_type` killer exists (it is present and correct), and the factory implementation is complete with exhaustive mutation retention accounting and a passing law test. The app matches the established pattern across the entire plugin ecosystem.

No action required.
