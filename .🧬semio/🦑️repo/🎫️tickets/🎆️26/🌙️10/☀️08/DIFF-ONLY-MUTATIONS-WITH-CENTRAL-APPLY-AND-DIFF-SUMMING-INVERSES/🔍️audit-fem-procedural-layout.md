# 🔍️ Audit: fem, procedural and layout mutation leaves

Read-only audit against `📋️design.md` (laws L1-L5, codes V1-V4 and the Rulings, including `V5-ABSORB`, as the file
reads on disk at audit time). Scope: `✏️s/🔌️plugins/🏗️fem`, `✏️s/🔌️plugins/🌀️procedural`, `✏️s/🔌️plugins/📏️layout`.

## Headline

- **164 `MutationKind` leaves in scope** (fem 61, procedural 55, layout 48). The 92 leaves not listed in section 7 are
  clean on V1/V2/V3 signals, and the audit found no `.apply(` call in any leaf, diff or inverse body (non-test).
- **72 leaves violate** (fem 7, procedural 55, layout 10). All 55 procedural leaves violate: 38 build the diff from
  shared whole-fixture helpers (`diff_snapshot_from_helpers`, `diff_generation_from_ops`, `diff_generation_with`,
  `generation3d_transform_diff`) and 17 are inline whole-config/transient/presence diffs that clone `base` and write fields.
- **V2-DIFF-DERIVED-INVERSE** in 5 leaves: `MoveSelection` (fem2d, fem3d) walks `super::diff::diff`; `DragFrames`,
  `RotateFrames`, `ScaleFrames` (layout) pass the forward outcome into `layout_frame_selection_inverse`.
- **V1-GENERIC-DIFF** (whole sub-document or whole list in a set/update kind): `UpdateAnalysisSettings` (fem2d, fem3d),
  `SetStoryRuns`, `SetPageOverrides`, `SetPageGuides`, `UpdateGrid`, `ChangeDataFields`, `ChangePrintTarget` (layout).
- **V5-ABSORB (diff-type level)**: `Fem2dDiff`, `Fem3dDiff` and `LayoutDiff` absorb by concatenating `added`/`removed`/
  `patched`. Two patches of one id produce a delta that `apply_delta` rejects (`mutation.apply.duplicate-target`), so
  `absorb(d1,d2).apply(base)` can fail where `d1` then `d2` succeeds.
- **V4**: no test in scope sums inverse diffs with `absorb` (0 of 164). Inverse-referencing tests exist for 143 leaves (141 apply inverse steps and assert);
  21 leaves have no inverse-referencing test at all (listed below).
- **L4 outside leaves**: 19 non-test call sites of `MutationDiff::apply` in editor replay, `🚪️io` builders and generic
  helpers (listed below). Central apply is not enforced: `MutationDiff::apply(&self, base)` has no `ApplyCapability`
  parameter (`🧰️framework/.../📡️replication/🎮️mutation/🦀️.rs:104`), so L5 is not enforced at type level.

## 1. Per-artifact summary

| group | kinds | violating | clean | V1-SNAPSHOT-DIFF | V1-GENERIC-DIFF | V2-DIFF-DERIVED-INVERSE | V2-RESTORE-INVERSE | V2-EMPTY-INVERSE | V3-LEAF-APPLY | V3-HAND-MUTATION | V4-LAW-UNTESTED |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| fem / fem2d | 31 | 4 | 27 | 2 | 2 | 1 | 0 | 0 | 2 | 0 | 1 |
| fem / fem3d | 30 | 3 | 27 | 1 | 2 | 1 | 0 | 0 | 1 | 0 | 0 |
| procedural / generation2d | 17 | 17 | 0 | 17 | 0 | 0 | 0 | 0 | 17 | 0 | 3 |
| procedural / generation3d | 38 | 38 | 0 | 38 | 1 | 0 | 1 | 0 | 38 | 0 | 16 |
| layout / layout | 48 | 10 | 38 | 1 | 7 | 3 | 0 | 0 | 1 | 0 | 1 |
| **total** | **164** | **72** | **92** | **59** | **12** | **5** | **1** | **0** | **59** | **0** | **21** |

