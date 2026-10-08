# Executor report: fem-procedural-layout

Scope: `✏️s/🔌️plugins/🏗️fem` (2d, 3d), `✏️s/🔌️plugins/🌀️procedural` (generation2d, generation3d; MutationKind leaves and their diff types only), `✏️s/🔌️plugins/📏️layout`.

Status: WRITTEN BUT UNVERIFIED. Nothing in this scope has been compiled or run. Every cargo attempt (`"$T/🚦️gate.sh" fem-procedural-layout -- cargo check -p semio-s-artifact-fem-2d --target wasm32-wasip2 --message-format=short`, run from `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d`) stops in the peer crate `semio-framework-replication` with 12 `E0308` errors (`RetainedCloneGrant` vs `Grant`, `SharedOwner<String>` vs `Arc<String>` in `📡️wire/🏠️local-interaction/🌳️root/…`), i.e. the peer `🌱️value/🗂️ordered` change in flight. Last log: `T/🗑️generated/fem-procedural-layout/check-fem2d.log`. Static checks that were possible and are green: `rustfmt --edition 2021` parse of all 629 touched `🦀️.rs` files (no syntax error), `json.load` of all 563 touched JSON files, `bun x tsc --noEmit --noUnusedLocals --strict` of the four fem/procedural diff `🟦️.ts` and the layout diff `🟦️.ts`.

## Design applied

- One sparse, owned-field diff type per artifact. Every diff type has a concrete `MutationDiff::{apply (with `ApplyCapability`), absorb, retire_cold}` and `DiffAlgebra::{inverse, between, is_empty}`. `absorb` is a coalescing normal form (per id: patch / remove / add / replace = removed+added), never a concatenation. Leaves build their rows directly from `(payload, base)`; the shared whole-fixture helpers are gone.
- Position-exact inverses (wave-2 ruling). Every ordered-collection create/insert kind carries `index: Option<usize>` (absent or past the end = append), its diff emits `added` plus a complete `reordered` list through a per-diff-module `insertion_order(...)`, and the matching delete/remove inverse emits the create with `index: Some(original position)`.
- Replace kinds invert to the same kind carrying the base value (accepted by the ruling).
- Central apply: 19 non-leaf apply sites and about 370 test sites use `protocol::apply_diff`; the removed `MutationOutcome::apply_to` call sites (`fem{2,3}d/…/🚪️io/📝️text/🧬️mutations/🦀️.rs`, fem transient unit test) use `store::apply_outcome`.

## Per kind: before -> after

Classes: WS = whole-snapshot/whole-collection diff, CI = derived/clone-and-forward-diff inverse, CW = clone-and-write inline diff, CAT = concatenating `absorb`.

### fem (2d and 3d, identical shape)
| Kind | Before | After |
|---|---|---|
| create/delete node, element, section, material, support, load-case, combination, region (3d: solid) | WS diff (artifact field), delete inverse appended | sparse id-keyed rows; create gains `index`; delete inverse = create with `index: Some(pos)` |
| add-load / remove-load | WS (whole load case) | nested `…LoadCasePatch{loads: …LoadsDelta}` row; `AddLoad.index`; `RemoveLoad` inverse = `AddLoad` at original position |
| change-load-case-name / self-weight, replace-load, replace-combination, replace-element/node/region/section/material/support | WS or whole-record | owned-field patches or keyed replace rows; replace inverse = same kind with base value |
| update-analysis-settings | WS | `…AnalysisPatch` of changed fields only |
| move-selection | CI (derived from forward diff) | per-record absolute `Replace…` setters computed in the leaf (`moved_node`, `moved_region`/`moved_solid`, `breach`) |
| set-playback-clock (results transient) | CW | `FemResultsWindowTransientDiff{clock: Option<FemPlaybackClockChange>}` with `MutationDiff`/`DiffAlgebra` |
| results/model window config aggregates (4 hand `impl Mutation<`) | WS config | `Update{patch: …ConfigPatch}`; patch is owned-field, doubles as diff, `against(base)` trims unchanged fields |
| `Fem2dDiff`/`Fem3dDiff` absorb | CAT | coalescing normal form, unit tests added (`🔺️diff/🧪️tests/🔬️unit`) |

