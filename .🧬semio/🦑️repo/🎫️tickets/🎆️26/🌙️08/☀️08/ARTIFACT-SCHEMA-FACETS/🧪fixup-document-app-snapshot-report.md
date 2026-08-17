# 🧪 Fixup Report — DocumentApp Projection → Snapshot (`semio-framework-plugin`)

## Classification counts (288 lines with `projection`/`Projection`)

| Bucket | Count | Notes |
|---|---:|---|
| **Renamed** (document-state) | **222** | Persist/snapshot/DocumentApp/store/test-double names |
| **Deliberately kept** (3D camera / other) | **66** | WorldProjection region + camera tests + media phrase + WindowKindDefinition field boundary |
| **Remaining after edit** | **68** | 66 kept + 2 `document_projection_schema` boundary lines that still contain the Definition field name |

### Judgment: `projection_spec_json_projects_only_active_kind_fields`

**Camera (kept).** The test builds a `WorldProjectionConfig`, calls `world3d_projection_spec_json`, and asserts only the active kind's fields appear in the camera projection JSON. Unrelated to document Snapshot.

## Symbols renamed (document-state)

Associated types / traits:
- `DocumentApp::Projection` → `DocumentApp::Snapshot`
- `type Projection = …` → `type Snapshot = …`
- `$Projection` metavariable in `app_commands!` → `$Snapshot`
- `A::Projection` / `Self::Projection` → `…::Snapshot`

Store / API methods:
- `.projection()` / `fn projection` → `.snapshot()` / `fn snapshot`
- `.projection_with_conflicts()` → `.snapshot_with_conflicts()`
- `initial_projection` → `initial_snapshot`
- `test_projection` → `test_snapshot`

View fields / locals:
- `DocumentView` / `ConfigView` / `DraftView` field `projection` → `snapshot`
- `doc.projection` / `cfg.projection` / draft locals → `snapshot`
- `draft_projection` → `draft_snapshot`
- `projection_override_json` / `override_projection` → `snapshot_override_json` / `override_snapshot`
- `projection_a` / `projection_b` → `snapshot_a` / `snapshot_b`
- WindowKindSpec field `document_projection_schema` → `document_snapshot_schema` (mapped to/from still-named `WindowKindDefinition::document_projection_schema` at the framework-manifest boundary)

Test doubles:
- `DummyProjection` → `DummySnapshot`
- `TestProjection` → `TestSnapshot`
- `revert_to_command_restores_the_projection_and_appends_one_entry` → `…_snapshot_…`

Docstring/English document-state wording updated in parallel (`projection` → `snapshot` where it named the document type). Export-media docstring reworded to “Pure export …” to avoid the overloaded noun.

## Deliberately kept (camera / non-document)

- `WorldProjectionConfig`, `WorldProjectionSpec`, region `WorldProjection`
- `world3d_projection_spec_json`, `world3d_camera_projection_json`, `world3d_projection_pose`, `world3d_projection_measures`
- `apply_world3d_projection_action`, `world3d_projection_action_moves_pose`
- `setProjection`, `setProjectionParam` (TS-facing action ids)
- `Oblique_projection`, `Axonometric_projection` (Wikipedia links)
- `computeWorldProjectionPose` (comment reference)
- JSON key `"projection"` inside `world3d_camera_projection_json`
- UI measure ids `{id_prefix}-projection-…` and measure group label `"Projection"`
- Tests `projection_spec_json_projects_only_active_kind_fields`, `projection_measures_tree_matches_the_requested_taxonomy`
- English `MediaType` projection phrase (media typing, not document Snapshot)
- `WindowKindDefinition::document_projection_schema` field name on the **manifest** type (out of this crate’s rename scope); SDK `WindowKindSpec` uses `document_snapshot_schema` and maps at the boundary

## `ArtifactEngine::Artifact`

**None in this crate.** `semio-framework-plugin` has no `impl ArtifactEngine`. Test doubles are `DocumentApp` apps (`DummySnapshot` / `TestSnapshot`), not artifact engines. P6 handcrafted `DocumentDsl`/`DocumentPack`/`OpText`/`OpBinary` were added for those SDK test doubles so `cargo test --lib` compiles after the derive stopped emitting those traits.

Real plugin engines must keep `Artifact` and `Snapshot` distinct; coincidence is only acceptable for SDK test doubles (stated here; no engine impl to annotate in-tree).

## Files edited

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (primary)
- Ticket artifacts under `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️08/ARTIFACT-SCHEMA-FACETS/` (this report + classification/logs)

No edits under `✏️s/🔌️plugins/**`.

## Gate tails (verbatim)

### 1. `cargo check -p semio-framework-plugin`

```
warning: `semio-framework-plugin` (lib) generated 15 warnings (run `cargo fix --lib -p semio-framework-plugin` to apply 15 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.89s
```

**PASS**

### 2. `cargo test -p semio-framework-plugin --lib`

```
test result: ok. 138 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

**PASS**

### 3. `cargo check -p semio-framework-os-kernel`

```
warning: `semio-framework-os-kernel` (lib) generated 45 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 22 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.18s
```

**PASS**

### 4. `cargo check -p semio-s-plugin-lowpoly`

```
Some errors have detailed explanations: E0046, E0407, E0425, E0432, E0437, E0559, E0609.
For more information about an error, try `rustc --explain E0046`.
warning: `semio-s-plugin-lowpoly` (lib) generated 4 warnings
error: could not compile `semio-s-plugin-lowpoly` (lib) due to 97 previous errors; 4 warnings emitted
```

**FAIL (expected for this wave)** — all 97 errors are inside `✏️s/🔌️plugins/💠️lowpoly/**` (fan-out owned). Zero errors on shared framework surfaces after this fixup.

### Lowpoly remaining blockers split

**Inside `✏️s/🔌️plugins/💠️lowpoly/**` (leave for fan-out):**
- `doc.projection` / `cfg.projection` → need `snapshot` (DocumentView/ConfigView fields) — ~80 E0609
- `DocumentApp` still declaring `type Projection` / `initial_projection` — E0437/E0407/E0046
- `LowpolyMutation::SetSnapshot { projection }` field rename to `snapshot` — E0559
- Missing/unresolved `LowpolyDiff` import paths under `crate::artifacts::lowpoly::diff` — E0432/E0425

**Shared framework surface (ours):** none remaining for this gate.