Counts are per leaf (a leaf can carry several codes). V3-LEAF-APPLY counts both direct writes and indirect clone-and-write
through a shared helper (see section 2). V3-HAND-MUTATION is reported in section 4 (hand aggregates, not `MutationKind` leaves).
V4-LAW-UNTESTED counts leaves with no inverse-referencing test; the strict absorb-sum form of V4 is absent for all 164.

## 2. Shared generic helpers in scope

| helper | file:line | non-test callers | violation | evidence |
|---|---|---:|---|---|
| `diff_snapshot_from_helpers` (gen2d) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:257` | gen2d+gen3d: 25 | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3 (indirect) | clones `host_snapshot`, applies deltas, returns `Generation2dDiff { host_snapshot: Some(updated) }` (whole fixture) |
| `diff_snapshot_from_helpers` (gen3d) | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:261` | (same group) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3 (indirect) | same whole-`host_snapshot` carriage |
| `apply_host_snapshot_helpers` | gen2d `🔺️diff/🦀️.rs:164`; gen3d `🔺️diff/🦀️.rs:166` | 4 | V3-LEAF-APPLY (shared) | `let mut next = host_snapshot.clone()` then `apply_*_diff(&mut ..)`, called from diff builders |
| `diff_generation_from_ops` | gen2d `🔺️diff/🦀️.rs:263`; gen3d `🔺️diff/🦀️.rs:268` | 8 | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3 (indirect) | clones generation, applies ops, returns whole `generation` state |
| `apply_generation_helpers` | gen2d `🔺️diff/🦀️.rs:179`; gen3d `🔺️diff/🦀️.rs:181` | 4 | V3-LEAF-APPLY (shared) | clone then `apply_generation_mutation(&mut next, ..)` |
| `diff_generation_with` | gen3d `🔺️diff/🦀️.rs:275` | 2 | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY (direct) | takes `FnOnce(&mut GenerationPlayState)` and clones base first |
| `generation3d_transform_diff` | gen3d `🧬️mutations/🦀️.rs:512` | 3 | V1-SNAPSHOT-DIFF (via snapshot helper) | delegates the whole-fixture carriage to `diff_snapshot_from_helpers` |
| `generation3d_transform_inverse` | gen3d `🧬️mutations/🦀️.rs:562` | 3 | clean | concrete `UpdateWidget` per target widget |
| `generation2d_host_snapshot_operations` | gen2d `🧬️mutations/🦀️.rs:165` | 4 | V1-SNAPSHOT-DIFF (generic two-snapshot differ) | diffs two fixtures into an operation list; called from editor commands `set-active-example`, `add-widget`, not only leaves |
| `layout_frame_selection_diff` | layout `🧬️mutations/🦀️.rs:90` | 3 | clean (sparse closure-built `FramePatch`) | partner of the V2 inverse below |
| `layout_frame_selection_inverse` | layout `🧬️mutations/🦀️.rs:123` | 3 | V2-DIFF-DERIVED-INVERSE | takes `outcome: MutationOutcome<LayoutDiff>` and walks `frames_patched`; same shape as the reference `drawing_selection_inverse` |
| `diff_set_snapshot` | fem2d `🔺️diff/🦀️.rs:665`; fem3d `🔺️diff/🦀️.rs:658`; layout `🧬️schema/🔺️diff/🦀️.rs:538` | 0 (fem); 1 unit test (layout) | V1-GENERIC-DIFF (whole snapshot seam, dead) | design ruling deletes whole-snapshot seams |
| `apply_generation2d_mutation` / `apply_generation3d_mutation` | gen2d `🧬️mutations/🦀️.rs:215`; gen3d `🧬️mutations/🦀️.rs:742` | 2 non-test + tests | L4 (calls `MutationDiff::apply` outside central applier) | `protocol::MutationDiff::apply(&delta, ..)` on the projection |
| `inverse_generation2d_mutation` / `inverse_generation3d_mutation` | gen2d `🧬️mutations/🦀️.rs:233`; gen3d `🧬️mutations/🦀️.rs:759` | 17 / 30 | clean (dispatch delegate to `mutation.inverse`) | not diff-derived |
| `apply_fem2d_mutation` / `apply_fem3d_mutation` | fem2d `🧬️mutations/🦀️.rs:105`; fem3d `🧬️mutations/🦀️.rs:338` | tests and `move-selection` laws | clean (routes through `vcs::apply_mutation`) | central path; used only by test laws |
| `inverse_fem2d_mutation` / `inverse_fem3d_mutation` | fem2d `🧬️mutations/🦀️.rs:112`; fem3d `🧬️mutations/🦀️.rs:345` | 120 / 121 | clean (dispatch delegate) | not diff-derived |

