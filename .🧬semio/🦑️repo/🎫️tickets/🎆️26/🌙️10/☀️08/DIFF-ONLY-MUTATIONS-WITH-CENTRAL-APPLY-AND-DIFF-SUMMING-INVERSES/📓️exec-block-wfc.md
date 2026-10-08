# Executor report: block and wfc plugins (executor `block-wfc`)

Scope: `✏️s/🔌️plugins/🧱️block` (3 artifacts, 105 kinds) and `✏️s/🔌️plugins/🀄️wfc` (5 artifacts, 82 kinds). Status: **WRITTEN BUT UNVERIFIED at the type level.** No Rust code of this scope has been compiled, because every `cargo check` of a scope crate stops in the peer's `semio-framework-replication` (12 E0308 errors from the in-flight `🌱️value/🗂️ordered` change) or is OOM-killed (exit 137). What was verified: Python oracles, the TypeScript wfc2d twin and a rustfmt syntax pass over all 514 Rust files touched in the last 25 hours (details below).

## Test and check status (exact commands)

| what | command | result |
|---|---|---|
| cargo check, all 8 crates | `"$T/🚦️gate.sh" block-wfc -- cargo check -p <crate> --target wasm32-wasip2 --message-format=short` from each artifact workspace dir (driver: `T/🗑️generated/block-wfc/out/check.sh`) | **not green.** block-2d, block-5d, wfc-2d, wfc-3d, wfc-grid2d, wfc-bitmap: exit 137 (OOM kill while building framework crates). block-3d, wfc-grid3d: exit 101, all errors are in `semio-framework-replication` (`RetainedCloneGrant` vs `Grant`, `SharedOwner<String>` vs `Arc<String>`), none in a scope crate. Outputs: `T/🗑️generated/block-wfc/out/<crate>.check.txt` |
| cargo check, block-2d retry after the wave-2 edits | same gate command, started in the background by the harness (`T/🗑️generated/block-wfc/out/block-2d.retry.txt`) | ran after the wave-2 edits and was OOM-killed (Killed: 9) while building framework crates, before reaching any block code; **no result** |
| cargo test | **not run** (wave-2 ruling: no test builds while disk is low and replication is mid-change) | all Rust tests below are written, none executed |
| rustfmt syntax pass | `rustfmt --edition 2021 --check <file>` over 514 touched `*.rs` | found and fixed 4 real syntax errors (`}}` left by my generator in 4 transient `apply` bodies); now 0 parse errors. This proves syntax only, not types. |
| Python independent oracles | `python3 …/🧪️tests/🧩️mutate-wfc2d-1/🐍️.py` (and wfc3d, grid3d) | pass; fixtures for wfc2d (18), wfc3d (18), grid3d (14) were rewritten from these oracles |
| TypeScript wfc2d twin | `bun test` in the wfc2d artifact | 91 pass |
| block, grid2d, bitmap fixture diff JSON, config/transient fixtures | needs a Rust run (`fixture_hook.py`, not applied) | **not regenerated**; their `🔺️diff/🔣️.json` still hold the old shapes |

## What was done

Every diff type now has a sparse declarative shape, a concrete `DiffAlgebra::inverse` (and `between`, `is_empty`) and a sound base-free `absorb`.