### procedural (generation2d, generation3d)
| Kind | Before | After |
|---|---|---|
| create/replace/delete widget, connect/replace/disconnect synapse, move-widget, clear-widget-layout, update-camera, change-schema, rename generation, change-generation-value, change-slider-value, move-nodes (2d); update-widget, change-widget-input, update-synapse, delete-widget-position, change-generation-preview, select-generation (3d) | WS via `diff_snapshot_from_helpers` (25 sites) / `diff_generation_from_ops` (8 sites) | direct sparse rows (`…WidgetsDelta`, `…SynapsesDelta`, `…LayoutDelta`, `…GenerationsDelta`, `…SelectionChange`, `…PreviewChange`); slider change goes through `set_widget_slider_value` on a probe widget into `…WidgetPatch::Slider` |
| create-generation / delete-generation | WS, append-only inverse | `CreateGeneration.index`; delete inverse = create at `Some(pos)`; 3d adds a `SelectGeneration` restore step when the deleted generation was not the selected one |
| create-widget / connect-synapse | index ignored | index honoured through `reordered`; delete/disconnect inverses restore the original position |
| editor config, view config, view presence, view transient, generation transient (2d, 3d) | CW (clone and write) | `…Patch` types with `MutationDiff`/`DiffAlgebra`; aggregators use `diff = …Patch`; `impl_whole_record_config!` replaced by `impl store::ConfigRecord` |
| gen3d editor config `SetSnapshot` | whole-snapshot kind | deleted (variant, leaf dir, fixtures, oracle rows, test case rows); commands emit `config_replacement(base, next)` as sparse leaf mutations |
| `Generation2dDiff`/`Generation3dDiff` absorb | CAT | coalescing; gen3d `DiffRegions` derived from rows; dead `diff_set_snapshot` deleted |
| text diff codecs (`🚪️io/📝️text/🔺️diff`) | old field list | rewritten for the new field ids; `CreateGeneration` DSL variant carries `index` |

Owned by `small-plugins` and left untouched: the hand config impls under `🌀️procedural/…/🎚️config/🦀️.rs`, `Generation3dPresenceMutation` descriptor, `Generation3dPreviewWindowTransient`.

### layout
| Kind | Before | After |
|---|---|---|
| set-story-runs, set-page-guides, set-page-overrides | WS (whole list) | positional / keyed deltas (`TextStyleRunsDelta`, `PageGuidesDelta`, `PageOverridesDelta`); overrides require a unique `object_id` |
| update-grid, change-print-target, change-data-fields | WS | `GridPatch`, `PrintTargetChange`, `LayoutDataFieldsDelta` (presence enum + keyed entries) |
| set-drawing-text | clone + `&mut` | sparse `LayoutDrawingTextRow` |
| drag-frames, rotate-frames, scale-frames | CI (`layout_frame_selection_inverse`, deleted) | concrete per-target absolute setters (`moved`, `turned`, `scaled` + `inverse_*_frames`) |
| create-page, create-story, create-link | `index` accepted but ignored (always appended) | diff uses `insertion_order`, delete inverses already carried `Some(pos)` and are now actually honoured |
| create-character-style / delete-character-style | no index, inverse appended | `CreateCharacterStyle.index`; inverse = create at `Some(pos)` + update |
| create-layer (remove mode) | no index, flags lost | `CreateLayer.index`, new `PageLayerAdded{layer, index}` in `PagePatch.layer_added`; remove inverse = create at original position + `UpdateLayer` when locked/hidden |
| window config | WS | `Update{patch: LayoutWindowConfigPatch}` |
| `LayoutDiff` absorb | CAT | coalescing over keyed/positional/page-patch-sequence rows, unit tests added |

## Law fixtures

