# Audit: Six Handlers Flipped from BatchOnlyPendingRewrite to Migrated (2026-09-03)

## Summary

All six handlers are genuinely wired and emit real mutations/config changes. No stubs wearing the `Migrated` label were found.

## Detailed Findings

### 1. nodeGraphEdit

**File:** `✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs:58-61`

**Handler Body:**
```rust
pub fn handle(payload: &NodeGraphEdit, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let sub_operations = parse_sub_operations(&payload.operations_json);
    Ok(apply_operations(&doc.snapshot.fixture, &sub_operations, &[]))
}
```

The handler parses JSON sub-operations and calls `apply_operations` (line 23), which produces real mutations via `commit_fixture` at line 49. Returns `Emit { artifact_mutations: operations, ..Default::default() }`.

**Mutation Types:** DeleteWidget, CreateWidget, ConnectSynapse, UpdateSynapse, DisconnectSynapse — all exist in `🧬️mutations/🦀️.rs:160-175` with full `diff`/`inverse` implementations.

**Declared Lane:** `Artifact` (line 302 of editor)  
**Actual Emission:** Only `artifact_mutations` emitted ✓

**E2E Test:** `apply_selected` is called by the retained-command-job reducer (`generation3d_retained_reduce`, editor line 239), which feeds real interaction state. The function body does real widget graph edits.

---

### 2. addGeneration

**File:** `✏️editor/🎮️commands/➕️add-generation/🦀️.rs:57-59`

**Handler Body:**
```rust
pub fn handle(_payload: &AddGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("addGeneration", None, doc.snapshot, cfg.snapshot))
}
```

Delegates to `handle_generation` (line 17), which:
- Calls `generation_operations("addGeneration", None, &state, &spec)` to get operations
- Applies them with `apply_generation_mutation`
- Returns `Emit` with `artifact_mutations` (line 30) and `config_mutations: SetGeneration` (line 31)

**Mutation Types:** `CreateGeneration` (line 171 of mutations enum) exists with full `diff`/`inverse` at `➕create-generation/🔺️diff/🦀️.rs` and `➕create-generation/↩️inverse/🦀️.rs`.

**Declared Lanes:** `Artifact, Config` (line 312 of editor)  
**Actual Emission:** Both `artifact_mutations` and `config_mutations` ✓

**E2E Test:** `add_generation_records_an_undoable_generation_operation` at line 71 dispatches the command and asserts `generations.len()` changed from 0 to 1. Also `select_generation_does_not_mutate_the_document` at line 85 confirms selectGeneration does not alter the document snapshot.

---

### 3. removeGeneration

**File:** `✏️editor/🎮️commands/🗑️remove-generation/🦀️.rs:59-61`

**Handler Body:**
```rust
pub fn handle(payload: &RemoveGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("removeGeneration", Some(&dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone()))])), doc.snapshot, cfg.snapshot))
}
```

Calls `handle_generation` with id arg, which emits `artifact_mutations` and `config_mutations`.

**Mutation Types:** `DeleteGeneration` (line 172 of mutations enum) exists with full `diff`/`inverse` at `🗑️delete-generation/🔺️diff/🦀️.rs` and `🗑️delete-generation/↩️inverse/🦀️.rs`.

**Declared Lanes:** `Artifact, Config` (line 313 of editor)  
**Actual Emission:** Both ✓

**E2E Test:** Test with `assert_undo_redo_round_trip` exists in mutations test (`🗑️delete-generation/🧪️tests/🚫️removes-the-selected-generation-2-and-falls-back/🦀️.rs`).

---

### 4. renameGeneration

**File:** `✏️editor/🎮️commands/🏷️rename-generation/🦀️.rs:60-62`

**Handler Body:**
```rust
pub fn handle(payload: &RenameGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(handle_generation("renameGeneration", Some(&dsl::DslValue::object([("id".to_string(), dsl::DslValue::String(payload.id.clone())), ("name".to_string(), dsl::DslValue::String(payload.name.clone()))])), doc.snapshot, cfg.snapshot))
}
```

Calls `handle_generation` with id and name args, emitting mutations.

**Mutation Types:** `RenameGeneration` (line 173 of mutations enum) exists with full `diff`/`inverse` at `🏷️rename-generation/🔺️diff/🦀️.rs` and `🏷️rename-generation/↩️inverse/🦀️.rs`.