- **Shared block algebra** (`🧱️block/🧬️schema/🧱️shared/🦀️.rs`, region `🔖️Patches`): `BlockPatch` (per-field patch with `patched/between/inverse/absorb/is_empty`), `BlockRows` (id-keyed `added/removed/patched/reordered`), macros `block_optional!`, `block_patch!`, `block_rows!`, shared patch types for identity, meta, cameras, authors, attributes, compatibility rules, representations (ordered tag/attribute sets with suffix-rewrite inverse), and `block_insert_order` (new). Re-exported from `🧱️block/🦀️.rs`.
- **Block diffs** (2d/3d/5d `🧬️schema/🔺️diff/🦀️.rs`): `Block*Diff` is field-sparse (identity, presentation, cameras, meta as patches; handles/vortices/grips, kinds, authors, attributes, compatibility, representations as keyed row deltas). `diff_set_snapshot` and every whole-entity copy are gone. 3d reads/writes `vortex_kinds` through `vortex_kinds_of`/`set_vortex_kinds`.
- **Block leaves**: all 100 V1-GENERIC kinds emit one field patch or one row delta; representation tag/attribute kinds are no-ops when already present/absent; `add-*` inverses return `Vec::new()` when the item exists.
- **Block empty states** (config 2d/5d, presence 2d/3d/5d): `*Diff {}` plus uninhabited `*Mutation` enums (manual `Mutation/OpText/OpBinary`), so there is no whole-state diff to carry.
- **Block3d config**: `Block3dConfigDiff` (per-field `Option`s, windows as `Block3dWindowsDelta` rows). New in this pass: window rows equal to the default view are never stored and rows stay sorted by window id, so `SetWindow*` edits and their inverses (which create or remove rows) restore the exact row list. The `Snapshot` mutation variant, `upsert_window_view_index` and the `large_enum_variant` justification comment are deleted.
- **Block3d transient**: `Block3dWorldWindowTransientDiff` + `set-brush-preview` as an optional field diff.
- **wfc diffs** (2d/3d/grid2d/grid3d/bitmap): `{P}Rows<T,Q>{removed,added,patched}` with canonical-position insertion (rows sorted by key, so remove then re-add lands at the same index), per-artifact patch types, `Grid3dAxisPatch{length,sizes}` for cell sizes, bitmap ordered op logs (`input_ops`, `palette_ops`). `resize-input` inverse restores only the cropped strips (`read_region`).
- **wfc config/transient/window**: `Wfc2dConfigDiff`/`Wfc3dConfigDiff` (`change-camera`, `change-active-tile`, `replace-config` now emits only the differing fields via `between`, inverts to itself with the base value), transient aggregates and `set-solve` (2d/3d/bitmap), grid2d/grid3d/bitmap window configs as setter variants.
- **Absorb soundness** (8 audited types): create-then-delete and patch-then-delete coalesce in the block rows macro and in the wfc rows; unit tests in each `🔺️diff/🧪️tests/🔬️unit/🦀️.rs` cover create∘delete, patch∘delete, patch∘patch, delete∘create and create∘patch with an `assert_absorb_law` (sequential equals absorbed).
- **Central apply**: no leaf or non-leaf site applies a diff directly any more; the remaining stale `apply_to` users (block2d io mutation report, block3d transient test, 7 docstrings) were moved to `protocol::apply_diff`.

### Wave-2 ruling (position-exact inverses)

- Ordered block collections: the 17 top-level insert kinds (`create-handle`, `create-handle-kind`, `create-vortex`, `create-vortex-kind`, `create-grip`, `create-grip-kind`, `create-representation`, `add-author`, `add-attribute`, `add-compatibility-rule` in each artifact that has them) and the 4 nested kinds (`add-representation-tag`, `add-representation-attribute` in 3d and 5d) carry `index: Option<u32>` (absent or past the end appends). The diff sets `reordered` through `block_insert_order` (nested sets rewrite the suffix). Every matching delete/remove inverse now calls the `*_at` builder with the base position. `*_at` builders are re-exported next to the plain builders. Schema JSON, `🟦️.ts`, `🛰️.proto`, `🔗️.graphql` of the 21+ kinds carry the optional `index`.
- wfc collections are canonically sorted, so a middle-row remove or insert inverts exactly by construction; each of the four wfc row diffs got `inverse_restores_middle_rows`.
- Whole-document kinds / helpers: none remain in scope (`diff_set_snapshot`, `Snapshot`/`Restore` variants, `replacement` helpers are deleted; `rg` shows only the unrelated solver `WfcRestore` checkpoint type).

### Tests written (not run)

- `🧱️block/🧬️schema/🧱️shared/🧪️tests/🦀️.rs`: algebra tests for patches/rows/ordered sets, `inserting_at_an_index_reorders_only_inside_the_list`.
- Per block artifact `🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`: `deleting_or_creating_a_middle_row_restores_its_original_index` (3 rows per collection, `assert_mutation_inverse_sum_law` plus `round_trip` for every delete/remove and every `*_at` insert; also asserts exact orders).
- Block3d config tests: `config_operation_inverse_restores_only_the_changed_field`, `window_edits_restore_the_exact_row_list_including_middle_rows`. Block 2d/5d config tests: `an_empty_config_has_only_the_empty_diff`.
- wfc diff unit tests (generated): apply/absorb/inverse/between laws plus `inverse_restores_middle_rows`.
- Per-leaf `assert_mutation_inverse_sum_law` fixtures exist for the existing leaf fixtures (dev-dep `protocol-laws` added to the 5 wfc crates); the new law tests above are unit-level, no new JSON fixture rows were produced.