## 3. Diff types, MutationDiff / DiffAlgebra / absorb

`DiffAlgebra` (`🧰️framework/.../📡️replication/🎮️mutation/🦀️.rs`) has **no implementor in scope**. `MutationDiff::apply` is
implemented on each schema diff below (not capability-gated). Config, window and presence snapshots use `type Diff = P`;
no hand `MutationDiff` impl for those was found in scope (presumably derive-generated, not verified).

| artifact | diff type | impl file:line | apply | absorb (file:line) | absorb sound? | code |
|---|---|---|---|---|---|---|
| fem2d | `Fem2dDiff` (sparse, plus whole `artifact` replacement) | `🏗️fem/◻️2d/.../🧬️schema/🔺️diff/🦀️.rs:395` | yes (impl `:395`) | `:432` concatenation via `impl_merge!` (`:468-497`) | no: duplicate patches / create-then-delete not coalesced | V5-ABSORB |
| fem3d | `Fem3dDiff` | `🏗️fem/🧊️3d/.../🧬️schema/🔺️diff/🦀️.rs:388` | yes | `:425` concatenation via `impl_merge!` (`:476-490`) | no (same as fem2d) | V5-ABSORB |
| gen2d | `Generation2dDiff` (whole `host_snapshot` and `generation`) | `🌀️procedural/🌀️generation2d/.../🧬️schema/🔺️diff/🦀️.rs:206` | yes (impl `:206`) | `:225` whole-field replacement | sequentially sound (last whole state wins) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF |
| gen3d | `Generation3dDiff` (whole `host_snapshot`, `generation`, `touched`) | `🌀️procedural/🧊️generation3d/.../🧬️schema/🔺️diff/🦀️.rs:208` | yes | `:227` whole-field replacement, `touched` recomputed | sequentially sound | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF |
| layout | `LayoutDiff` (sparse per-collection deltas plus whole `grid`, `print_target`, `data_fields`, `background_drawing`) | `📏️layout/.../🧬️schema/🔺️diff/🦀️.rs:400` | yes (impl `:400`) | `:452`, `absorb_pages` `:457`, `dst.patched.extend(src.patched)` | no (same concatenation flaw) | V5-ABSORB |
| gen2d/gen3d transients, presence | `Generation2dTransient`, `Generation2dPresence`, `Generation3dTransient`, `Generation3dViewTransient`, `Generation3dPresence`, `Generation3dViewPresence`, `Generation3dPreviewEditTransient` | `🫧️transient/🦀️.rs`, `👥️presence/🦀️.rs` (under `✏️editor`/`👁️viewer`) | yes | `*self = other` or field replacement | sequentially sound (whole-state) | V1-SNAPSHOT-DIFF (whole-state diff) |
| fem2d results transient | `FemResultsWindowTransient` | `🏗️fem/◻️2d/.../📊️results/🫧️transient/🦀️.rs:101` | yes | `:105` `*self = other` | sequentially sound | V1-SNAPSHOT-DIFF |