- `inverse_diffs_sum_to_negative_diff` (async, calls `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation(), &before()).await`) is in 174 fixture test files appended by script, plus 24 layout files and 4 fem `move-selection` files (via a new `laws::inverse_sum` in both `🧭️move-selection/🦀️.rs`), plus layout reorder-frame and create-layer unit tests. Refusal and no-op fixtures are intentionally excluded.
- Middle-row law fixtures (new fixture dir plus test, registered in the root `mod` list): `📍️removes-a-middle-row` for fem 2d (node, element, section, material, support, load-case, combination, remove-load, region) and 3d (same, solid), procedural 2d (delete-widget, disconnect, delete-generation) and 3d (delete-widget, disconnect-synapse, delete-generation), layout (delete-page, delete-story, delete-link, delete-frame, delete-character-style); `📍️moves-a-middle-row` for layout reorder-pages. Unit tests: layout reorder-frame (middle frame) and create-layer (locked middle layer).
- Generators and migrations are kept in `T/🗑️generated/fem-procedural-layout/` (`fem_middle_fixtures.py`, `layout_middle_fixtures.py`, `procedural_middle_fixtures.py`, `add_index_fem.py`, `add_index_literals.py`, `async_sumlaw.py`, earlier `migrate_*.py`/`gen_*.py`).

## Schema documents touched

Diff docs (`🔗️.graphql`, `🛰️.proto`, `🔣️.json`, `🟦️.ts`) for fem 2d/3d, procedural 2d/3d and layout; mutation payload schemas, TS, GraphQL and text-codec grammars for every kind that gained `index` (`create-*`, `add-load`, `create-generation`, `create-character-style`, `create-layer`); layout `PagePatch.layer_added` now `PageLayerAdded`. Mutation fixture JSON for create kinds carries `"index": null`.

## Test status

All "not run". Exact commands to run once `semio-framework-replication` builds again (from the artifact workspace dir, through the gate):

- `cd ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d && "$T/🚦️gate.sh" fem-procedural-layout -- cargo check -p semio-s-artifact-fem-2d --target wasm32-wasip2 --message-format=short`
- same for `🧊️3d` (`semio-s-artifact-fem-3d`), `🌀️procedural/🗿️artifacts/🌀️generation2d`, `…/🧊️generation3d` (crate names in each `📦️packages/🦀️rust/Cargo.toml`), `📏️layout/🗿️artifacts/📏️layout` (`semio-s-artifact-layout-layout`).
- then `cargo test -p <crate> inverse_diffs_sum_to_negative_diff removes_a_middle_row middle absorb` (test builds are paused by the coordinator until the disk and `🌱️value/🗂️ordered` situation settles).

Expected first-compile fallout (not yet seen): unused or changed imports in leaf `🔺️diff` files, `base` unused in state-patch leaves, procedural/layout unit tests that still assert old diff shapes, layout `PagePatch{guides:Some(vec)}`-style literals in tests, DSL text codec round trip of the new trailing `Option<usize>` after a `statements`/`block` field (fem `variants_text`, procedural DSL enums), `deny_unknown_fields` on `PageLayerAdded`, and the `laws::*` helper in fem `move-selection` using `apply_fem2d_mutation`.

## Open issues

- gen2d has no select-generation kind, so `DeleteGeneration` inverse restores the selection only when the deleted generation was the selected one (gen3d restores it with a `SelectGeneration` step).
- `LayoutDiff::between` for pages is best effort; `background_drawing` and `referenced_model` stay `Option<Option<…>>` slots.
- `ChangeGenerationValue` inverse sets Null for an absent key (no representable "absent" in the value row).
- fixtures regenerated by script (diff JSON, middle-row fixtures, state diffs) are unverified against the Rust encoder; python oracle scripts (`🐍️.py`) compare snapshots only and were not touched.
- `assert_mutation_inverse_sum_law` is async and sits in `os_spr::protocol_laws`; tests use `semio_framework_async_macros::async_test` (declared in all three crates).
- Subset-level differential cases (`🧪️tests/…-mutate-…/🥒️.feature` and `🦀️.rs`) were not extended with the new fixtures; the leaf-level fixtures carry the law.
- Layout text/binary op codecs are JSON based, so the new `index` fields travel; the layout text grammar documents were extended (`create-character-style … text? number?`, `create-layer … text number? boolean?`).