## Open issues

1. **No compile proof.** Everything above is unverified until `semio-framework-replication` builds again. Expect a first pass of type errors, most likely in: the generated block diff macros (`block_patch!` field lists), `crate::vortex_kinds_of` borrow in `create-vortex-kind`, the `existing.clone()`/`position` tuple pattern in the 13 delete/remove inverses, `Option<u32>` in `DslRecord`/`OpBinary` payload derivation, and the old-API call sites I could not find by grep.
2. **Fixtures not regenerated** for block (2d/3d/5d), grid2d, bitmap diff JSON and the config/transient fixtures; they need one test run with the temporary `[DEBUG]` hook (`T/🗑️generated/block-wfc/gen/fixture_hook.py`, `--remove` strips it). No middle-row JSON fixture exists yet; the middle-row coverage is the unit tests above.
3. **Block diff schema companions are stale**: `🧬️schema/🔺️diff/🟦️.ts`, `🔣️.json`, `🛰️.proto`, `🔗️.graphql` of block 2d/3d/5d still describe the old `replacement` patches. They are derived from `#[artifact_schema]`; regenerate with the schema generator once the crates compile. (The wfc equivalents were rewritten.)
4. **Inverse list order.** The framework applies `Mutation::inverse` reversed (`assert_mutation_inverse_sum_law`, store fold `back.reverse()`). The 10 multi-step wfc inverses (`delete-tile` in 2d/3d/grid2d/grid3d, `delete-slot` 2d/3d, `resize-grid`, `mask-cell` (grid2d), `resize-input`, `resize-output`) were authored in application order and are now reversed before returning (`restore.reverse()`); their old fixture tests, which looped in vec order, now iterate `inverse.iter().rev()`. Single-step inverses (all block kinds) are unaffected. Other plugins may still carry the old order; the coordinator should state the convention in `design.md`.
5. **Window rows invariant** (block3d config): the canonical-row invariant (no default-equal rows, sorted by window id) holds for every row written through the config diff; a config loaded from disk with default-equal or unsorted rows is normalised only when its window is next edited.
6. Peer-owned errors (not fixed): `semio-framework-replication` E0308 (`RetainedCloneGrant`/`Grant`, `SharedOwner`/`Arc`).
7. TS twins for wfc3d/grid2d/grid3d/bitmap diffs were rewritten but their `bun test` has not been run; only wfc2d was (91 pass).

## Files touched (groups; all repo-relative under `✏️s/🔌️plugins`)

- `🧱️block/🦀️.rs`, `🧱️block/🧬️schema/🧱️shared/🦀️.rs`, `🧱️block/🧬️schema/🧱️shared/🧪️tests/🦀️.rs`
- `🧱️block/🗿️artifacts/{◻️2d,🧊️3d,🖐️5d}/**/🧬️schema/🔺️diff/{🦀️.rs,🧪️tests/🔬️unit/🦀️.rs}`
- `🧱️block/🗿️artifacts/*/**/🧬️schema/🧬️mutations/*/{🦀️.rs,🔺️diff,↩️inverse,🟦️.ts,🧬️schema/🔣️.json}` for the 105 kinds, plus `🧬️mutations/{🦀️.rs,🛰️.proto,🔗️.graphql,🧪️tests/🔬️unit/🦀️.rs}` per artifact
- `🧱️block/🗿️artifacts/*/**/✏️editor/{🎚️config,👥️presence}` (2d/3d/5d), 3d `🌐️world/🫧️transient` and `✏️editor/🦀️.rs`, 2d `🚪️io/📝️text/🧬️mutations/🦀️.rs`
- `🀄️wfc/🗿️artifacts/{◻️2d,🧊️3d,🔲️grid2d,🧱️grid3d,🖼️bitmap}/**/🧬️schema/{🔺️diff,🧬️mutations,📸️snapshot}`, `✏️editor/{🎚️config,🫧️transient,🪟️window}`, `🧪️tests/🧩️mutate-*/🐍️.py`, `🧫️fixtures` (wfc2d, wfc3d, grid3d diff JSON), `Cargo.toml` (dev-dep `protocol-laws`), TS twins `🔺️diff/🟦️.ts`
- Scratch (ticket-local, kept as inputs): `T/🗑️generated/block-wfc/gen/*.py`, `out/check.sh`.