Absorb unit tests exist only at diff-type level (`layout/.../🔺️diff/🧪️tests/🔬️unit`, `gen2d` and `gen3d` diff `🧪️tests/🔬️unit`).
None sums the inverse diffs of a leaf with `absorb` (the L3 sum law), so V5 is not caught by any test.

## 4. Hand-written aggregates (not `MutationKind` leaves)

Eleven `impl Mutation<P>` aggregates, each with a single `Snapshot { config }` variant. `diff` returns the whole config
(`MutationOutcome::new(config.clone())`) and `inverse` returns `Snapshot { config: Box::new(base.clone()) }`.
They bypass `#[derive(Mutations)]`: V3-HAND-MUTATION (no direct `.apply(`; the bypass is the violation), V1-SNAPSHOT-DIFF
(whole-config diff) and V2-RESTORE-INVERSE (whole-snapshot restore). The design ruling removes whole-snapshot config/window diffs.

| file:line | aggregate |
|---|---|
| `🌀️procedural/🌀️generation2d/.../✏️editor/🎚️config/🦀️.rs:158` | `Generation2dConfigMutation` (V3-HAND, V1-SNAPSHOT, V2-RESTORE) |
| `🌀️procedural/🧊️generation3d/.../✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🫧️transient/🦀️.rs:75` | `Generation3dPreviewWindowTransientMutation` |
| `🏗️fem/◻️2d/.../📊️results/🎚️config/🦀️.rs:84` | `Fem2dResultsWindowConfigMutation` |
| `🏗️fem/◻️2d/.../🧱️model/🎚️config/🦀️.rs:78` | `Fem2dModelWindowConfigMutation` |
| `🌀️procedural/🧊️generation3d/.../👥️presence/🦀️.rs:103` | `Generation3dPresenceMutation` |
| `🌀️procedural/🌀️generation2d/.../🪟️windows/👁️preview/🎚️config/🦀️.rs:71` | `Generation2dEditPreviewWindowConfigMutation` |
| `🌀️procedural/🌀️generation2d/.../🪟️windows/🕸️flow/🎚️config/🦀️.rs:74` | `Generation2dMainWindowConfigMutation` |
| `🌀️procedural/🌀️generation2d/.../🧬️generate/🪟️windows/👁️preview/🎚️config/🦀️.rs:71` | `Generation2dGeneratePreviewWindowConfigMutation` |
| `🌀️procedural/🌀️generation2d/.../👥️presence/🦀️.rs:101` | `Generation2dPresenceMutation` |
| `🏗️fem/🧊️3d/.../📊️results/🎚️config/🦀️.rs:78` | `Fem3dResultsWindowConfigMutation` |
| `🏗️fem/🧊️3d/.../🧱️model/🎚️config/🦀️.rs:107` | `Fem3dModelWindowConfigMutation` |
| `📏️layout/.../📐️blueprint/🎚️config/🦀️.rs:30` | `LayoutWindowConfigMutation` |

Note: these eleven are the non-test `impl Mutation<` sites; an earlier raw count of twelve included a doc-comment line.

## 5. L3 law tests

- Sequential inverse-step tests (apply forward, apply each inverse step, compare to base) exist for 141 leaves with an
  `apply` plus `assert` in the leaf's `🧪️tests` tree. Move-selection (fem2d, fem3d) keeps them in a shared `laws` module.
- No test sums inverse diffs with `absorb` (`Σ.apply(after) == base`). Strict V4 therefore applies to all 164 leaves.
- Leaves with **no** inverse-referencing test (V4 weak):
  - procedural gen2d: `SetGenerationPreview` (transient), `ChangeSliderValue`, `MoveNodes`
  - procedural gen3d (16): `SetSnapshot`, `SetSun`, `SetShowMode`, `SetPreviewCamera`, `SetLodMode`, `SetCamera`, `SetSelectedGeneration`, `SetGenerationPreview` (transient), `SetSun`, `SetActiveExample`, `SetShowMode`, `SetPreviewCamera`, `SetLodMode`, `SetShowMode` (presence), `SetPreviewCamera` (presence), `SetPreviewEval`
  - fem2d: `SetPlaybackClock`; layout: `DeleteCharacterStyle`