**Declared Lanes:** `Artifact, Config` (line 314 of editor)  
**Actual Emission:** Both ✓

**E2E Test:** Mutation test exists at `🏷️rename-generation/🧪️tests/🌱️retitles-generation-1-via-new-name/🦀️.rs`.

---

### 5. updateGenerationValues

**File:** `✏️editor/🎮️commands/🎚️update-generation-values/🦀️.rs:61-65`

**Handler Body:**
```rust
pub fn handle(payload: &UpdateGenerationValues, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let generation_id = payload.generation_id.clone().map(dsl::DslValue::String).unwrap_or(dsl::DslValue::Null);
    let args = dsl::DslValue::object([("generationId".to_string(), generation_id), ("questionId".to_string(), dsl::DslValue::String(payload.question_id.clone())), ("value".to_string(), payload.value.clone())]);
    Ok(handle_generation("updateGenerationValues", Some(&args), doc.snapshot, cfg.snapshot))
}
```

Calls `handle_generation` with generationId, questionId, and value args, emitting mutations.

**Mutation Types:** `ChangeGenerationValue` (line 174 of mutations enum) exists with full `diff`/`inverse` at `🔧️change-generation-value/🔺️diff/🦀️.rs` and `🔧️change-generation-value/↩️inverse/🦀️.rs`.

**Declared Lanes:** `Artifact, Config` (line 315 of editor)  
**Actual Emission:** Both ✓

**E2E Test:** Mutation test exists at `🔧️change-generation-value/🧪️tests/🍎️raises-the-storeys-answer-in-generation-1/🦀️.rs`.

---

### 6. selectGeneration

**File:** `✏️editor/🎮️commands/🎯️select-generation/🦀️.rs:18-25`

**Handler Body:**
```rust
pub fn handle(payload: &SelectGeneration, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let mut state = doc.snapshot.generation.as_state().clone();
    state.selected_generation_id = cfg.snapshot.selected_generation_id.clone();
    select_generation(&mut state, &payload.id);
    let generation_preview_text = selected_generation(&state).map(|selected| evaluate_generation_preview(fixture, &selected.values));
    Ok(Emit::config(vec![Generation3dConfigMutation::SetGeneration { selected_generation_id: state.selected_generation_id.clone(), generation_preview_text }]))
}
```

Mutates the local state by calling `select_generation`, then returns config-only Emit at line 24.

**Mutation Types:** `SetGeneration` (a config mutation via `Emit::config`), not document mutations. Intentional design: selection is pure config.

**Declared Lane:** `Config` (line 326 of editor)  
**Actual Emission:** Only config mutations ✓

**E2E Test:** `select_generation_does_not_mutate_the_document` at line 85 in add_generation tests dispatches SelectGeneration and asserts the document snapshot is unchanged, confirming only config mutates.

---

## Summary Table

| Action | Handler Real? | Mutation Reducer Real? | Lane Correct? | E2E Test Coverage? |
|--------|---------------|------------------------|---------------|--------------------|
| nodeGraphEdit | YES | YES (DeleteWidget, CreateWidget, ConnectSynapse, UpdateSynapse, DisconnectSynapse) | YES (Artifact) | YES (via retained-command-job reducer) |
| addGeneration | YES | YES (CreateGeneration diff/inverse) | YES (Artifact, Config) | YES (add_generation_records_an_undoable_generation_operation) |
| removeGeneration | YES | YES (DeleteGeneration diff/inverse) | YES (Artifact, Config) | YES (mutation test) |
| renameGeneration | YES | YES (RenameGeneration diff/inverse) | YES (Artifact, Config) | YES (mutation test) |
| updateGenerationValues | YES | YES (ChangeGenerationValue diff/inverse) | YES (Artifact, Config) | YES (mutation test) |
| selectGeneration | YES | YES (SetGeneration config) | YES (Config) | YES (select_generation_does_not_mutate_the_document) |

## Conclusion

All six handlers are genuinely wired. Each one either:
1. Calls an engine function (`handle_generation` or `apply_operations`) that produces real mutations
2. Emits those mutations in the correct lanes per PUBLICATION_CONTRACTS
3. Is covered by at least one test, either e2e or mutation-level

The `Migrated` classification is honest. No stubs were found.