## Per-kind classification (before -> after)

Before codes come from `T/🔍️audit-block-wfc.md`: V1G = V1-GENERIC-DIFF (whole entity/list/sub-document copy), V1S = V1-SNAPSHOT-DIFF (whole state), V2R = V2-RESTORE-INVERSE, V3L = V3-LEAF-APPLY / clone-and-write leaf, V4N = V4-LAW-UNTESTED, clean = no strict code. After: every kind is declarative sparse with a concrete inverse; the right-hand column names the shape. Tests per kind: law/unit tests written, not run (see status).

| artifact | kind | before | after |
|---|---|---|---|
| block ◻️2d | `add-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `add-author` | V1G, V3L | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `add-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `change-handle-handle-kind` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `change-handle-kind-color` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `change-handle-kind-default-wire-kind` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `change-handle-kind-label` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `change-meta-description` | clean | meta field patch |
| block ◻️2d | `change-node-kind-description` | V1G | kind-identity field patch |
| block ◻️2d | `change-node-kind-icon` | V1G | kind-identity field patch |
| block ◻️2d | `change-node-kind-label` | V1G | kind-identity field patch |
| block ◻️2d | `change-node-kind-unit` | V1G | kind-identity field patch |
| block ◻️2d | `change-node-kind-variant` | V1G | kind-identity field patch |
| block ◻️2d | `create-handle` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `create-handle-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `delete-handle` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `delete-handle-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `move-camera2d` | V1G | camera field patch |
| block ◻️2d | `move-handle` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `remove-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `remove-author` | V1G | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `remove-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block ◻️2d | `rename-handle-kind` | V1G | per-row field patch (id-keyed rows) |
| block ◻️2d | `rename-node-kind` | V1G | kind-identity field patch |
| block ◻️2d | `scale-camera2d` | V1G | camera field patch |
| block ◻️2d | `update-presentation` | V1G | presentation / part field patch |
| block 🖐️5d | `add-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `add-author` | V1G, V3L | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `add-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `add-representation-attribute` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🖐️5d | `add-representation-tag` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🖐️5d | `change-grip-grip-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🖐️5d | `change-grip-kind-color` | V1G | per-row field patch (id-keyed rows) |
| block 🖐️5d | `change-grip-kind-default-rope-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🖐️5d | `change-grip-kind-label` | V1G | per-row field patch (id-keyed rows) |
| block 🖐️5d | `change-meta-description` | clean | meta field patch |
| block 🖐️5d | `change-part-kind-description` | V1G | kind-identity field patch |
| block 🖐️5d | `change-part-kind-icon` | V1G | kind-identity field patch |
| block 🖐️5d | `change-part-kind-label` | V1G | kind-identity field patch |
| block 🖐️5d | `change-part-kind-unit` | V1G | kind-identity field patch |
| block 🖐️5d | `change-part-kind-variant` | V1G | kind-identity field patch |
| block 🖐️5d | `change-representation-description` | V1G | representation row patch (per-field) |
| block 🖐️5d | `change-representation-lod` | V1G | representation row patch (per-field) |
| block 🖐️5d | `change-representation-mesh-url` | V1G | representation row patch (per-field) |
| block 🖐️5d | `create-grip` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `create-grip-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `create-representation` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `delete-grip` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `delete-grip-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `delete-representation` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `move-camera2d` | V1G | camera field patch |
| block 🖐️5d | `move-camera3d` | V1G | camera field patch |
| block 🖐️5d | `move-grip2d` | clean | per-row field patch (id-keyed rows) |
| block 🖐️5d | `move-grip3d` | clean | per-row field patch (id-keyed rows) |
| block 🖐️5d | `remove-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `remove-author` | V1G | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `remove-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🖐️5d | `remove-representation-attribute` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🖐️5d | `remove-representation-tag` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🖐️5d | `rename-grip-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🖐️5d | `rename-part-kind` | V1G | kind-identity field patch |
| block 🖐️5d | `rename-representation` | V1G | representation row patch (per-field) |
| block 🖐️5d | `resize-grip3d` | clean | per-row field patch (id-keyed rows) |
| block 🖐️5d | `scale-camera2d` | V1G | camera field patch |
| block 🖐️5d | `scale-camera3d` | V1G | camera field patch |
| block 🖐️5d | `update-part2d` | clean | presentation / part field patch |
| block 🖐️5d | `update-part3d` | clean | presentation / part field patch |
| block 🧊️3d | `add-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `add-author` | V1G, V3L | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `add-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `add-representation-attribute` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🧊️3d | `add-representation-tag` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🧊️3d | `change-meta-description` | clean | meta field patch |
| block 🧊️3d | `change-object-kind-description` | V1G | kind-identity field patch |
| block 🧊️3d | `change-object-kind-icon` | V1G | kind-identity field patch |
| block 🧊️3d | `change-object-kind-label` | V1G | kind-identity field patch |
| block 🧊️3d | `change-object-kind-unit` | V1G | kind-identity field patch |
| block 🧊️3d | `change-object-kind-variant` | V1G | kind-identity field patch |
| block 🧊️3d | `change-representation-description` | V1G | representation row patch (per-field) |
| block 🧊️3d | `change-representation-lod` | V1G | representation row patch (per-field) |
| block 🧊️3d | `change-representation-mesh-url` | V1G | representation row patch (per-field) |
| block 🧊️3d | `change-vortex-kind-color` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `change-vortex-kind-default-cable-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `change-vortex-kind-label` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `change-vortex-label` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `change-vortex-vortex-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `create-representation` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `create-vortex` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `create-vortex-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `delete-representation` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `delete-vortex` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `delete-vortex-kind` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `move-camera3d` | V1G | camera field patch |
| block 🧊️3d | `move-vortex` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `remove-attribute` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `remove-author` | V1G | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `remove-compatibility-rule` | clean | keyed row delta (added/removed + `reordered`), optional `index` on inserts, position-exact inverse |
| block 🧊️3d | `remove-representation-attribute` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🧊️3d | `remove-representation-tag` | V1G | ordered-set patch on one representation (suffix rewrite), optional `index` on adds, position-exact inverse |
| block 🧊️3d | `rename-object-kind` | V1G | kind-identity field patch |
| block 🧊️3d | `rename-representation` | V1G | representation row patch (per-field) |
| block 🧊️3d | `rename-vortex-kind` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `resize-vortex` | V1G | per-row field patch (id-keyed rows) |
| block 🧊️3d | `scale-camera3d` | V1G | camera field patch |
| block 🧊️3d | `set-brush-preview` | V1S, V3L, V4N | transient field diff (`Block3dWorldWindowTransientDiff`) |
| wfc ◻️2d | `change-active-tile` | V1S, V3L, V4N | config field diff (only differing fields) |
| wfc ◻️2d | `change-camera` | V1S, V3L, V4N | config field diff (only differing fields) |
| wfc ◻️2d | `change-seed` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `change-tile-media` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `change-tile-weight` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `connect-slots` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `create-rule` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `create-slot` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `create-tile` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `delete-rule` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `delete-slot` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `delete-tile` | V1G | keyed row delta, canonical-position insert |
| wfc ◻️2d | `disconnect-slots` | clean | keyed row delta, canonical-position insert |
| wfc ◻️2d | `drag-slots` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `move-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `pin-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `replace-config` | V1S, V2R, V4N | config field diff (only differing fields) |
| wfc ◻️2d | `resize-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `set-slot-positions` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc ◻️2d | `set-solve` | V1S, V2R, V4N | transient field diff |
| wfc ◻️2d | `unpin-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `change-cell-size` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `change-periodicity` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `change-seed` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `change-tile-media` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `change-tile-weight` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `create-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🔲️grid2d | `create-tile` | clean | keyed row delta, canonical-position insert |
| wfc 🔲️grid2d | `delete-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🔲️grid2d | `delete-tile` | clean | keyed row delta, canonical-position insert |
| wfc 🔲️grid2d | `mask-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `pin-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `resize-grid` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `unmask-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🔲️grid2d | `unpin-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `add-palette-color` | V1G, V3L | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `change-model` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `change-palette-color` | V1G, V3L | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `change-seed` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `paint-input-stroke` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `pin-pixel` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `remove-palette-color` | V1G, V3L | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `resize-input` | V1G, V2R | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `resize-output` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `set-input-pixels` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🖼️bitmap | `set-solve` | V1S, V2R, V4N | transient field diff |
| wfc 🖼️bitmap | `unpin-pixel` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `change-active-tile` | V1S, V3L, V4N | config field diff (only differing fields) |
| wfc 🧊️3d | `change-camera` | V1S, V3L, V4N | config field diff (only differing fields) |
| wfc 🧊️3d | `change-seed` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `change-tile-media` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `change-tile-weight` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `connect-slots` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `create-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `create-slot` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `create-tile` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `delete-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `delete-slot` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `delete-tile` | V1G | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `disconnect-slots` | clean | keyed row delta, canonical-position insert |
| wfc 🧊️3d | `drag-slots` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `move-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `pin-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `replace-config` | V1S, V2R, V4N | config field diff (only differing fields) |
| wfc 🧊️3d | `resize-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `set-slot-positions` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧊️3d | `set-solve` | V1S, V2R, V4N | transient field diff |
| wfc 🧊️3d | `unpin-slot` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `change-cell-sizes` | V1G, V2R | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `change-periodicity` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `change-seed` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `change-tile-media` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `change-tile-weight` | V1G | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `create-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🧱️grid3d | `create-tile` | clean | keyed row delta, canonical-position insert |
| wfc 🧱️grid3d | `delete-rule` | clean | keyed row delta, canonical-position insert |
| wfc 🧱️grid3d | `delete-tile` | clean | keyed row delta, canonical-position insert |
| wfc 🧱️grid3d | `mask-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `pin-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `resize-grid` | V1G, V2R | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `unmask-cell` | clean | sparse per-field / per-row patch, concrete inverse |
| wfc 🧱️grid3d | `unpin-cell` | clean | sparse per-field / per-row patch, concrete inverse |