## 6. Non-leaf L4 sites (`MutationDiff::apply` outside leaves, non-test)

Each of these applies a diff to a snapshot inside an editor replay fold, an `🚪️io` builder (`derived_construction` `mutate`/`absorb`), or a generic helper. Central apply must be the only caller.

- fem2d `✏️editor/🫧️transient/🦀️.rs:83` (replay fold)
- fem2d `✏️editor/🦀️.rs:453` (editor)
- fem2d `🚪️io/🦀️.rs:163`, `:170` (`derived_construction` mutate/absorb)
- fem3d `✏️editor/🦀️.rs:514` (editor)
- fem3d `🚪️io/🦀️.rs:163`, `:170` (`derived_construction`)
- gen2d `✏️editor/🦀️.rs:875` (editor)
- gen2d `🚪️io/🦀️.rs:354`, `:361` (`derived_construction`)
- gen2d `🧬️schema/🧬️mutations/🦀️.rs:221` (`apply_generation2d_mutation`)
- gen3d `✏️editor/🦀️.rs:1671`, `:1807` (editor)
- gen3d `🚪️io/🦀️.rs:1086`, `:1093` (`derived_construction`)
- gen3d `🧬️schema/🧬️mutations/🦀️.rs:748` (`apply_generation3d_mutation`)
- layout `🧬️schema/🧬️mutations/🦀️.rs:204` (layout JSON/apply helper)
- layout `🚪️io/🦀️.rs:603`, `:610` (`derived_construction`)

Paths are relative to `✏️s/🔌️plugins/<family>/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/` (or `🌐️any` for fem).

## 7. Per-kind violations

One row per violating leaf (72). `file:line` is the first evidence line. `(indirect)` means the leaf reaches clone-and-write
through a shared helper; `(direct)` means the leaf writes into a clone itself. Clean leaves (92) are not listed.