## Wave 3

- **Inverse order**: matches the wave-3 ruling (replay last-to-first). The 10 multi-step wfc inverses and their fixture loops were already converted (see open issue 4).
- **Block diff mirrors regenerated** (no compile needed): `🧬️schema/🔺️diff/{🟦️.ts,🔣️.json,🛰️.proto,🔗️.graphql}` for block 2d/3d/5d are rewritten from the sparse Rust types (patch objects with optional `Option` wrappers, row deltas with `added/removed/patched/reordered`). Generator: `T/🗑️generated/block-wfc/gen/block_mirrors.py`; the three JSON schemas parse as JSON. The old parser helpers in the stale TS had no external users and are gone (types only).
- **cargo check, block-2d** (`"$T/🚦️gate.sh" block-wfc -- cargo check -p semio-s-artifact-block-2d --target wasm32-wasip2 --message-format=short`, run from the artifact dir, output `T/🗑️generated/block-wfc/out/w3-block-2d.txt`): waited about 45 minutes in the gate queue (43 waiters), then **failed before any scope crate** with 14 errors in `semio-framework-replication` (`🔗️causal/🔀️transition/🔁️fold/🦀️.rs`: unresolved `semio_framework_value::retirement::owned_retirement`, `ErasedSnapshotRetirement` missing `next_close_byte_demand`, `RetainedCloneStep` vs `SnapshotRetirementStep`). These are a different set than the earlier `Grant` errors, i.e. a peer is still changing `semio_framework_value` retirement. No error mentions block or wfc code.
- **Not done in wave 3** (blocked on a compiling framework): cargo check for the other 7 crates, `cargo test`, fixture regeneration for block/grid2d/bitmap (`fixture_hook.py`). Counts: 0 crates green, 0 tests run.

### Gate burn-down R10 (leaf-between), 7 -> 0 (by source scan, not compiled)

`T/🗑️generated/coord/gate-run-2.log` listed 7 `diff-only-mutation/leaf-between` hits in this scope; all are replaced by declarative diffs built from payload and base reads:
- wfc2d / wfc3d `replace-config`: per-field `(base.f != config.f).then(..)` instead of `DiffAlgebra::between`.
- wfc grid3d `change-cell-sizes`: axis rows built inline from payload sizes versus the base axis (length unchanged by validation).
- wfc grid3d `resize-grid` (3 axes): new `Grid3dAxisPatch::resizing(base, extent)` (length plus appended cells), replacing `between(base, resized_axis(..))`.
- block3d config `window_edit`: the patched window row now carries the edit's own `Block3dWindowViewPatch` rather than a recomputed `between(row, next)`.
Remaining `between` occurrences are only `DiffAlgebra::between` implementations on diff types. rustfmt syntax check passes; type-check still blocked on the framework (see Wave 3).

## Wave 4 (translators T6, T8, T9) — written, not built

Foundation status at the time: `RED 19:09:19 native=101 wasm=101` (`T/🗑️generated/coord/foundation.status`), so nothing below was compiled or run. rustfmt syntax check passes on every touched file.