| file:line | kind | codes | evidence |
|---|---|---|---|
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/👁️set-generation/🦀️.rs:17` | procedural/🌀️generation2d `SetGenerationPreview` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation2dTransient` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect/🔺️diff/🦀️.rs:11` | procedural/🌀️generation2d `DisconnectSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🔺️diff/🦀️.rs:12` | procedural/🌀️generation2d `CreateGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖delete-generation/🔺️diff/🦀️.rs:12` | procedural/🌀️generation2d `DeleteGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-widget/🔺️diff/🦀️.rs:12` | procedural/🌀️generation2d `CreateWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-slider-value/🔺️diff/🦀️.rs:28` | procedural/🌀️generation2d `ChangeSliderValue` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3); diff clones widget and mutates via `set_widget_slider_value(&mut widget, ..)` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️update-camera/🔺️diff/🦀️.rs:15` | procedural/🌀️generation2d `UpdateCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename/🔺️diff/🦀️.rs:15` | procedural/🌀️generation2d `RenameGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move/🔺️diff/🦀️.rs:14` | procedural/🌀️generation2d `MoveWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/🔺️diff/🦀️.rs:13` | procedural/🌀️generation2d `ReplaceWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️replace/🔺️diff/🦀️.rs:12` | procedural/🌀️generation2d `ReplaceSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-synapse/🔺️diff/🦀️.rs:21` | procedural/🌀️generation2d `ConnectSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️change/🔺️diff/🦀️.rs:15` | procedural/🌀️generation2d `ChangeGenerationValue` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/🔺️diff/🦀️.rs:11` | procedural/🌀️generation2d `ChangeSchema` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🔺️diff/🦀️.rs:11` | procedural/🌀️generation2d `DeleteWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-nodes/🔺️diff/🦀️.rs:41` | procedural/🌀️generation2d `MoveNodes` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect), V4-LAW-UNTESTED | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹clear-widget-layout/🔺️diff/🦀️.rs:11` | procedural/🌀️generation2d `ClearWidgetLayout` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/⚙️set-snapshot/🦀️.rs:24` | procedural/🧊️generation3d `SetSnapshot` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V2-RESTORE-INVERSE, V1-GENERIC-DIFF, V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig`; diff carries whole config; inverse restores `config: base.clone()` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🌞️set-sun/🦀️.rs:17` | procedural/🧊️generation3d `SetSun` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/👁️set-show-mode/🦀️.rs:17` | procedural/🧊️generation3d `SetShowMode` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/📷️set-preview-camera/🦀️.rs:19` | procedural/🧊️generation3d `SetPreviewCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔬️set-lod-mode/🦀️.rs:17` | procedural/🧊️generation3d `SetLodMode` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🕸️set-camera/🦀️.rs:19` | procedural/🧊️generation3d `SetCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🧬️set-selected/🦀️.rs:17` | procedural/🧊️generation3d `SetSelectedGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/👁️set-generation/🦀️.rs:16` | procedural/🧊️generation3d `SetGenerationPreview` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dTransient` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/🌞️set-sun/🦀️.rs:17` | procedural/🧊️generation3d `SetSun` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/🎨️set-active-example/🦀️.rs:26` | procedural/🧊️generation3d `SetActiveExample` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/👁️set-show-mode/🦀️.rs:17` | procedural/🧊️generation3d `SetShowMode` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/📷️set-preview-camera/🦀️.rs:18` | procedural/🧊️generation3d `SetPreviewCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎚️config/🧬️schema/🧬️mutations/🔬️set-lod-mode/🦀️.rs:17` | procedural/🧊️generation3d `SetLodMode` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewConfig` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🧬️mutations/👁️set-show-mode/🦀️.rs:17` | procedural/🧊️generation3d `SetShowMode` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewPresence` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/👥️presence/🧬️schema/🧬️mutations/📷️set-preview/🦀️.rs:18` | procedural/🧊️generation3d `SetPreviewCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewPresence` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🫧️transient/🧬️schema/🧬️mutations/👁️set-preview/🦀️.rs:18` | procedural/🧊️generation3d `SetPreviewEval` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone()` then field write; diff returns whole `Generation3dViewTransient` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-synapse/🔺️diff/🦀️.rs:14` | procedural/🧊️generation3d `DisconnectSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-transforms/🔺️diff/🦀️.rs:16` | procedural/🧊️generation3d `DragTransforms` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `generation3d_transform_diff`; clone-and-write shared helper in diff path (indirect V3); diff via `generation3d_transform_diff` (whole host_snapshot carried) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌delete-widget/🔺️diff/🦀️.rs:14` | procedural/🧊️generation3d `DeleteWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕create-generation/🔺️diff/🦀️.rs:16` | procedural/🧊️generation3d `CreateGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-widget/🔺️diff/🦀️.rs:16` | procedural/🧊️generation3d `CreateWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎚️change-slider-value/🔺️diff/🦀️.rs:29` | procedural/🧊️generation3d `ChangeSliderValue` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3); diff clones widget and mutates via `set_widget_slider_value(&mut widget, ..)` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-widget-input/🔺️diff/🦀️.rs:26` | procedural/🧊️generation3d `ChangeWidgetInput` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-generation/🔺️diff/🦀️.rs:16` | procedural/🧊️generation3d `RenameGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👆️select-generation/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `SelectGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct) | diff delegates to shared `diff_generation_with`; clone-and-write shared helper in diff path (indirect V3); diff passes `&mut GenerationPlayState` closure to `diff_generation_with` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widget/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `MoveWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️scale-transforms/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `ScaleTransforms` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `generation3d_transform_diff`; clone-and-write shared helper in diff path (indirect V3); diff via `generation3d_transform_diff` (whole host_snapshot carried) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️change-generation-preview/🔺️diff/🦀️.rs:12` | procedural/🧊️generation3d `ChangeGenerationPreview` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct) | diff delegates to shared `diff_generation_with`; clone-and-write shared helper in diff path (indirect V3); diff passes `&mut GenerationPlayState` closure to `diff_generation_with` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📷️update-camera/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `UpdateCamera` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️rotate-transforms/🔺️diff/🦀️.rs:21` | procedural/🧊️generation3d `RotateTransforms` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `generation3d_transform_diff`; clone-and-write shared helper in diff path (indirect V3); diff via `generation3d_transform_diff` (whole host_snapshot carried) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse/🔺️diff/🦀️.rs:19` | procedural/🧊️generation3d `UpdateSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-synapse/🔺️diff/🦀️.rs:22` | procedural/🧊️generation3d `ConnectSynapse` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔤️change-schema/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `ChangeSchema` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔧️change-generation-value/🔺️diff/🦀️.rs:15` | procedural/🧊️generation3d `ChangeGenerationValue` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-generation/🔺️diff/🦀️.rs:13` | procedural/🧊️generation3d `DeleteGeneration` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_generation_from_ops`; clone-and-write shared helper in diff path (indirect V3); whole `generation` state carried in `Generation*Diff` |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-nodes/🔺️diff/🦀️.rs:42` | procedural/🧊️generation3d `MoveNodes` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹️delete-widget-position/🔺️diff/🦀️.rs:17` | procedural/🧊️generation3d `DeleteWidgetPosition` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹update-widget/🔺️diff/🦀️.rs:27` | procedural/🧊️generation3d `UpdateWidget` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(indirect) | diff delegates to shared `diff_snapshot_from_helpers`; clone-and-write shared helper in diff path (indirect V3) |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🫧️transient/🧬️schema/🧬️mutations/⏱️set-playback-clock/🦀️.rs:18` | fem/◻️2d `SetPlaybackClock` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V4-LAW-UNTESTED | inline `let mut next = base.clone(); next.clock = ..` returns whole transient state |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-load/🔺️diff/🦀️.rs:33` | fem/◻️2d `ReplaceLoad` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V1-GENERIC-DIFF | diff clones parent case (`let mut item = existing.clone()`), swaps load via `&mut item.loads`, patches whole case record |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs:20` | fem/◻️2d `UpdateAnalysisSettings` | V1-GENERIC-DIFF | diff carries whole `analysis` settings sub-document (`analysis: Some(payload.settings.clone())`) |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/↩️inverse/🦀️.rs:12` | fem/◻️2d `MoveSelection` | V2-DIFF-DERIVED-INVERSE | inverse calls `super::diff::diff` and rebuilds `ReplaceNode`/`ReplaceRegion` from `diff.nodes/regions.patched` |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🏋️load/🧬️schema/🧬️mutations/🔁️replace-load/🔺️diff/🦀️.rs:34` | fem/🧊️3d `ReplaceLoad` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V1-GENERIC-DIFF | diff clones parent case (`let mut item = existing.clone()`), swaps load via `&mut item.loads`, patches whole case record |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/📈️analysis/🧬️schema/🧬️mutations/🎛️update-analysis-settings/🔺️diff/🦀️.rs:16` | fem/🧊️3d `UpdateAnalysisSettings` | V1-GENERIC-DIFF | diff carries whole `analysis` settings sub-document (`analysis: Some(payload.settings.clone())`) |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/↩️inverse/🦀️.rs:11` | fem/🧊️3d `MoveSelection` | V2-DIFF-DERIVED-INVERSE | inverse calls `super::diff::diff` and rebuilds `ReplaceNode`/`ReplaceRegion` from `diff.nodes/regions.patched` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-frames/🦀️.rs:31` | layout/📏️layout `DragFrames` | V2-DIFF-DERIVED-INVERSE | inverse calls `layout_frame_selection_inverse(base, page, diff_*(self, base))`, which walks the forward `frames_patched` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️set-story-runs/🦀️.rs:50` | layout/📏️layout `SetStoryRuns` | V1-GENERIC-DIFF | patch sets whole `style_runs: Some(payload.runs.clone())` list |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️set-drawing-text/🦀️.rs:98` | layout/📏️layout `SetDrawingText` | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY(direct), V1-GENERIC-DIFF | clones `child.content` then `iter_mut`/`&mut layer.root` writes; diff carries whole `background_drawing` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📎set-page-overrides/🦀️.rs:54` | layout/📏️layout `SetPageOverrides` | V1-GENERIC-DIFF | patch sets whole `overrides: Some(payload.overrides.clone())` list |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐update-grid/🦀️.rs:43` | layout/📏️layout `UpdateGrid` | V1-GENERIC-DIFF | diff carries whole `GridSettings` sub-document (`grid: Some(next)`) for a set-field kind |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️set-page-guides/🦀️.rs:44` | layout/📏️layout `SetPageGuides` | V1-GENERIC-DIFF | patch sets whole `guides: Some(payload.guides.clone())` list |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔃️rotate-frames/🦀️.rs:33` | layout/📏️layout `RotateFrames` | V2-DIFF-DERIVED-INVERSE | inverse calls `layout_frame_selection_inverse(base, page, diff_*(self, base))`, which walks the forward `frames_patched` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖨️change-print-target/🦀️.rs:50` | layout/📏️layout `ChangePrintTarget` | V1-GENERIC-DIFF | diff carries whole `print_target` sub-document |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗜️scale-frames/🦀️.rs:33` | layout/📏️layout `ScaleFrames` | V2-DIFF-DERIVED-INVERSE | inverse calls `layout_frame_selection_inverse(base, page, diff_*(self, base))`, which walks the forward `frames_patched` |
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧾change-data-fields/🦀️.rs:44` | layout/📏️layout `ChangeDataFields` | V1-GENERIC-DIFF | diff carries whole form dictionary (`FormDictionaryChange { dictionary: payload.new_fields.clone() }`) |

Borderline leaves not counted as violations: `UpdateCharacterStyle`, `UpdateParagraphStyle` (layout; patch carries every
style field, so the mutation owns them all), `ReorderPages`, `ReorderFrame` (whole order list, index-based reorder),
the `ChangePrintTarget` and `RenameLayout` inverses (each restores one field; the diff side is counted), and `UpdateCamera` gen3d (inverse restores whole camera).

## 8. Method and limits

- Enumeration: `rg` for `impl MutationKind<` (164 sites, all under a `🧬️mutations` directory). The scanner's first pass
  matched one out-of-scope kind under `📖️playbook/🧩️extensions/🌀️procedural/.../📦️set-payload`; it is excluded.
- Bodies: `scan2.py` extracts `diff`/`inverse` bodies from the leaf's own `🔺️diff` and `↩️inverse` trees, or from the kind file
  for layout and the 17 inline procedural leaves, and follows helper calls within the same tree. Cross-file helpers were
  resolved with `rg` (section 2). Test files are excluded from the flags.
- Limit: `nontest()` truncates at the first `#[cfg(test)]`. Files with an earlier test module lose code after it; the
  affected leaves were spot-checked by hand (move-selection, change-slider-value, replace-load, set-drawing-text).
- Limit: V4 counts only inverse-referencing tests in the leaf's own `🧪️tests` tree, not aggregate-level tests.
- The design file changed on disk during the audit (V5-ABSORB, the Minimality ruling and the removed generic seams were
  added). The final pass uses the updated file.
- Scripts and intermediates: `🗑️generated/fem-procedural-layout-audit/` (`scan2.py`, `report.py`, `write_report.py`,
  `kinds2.json`, `report.json`). Not deleted, so the classification can be rerun.

No file outside `🗑️generated/fem-procedural-layout-audit/` and this report was written. No build, test or git command was run.