- **All seven `replace_document_operations` copies are deleted** (block 2d/3d/5d `🎨️edit` and `🎬️set-active-example`, and wfc bitmap `🎬️set-active-example`); `rg replace_document_operations` over `🧱️block` and `🀄️wfc` returns 0.
- **`set-active-example` = load.** Each handler now returns `Emit::effect(Effect::LoadDocument { pack, spr })` (packed snapshot plus `store::empty_document_spr`; the runtime re-stamps the identity) when the named example differs from the open document, and `Emit::default()` for an unknown id or the already-open example. No mutation rows, no diff, no history row. Block: one `load_document_effect(&BlockNdSnapshot)` per dimension in `set-active-example`; bitmap: `load_document_effect(&BitmapSnapshot)` in its own `set-active-example`.
- **`edit` (block)**: its payload is the whole next document as JSON text, so there is no addressed change to map onto concrete kinds; per the wave-4 rule "JSON load = load" it now calls the same `load_document_effect` (only when the parsed document differs from the open one; unparsable text stays a no-op). If a per-field edit gesture is wanted later, it needs a new payload carrying the addressed field, which the concrete kinds already cover.
- **Publication lanes**: block `setActiveExample` and `edit`, and bitmap `setActiveExample`, are `HostOnly` (like wfc2d's picker) instead of `Artifact`.
- **Tests updated**: block editor unit tests gained `context::load_example` (runs the handler and lands its `LoadDocument` through `artifact_app_laws::load_document`), the former `dispatch(SetActiveExample ..)` calls (10) use it, the 2d lane assertion expects `HostOnly` for the two loads, and each dimension has `example_switch_and_json_edit_load_a_document_instead_of_editing_one`. Bitmap's replay-order tests are replaced by three load laws (re-select answers nothing, switch is one `LoadDocument` and no mutation, unknown id is a no-op).
- **`block_rows_between` is NOT retired**: `DiffAlgebra::between` stays in the trait for sync/import (design.md), and the three block diff types plus `Block3dConfigDiff` implement it with `block_rows_between`; it is no longer reachable from any command or leaf.
- **Not verified**: whether the in-process dispatch lane stamps `LoadDocument` as the helper assumes, and whether the bitmap crate resolves `store`/`semio_framework::kernel` (it already uses `semio_framework_plugin`); first build will show.

## Wave 5 (gate-run-5: wfc 12 + block 4 breaches) — written, not built (foundation RED, no cargo)

Confirmation command: `bun ./📜️script.ts verify mutation-outcome-law` (outputs `T/🗑️generated/block-wfc/out/w5gate-before.txt` = 16 breaches in scope, `w5gate-after2.txt` and `w5gate-after3.txt` = scope lines counted below). After the first pass of fixes the gate listed **0** block/wfc lines; the list-delta migration below was re-checked in `w5gate-after3.txt`.

### Breaches fixed (16 -> 0)
- **R8 `&mut` in `🧬️mutations/🦀️.rs` (8: block 2d/3d/5d, wfc 2d/3d/grid2d/grid3d/bitmap)** and the "No compatibility re-exports" ruling: the `apply_<artifact>_mutation(&mut snapshot, &mutation)` helper is deleted in all eight crates. 751 call sites in 205 files (fixture tests, oracle adapters, unit tests) became `vcs::apply_mutation(&v, m).map(|(applied_state, _)| { v = applied_state; })`, an expression of the same `Result<(), MutationApplyError>` type, so `.expect/.is_ok/.is_err/.unwrap_or_else` call forms are unchanged; the `use ...::{apply_.., inverse_..}` lists lost the `apply_` name. `inverse_<artifact>_mutation` stays (read only, no `&mut`). Codemod: `T/🗑️generated/block-wfc/gen/w5_apply_helper.py`.
- **R9 `.apply(` in the block shared crate**: the `BlockRows::apply` seam is gone (see list-delta below).
- **R12 base clone (grid3d, bitmap)**: `Grid3dAxisPatch::patched` builds the axis from an index range (no `to_vec` into a `let mut`).
- **R8/R12 bitmap diff (4)**: the simulating op-log inverse is replaced by declarative state: `BitmapDiff { input: BitmapInputPatch { size, writes }, palette: BitmapPaletteDelta { removed (base idx), inserted {index (after idx), color}, recolored (base idx) } , ... }`. `inverse` reads the base buffer region by region and the base palette colour by colour; `absorb` is pure index arithmetic (`composed_input`: later resize wins, earlier writes clipped, a regrown area zero-filled; `BitmapPaletteDelta::composed` with `rank`/`nth_free`). Leaves (`add/remove/change-palette-color`, `resize-input`, `set-input-pixels`, `paint-input-stroke`), unit tests (palette coalescing, resize compose with zero fill, middle palette entries, writes layering), the TS twin (`applyBitmapDiff`), the JSON schema and the Python `declared_lanes` follow the new shape.
- **R14 `replace-config` (wfc2d, wfc3d)**: the inverse is now the concrete setters `[ChangeActiveTile(base), ChangeCamera(base)]` instead of `ReplaceConfig`.

### AMB-1 positional rows / "One positional list delta" (block)
- The local `block_rows!` / `BlockRows` / `BlockRowParts` / `reordered` (and `block_insert_order`) are **deleted**. Every block row list is a `protocol::list_delta!` delta: wire `removed [{id,index}]`, `inserted [{index,row}]`, `moved [{id,from,to}]`, `modified [{id,patch}]` (modification types keep the `…PatchEntry` names). Shared crate: attributes, authors, compatibility, representations; artifacts: handle kinds/handles (2d), vortex kinds/vortices (3d), grip kinds/grips (5d), per-window views (3d config). The shared crate now depends on `semio-framework-os-kernel` (`🧱️block/Cargo.toml` workspace dependency + package dependency).
- `block_patch!` also implements `protocol::list_delta::RowPatch` (hand impl for `BlockRepresentationPatch`); `block_apply_error` lifts a patch rejection; `block_insert_index(len, Option<u32>)` is the only local helper (slot of a new row).
- Diff types hold the plain delta (`Block2dDiff.handles: Block2dHandlesDelta`, not `Option`); `apply` uses `commit_onto(&base.rows, capability)`, `absorb`/`inverse(&base.rows)`/`is_empty` are the framework's. Leaves build rows from payload + base reads: 17 inserts `Delta::insertion(block_insert_index(len, payload.index), row)`, 17 removals `Delta::removal(&list, position)`.
- AMB-3: every `inverse` reads the base row by row (framework `Parts::inverse`, `BitmapPaletteDelta::inverse`, `inverse_input_patch`, `set_inverse`); no simulation, no apply on a copy.
- wfc keyed rows (`{P}Rows{removed,added,patched}` with canonical-position insertion, five copies) are **not** positional and still local; migrating them to `protocol::list_delta` changes their wire shape (added -> inserted with index) and is the remaining AMB-1 work in wfc (open issue below).
- No `between`: no non-impl callers remain in block/wfc (the four test callers were removed).

### Fixtures and mirrors
- Bitmap: 12 fixture diffs regenerated by `T/🗑️generated/block-wfc/gen/w5_bitmap_fixtures.py`, which also replays each diff with an independent Python apply and asserts equality with the committed `after`; `bun test ./…/🧩️suite/🟦️.ts` in the bitmap artifact: **20 pass, 0 fail**.
- Block diff mirrors (`🔺️diff/{🟦️.ts,🔣️.json,🛰️.proto,🔗️.graphql}`) regenerated for the positional/`modified` shapes (`block_mirrors.py`); the three JSON schemas parse. Block and grid2d diff fixture JSON (`🧫️fixtures/…/🔺️diff/🔣️.json`) are **still the old shapes**: regenerating them needs a Rust test run.

### Open (needs a compiling foundation)
1. cargo check of the 8 crates and every Rust test written in waves 1-5 (nothing compiled).
2. Block and grid2d diff fixtures (above).
3. wfc keyed rows -> `protocol::list_delta` (five artifacts' `{P}Rows`, ~100 leaf diffs, fixtures, Python oracles, TS twins).
4. Block create/add kinds that carry `index` still describe the same payload; the `rows` mirrors are required (non-null) fields now.
