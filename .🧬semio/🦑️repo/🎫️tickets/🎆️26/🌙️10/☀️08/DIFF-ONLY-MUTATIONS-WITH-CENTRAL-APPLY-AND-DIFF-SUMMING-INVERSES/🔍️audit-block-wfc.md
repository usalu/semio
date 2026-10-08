# Audit: block and wfc plugins, diff-only mutations (read-only)

Ticket: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️08/DIFF-ONLY-MUTATIONS-WITH-CENTRAL-APPLY-AND-DIFF-SUMMING-INVERSES`. Scope: `✏️s/🔌️plugins/🧱️block` (105 kinds) and `✏️s/🔌️plugins/🀄️wfc` (82 kinds). Paths are repo-relative to `/Users/ueli/Documents/semio`. Read-only: no source edits, no builds, no git writes.

## Method and rulings applied

- Kinds: 187 `impl MutationKind<` sites. 177 delegate diff/inverse to `super::diff::diff` / `super::inverse::inverse`. 10 editor config/transient kinds have their diff and inverse inline in the kind root.
- Hand-written `impl Mutation<P>`: 10 sites. `impl MutationDiff<P>`: 15 sites, all with `apply` and `absorb`. `DiffAlgebra`: 0 sites.
- Rulings from the updated `📋️design.md` are applied:
  - Minimality: a rename, set, move, change, scale, resize, pin, unpin, add or remove kind that emits a whole entity record, a whole list, or a whole sub-document is V1-GENERIC-DIFF. This is detected mechanically in the diff bodies (`..x.clone()` entity copies, `..base.<sub>.clone()` sub-document copies, clone-then-write records, `filter().cloned().collect()` lists, sub-documents built from payload). 91 kinds match; `delete-tile` matches through its slot-unpin cascade. Earlier this was counted as a soft category; it is now strict under the ruling.
  - Absorb soundness: V5-ABSORB is checked by reading each diff type's `absorb`.
  - Generic seams are deleted: whole-snapshot config and window diffs, and `SetSnapshot`/`Restore` inverse variants, are strict violations (V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE).
- Other strict checks, unchanged: V1-SNAPSHOT-DIFF (full replacement or base-clone diff), V2-RESTORE-INVERSE (inverse restores a whole snapshot, transient, config or collection), V2-DIFF-DERIVED and V2-EMPTY (none), V3-LEAF-APPLY (leaf writes into a clone of a whole base collection or config), V3-HAND-MUTATION, V4-LAW-UNTESTED (no inverse-law test at all).
- Concurrency: another process wrote different scripts and a `classified.json` into `🗑️generated/` during this audit. Those files are not mine and were not used. All numbers come from `🗑️generated/block-wfc/` (`extract.py`, `extract2.py`, `tests.py`, `minimal.py`, `final.py`, `compose2.py`).
- Limits: rg-based scan plus reading the bodies of each flagged kind and each absorb. Nothing was executed, so the absorb failures are from code reading, not from running the law tests.

## Per-artifact summary

| artifact | kinds | V1-SNAPSHOT-DIFF | V1-GENERIC-DIFF | V2-RESTORE-INVERSE | V2-DIFF-DERIVED | V2-EMPTY | V3-LEAF-APPLY | V4-LAW-UNTESTED (no inverse test) | clean (no strict code) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| block ◻️2d | 26 | 0 | 17 | 0 | 0 | 0 | 1 | 0 | 9 |
| block 🖐️5d | 41 | 0 | 29 | 0 | 0 | 0 | 1 | 0 | 12 |
| block 🧊️3d | 38 | 1 | 26 | 0 | 0 | 0 | 2 | 1 | 11 |
| wfc ◻️2d | 21 | 4 | 9 | 2 | 0 | 0 | 2 | 4 | 8 |
| wfc 🧊️3d | 21 | 4 | 9 | 2 | 0 | 0 | 2 | 4 | 8 |
| wfc 🔲️grid2d | 14 | 0 | 2 | 0 | 0 | 0 | 0 | 0 | 12 |
| wfc 🧱️grid3d | 14 | 0 | 4 | 2 | 0 | 0 | 0 | 0 | 10 |
| wfc 🖼️bitmap | 12 | 1 | 4 | 2 | 0 | 0 | 3 | 1 | 7 |
| **total** | **187** | 10 | 100 | 8 | 0 | 0 | 11 | 10 | **77** |

Clean = no strict code. The 77 clean kinds are the create, delete, sparse add/remove and sparse set kinds (for example `create-*`, `delete-*` except `delete-tile`, `add-attribute`, `remove-attribute`, `add-compatibility-rule`, `set-input-pixels`, `paint-input-stroke`, `resize-output`, `pin-cell`, `mask-cell`).

Empty inverses: 12 kinds return `Vec::new()` in a branch. Each branch is either a target-missing case where the forward diff errors, or a no-op where the forward diff is an unchanged patch (`remove-representation-tag`). None is a state-changing mutation with an empty inverse, so V2-EMPTY is 0.

## Shared generic helpers

| helper | file:line | leaf callers in scope | role / violation |
|---|---|---|---|
| `diff_set_snapshot` | repo-relative `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:526` (5d `:559`, 3d `:509`) | 0 | V1-SNAPSHOT-DIFF: whole-artifact diff builder (`artifact: Some(..)`). Defined in block, not called. Deleted by ruling. |
| `resized_axis` | `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:371` | 1 (`resize-grid`) | V1-GENERIC-DIFF: returns a whole resized axis array. |
| `vortex_kinds_of` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:138` | 11 kinds (reads) | clean: read-only lookup. |
| concrete builders (`create_rule`, `pin_slot`, `pin_cell`, `connect_slots`, `create_tile`, `set_slot_positions`, `change_seed`, `change_tile_media`, …) | `…/🧬️mutations/<kind>/🦀️.rs` and `crate::mutations::*` | 2–6 each | clean: concrete inverse builders. |
| lookups and region reads (`tile_index`, `pinned_index`, `masked_index`, `cell_key`, `cell_id`, `pin_index`, `read_region`, `in_bounds`, `cell_in_grid`, `stroke_extent`, `ordered_cell_index`) | `…/🧬️schema/📸️snapshot/🦀️.rs` (3d:157; grid3d:332/343/347/365; grid2d:231/242/246/252; bitmap:145/188/223) | 2–6 each | clean. |
| absorb merges (`absorb_delta`, `absorb_patches`, `absorb_col`, `merge_upserts`, `merge_pins`, `merge_delta!`) | diff files (block 2d `…/🔺️diff/🦀️.rs:345–414`; wfc merge helpers at `🔺️diff/🦀️.rs:44–68`) | applier side | see V5 below. |
| `apply_<artifact>_mutation` / `inverse_<artifact>_mutation` | each `…/🧬️schema/🧬️mutations/🦀️.rs` (block 2d :123, :130) | all | clean: apply goes through framework `vcs::apply_mutation`; inverse calls `mutation.inverse`. |
| diff-derived inverse (`*_inverse(base, outcome)`, `inverse_from_diff`, `diff(` in an inverse body) | none | 0 | V2-DIFF-DERIVED = 0. |

## Diff types and absorb

| artifact | diff type (file:line) | apply | absorb | DiffAlgebra | type-level finding | V5-ABSORB |
|---|---|---|---|---|---|---|
| block 2d | `Block2dDiff` (✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:305) | yes | yes | no | `authors` whole list (V1-GENERIC); `artifact` whole-snapshot escape hatch (V1-SNAPSHOT, unused) | yes |
| block 3d | `Block3dDiff` (✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:298) | yes | yes | no | `authors` whole list; `artifact` escape hatch | yes |
| block 5d | `Block5dDiff` (✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:339) | yes | yes | no | `authors` whole list; `artifact` escape hatch | yes |
| block 2d/3d/5d | `*Presence` (…✏️editor/👥️presence/🦀️.rs:21) | yes (`Ok(self.clone())`) | yes (replace) | no | whole state as diff (V1-SNAPSHOT) | sound (last-wins) |
| block 3d | `Block3dWorldWindowTransient` (…🫧️transient/🦀️.rs:71) | yes | yes (replace) | no | whole state (V1-SNAPSHOT) | sound |
| wfc 2d | `Wfc2dDiff` (✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:108) | yes | yes | no | sparse; merge_upserts | yes |
| wfc 3d | `Wfc3dDiff` (✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:106) | yes | yes | no | sparse; merge_upserts | yes |
| wfc 2d/3d | `Wfc2dTransient`, `Wfc3dTransient` (…✏️editor/🫧️transient/…/🦀️.rs:21) | yes | yes (replace) | no | whole transient (V1-SNAPSHOT) | sound |
| wfc grid2d | `Grid2dDiff` (✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:139) | yes | yes | no | sparse; merge_upserts | yes |
| wfc grid3d | `Grid3dDiff` (✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:143) | yes | yes | no | `cell_sizes_x/y/z` whole axis lists (V1-GENERIC); merge_upserts | yes |
| wfc bitmap | `BitmapDiff` (✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:128) | yes | yes | no | `palette`, `input_pixels` whole (V1-GENERIC); `merge_pins` | yes (pins); `input_regions` concatenation sound |
| wfc bitmap | `BitmapTransient` (…✏️editor/🫧️transient/🦀️.rs:67) | yes | yes (replace) | no | whole transient (V1-SNAPSHOT) | sound |

`DiffAlgebra` is implemented for none of the 15 diff types. The trait is in the framework (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:155`).

## Hand-written `impl Mutation<P>` sites (10)

These are not `MutationKind` leaves and bypass `#[derive(Mutations)]` (V3-HAND-MUTATION). File:line is the `impl` line.

| file:line | site | codes | evidence |
|---|---|---|---|
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:139` | block 2d config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | diff returns the whole `config.clone()` (`Snapshot` variant); inverse restores `Snapshot { config: base.clone() }`. Whole-snapshot config diff is deleted by ruling. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:214` | block 3d config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION, V3-LEAF-APPLY | `let mut next = base.clone()`, fields written in a match, whole config returned as the diff; every variant's inverse is `Snapshot { config: base.clone() }`. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs:136` | block 5d config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `config.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:89` | block 2d presence | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `presence.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:89` | block 3d presence | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `presence.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs:89` | block 5d presence | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `presence.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:63` | wfc grid2d window config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `config.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🦀️.rs:56` | wfc grid3d window config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `config.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs:29` | bitmap input window config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `config.clone()` diff; inverse restores `base.clone()`. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config/🦀️.rs:28` | bitmap output window config | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE, V3-HAND-MUTATION | whole `config.clone()` diff; inverse restores `base.clone()`. |

## Violating kinds

110 of 187 kinds have at least one strict code. Line is the diff function, or the `impl MutationKind` line for inline kinds.

| file:line | kind (artifact) | codes | evidence |
|---|---|---|---|
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤️add-author/🔺️diff/🦀️.rs:7` | add-author (block ◻️2d) | V1G, V3L | `let mut values = base.authors.clone(); values.push(..)` then `authors: Some(Block*AuthorList { values })`: whole author list carried |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷️change-handle-handle-kind/🔺️diff/🦀️.rs:7` | change-handle-handle-kind (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-handle-kind-color/🔺️diff/🦀️.rs:7` | change-handle-kind-color (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️change-handle-kind-default-wire/🔺️diff/🦀️.rs:7` | change-handle-kind-default-wire-kind (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️change-handle-kind-label/🔺️diff/🦀️.rs:7` | change-handle-kind-label (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-node-kind-description/🔺️diff/🦀️.rs:8` | change-node-kind-description (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-node-kind-icon/🔺️diff/🦀️.rs:8` | change-node-kind-icon (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-node-kind-label/🔺️diff/🦀️.rs:8` | change-node-kind-label (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️change-node-kind-unit/🔺️diff/🦀️.rs:8` | change-node-kind-unit (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-node-kind-variant/🔺️diff/🦀️.rs:8` | change-node-kind-variant (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥️move-camera2d/🔺️diff/🦀️.rs:8` | move-camera2d (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-handle/🔺️diff/🦀️.rs:7` | move-handle (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚷️remove-author/🔺️diff/🦀️.rs:8` | remove-author (block ◻️2d) | V1G | whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-handle-kind/🔺️diff/🦀️.rs:7` | rename-handle-kind (block ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-node-kind/🔺️diff/🦀️.rs:8` | rename-node-kind (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍️scale-camera2d/🔺️diff/🦀️.rs:8` | scale-camera2d (block ◻️2d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️update-presentation/🔺️diff/🦀️.rs:7` | update-presentation (block ◻️2d) | V1G | whole sub-document built from payload fields and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/🔺️diff/🦀️.rs:7` | add-author (block 🖐️5d) | V1G, V3L | `let mut values = base.authors.clone(); values.push(..)` then `authors: Some(Block*AuthorList { values })`: whole author list carried |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/🔺️diff/🦀️.rs:8` | add-representation-attribute (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/🔺️diff/🦀️.rs:8` | add-representation-tag (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-grip-grip-kind/🔺️diff/🦀️.rs:7` | change-grip-grip-kind (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-grip-kind-color/🔺️diff/🦀️.rs:7` | change-grip-kind-color (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪢change-grip-kind-default-rope-kind/🔺️diff/🦀️.rs:7` | change-grip-kind-default-rope-kind (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-grip-kind-label/🔺️diff/🦀️.rs:7` | change-grip-kind-label (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-part-kind-description/🔺️diff/🦀️.rs:8` | change-part-kind-description (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-part-kind-icon/🔺️diff/🦀️.rs:8` | change-part-kind-icon (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-part-kind-label/🔺️diff/🦀️.rs:8` | change-part-kind-label (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-part-kind-unit/🔺️diff/🦀️.rs:8` | change-part-kind-unit (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-part-kind-variant/🔺️diff/🦀️.rs:8` | change-part-kind-variant (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation/🔺️diff/🦀️.rs:8` | change-representation-description (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/🔺️diff/🦀️.rs:8` | change-representation-lod (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/🔺️diff/🦀️.rs:8` | change-representation-mesh-url (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera2d/🔺️diff/🦀️.rs:8` | move-camera2d (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎬move-camera3d/🔺️diff/🦀️.rs:8` | move-camera3d (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-grip2d/🔺️diff/🦀️.rs:7` | move-grip-2d (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭move-grip3d/🔺️diff/🦀️.rs:7` | move-grip-3d (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/🔺️diff/🦀️.rs:8` | remove-author (block 🖐️5d) | V1G | whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/🔺️diff/🦀️.rs:8` | remove-representation-attribute (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/🔺️diff/🦀️.rs:8` | remove-representation-tag (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-grip-kind/🔺️diff/🦀️.rs:7` | rename-grip-kind (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-part-kind/🔺️diff/🦀️.rs:8` | rename-part-kind (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/🔺️diff/🦀️.rs:8` | rename-representation (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-grip3d/🔺️diff/🦀️.rs:7` | resize-grip-3d (block 🖐️5d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera2d/🔺️diff/🦀️.rs:8` | scale-camera2d (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔎scale-camera3d/🔺️diff/🦀️.rs:8` | scale-camera3d (block 🖐️5d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️update-part2d/🔺️diff/🦀️.rs:7` | update-part-2d (block 🖐️5d) | V1G | whole sub-document built from payload fields and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👤add-author/🔺️diff/🦀️.rs:7` | add-author (block 🧊️3d) | V1G, V3L | `let mut values = base.authors.clone(); values.push(..)` then `authors: Some(Block*AuthorList { values })`: whole author list carried |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩add-representation-attribute/🔺️diff/🦀️.rs:8` | add-representation-attribute (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖add-representation-tag/🔺️diff/🦀️.rs:8` | add-representation-tag (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📃️change-object-kind/🔺️diff/🦀️.rs:8` | change-object-kind-description (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-object-kind-icon/🔺️diff/🦀️.rs:8` | change-object-kind-icon (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️change-object-kind-label/🔺️diff/🦀️.rs:8` | change-object-kind-label (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐change-object-kind-unit/🔺️diff/🦀️.rs:8` | change-object-kind-unit (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️change-object-kind-variant/🔺️diff/🦀️.rs:8` | change-object-kind-variant (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📜change-representation/🔺️diff/🦀️.rs:8` | change-representation-description (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏔️change-representation-lod/🔺️diff/🦀️.rs:8` | change-representation-lod (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌐change-representation-mesh-url/🔺️diff/🦀️.rs:8` | change-representation-mesh-url (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨change-vortex-kind-color/🔺️diff/🦀️.rs:7` | change-vortex-kind-color (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌change-vortex-kind-default-cable/🔺️diff/🦀️.rs:7` | change-vortex-kind-default-cable-kind (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎫change-vortex-kind-label/🔺️diff/🦀️.rs:7` | change-vortex-kind-label (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪧change-vortex-label/🔺️diff/🦀️.rs:7` | change-vortex-label (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷change-vortex-vortex-kind/🔺️diff/🦀️.rs:7` | change-vortex-vortex-kind (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎥move-camera3d/🔺️diff/🦀️.rs:8` | move-camera3d (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍move-vortex/🔺️diff/🦀️.rs:7` | move-vortex (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🙅remove-author/🔺️diff/🦀️.rs:8` | remove-author (block 🧊️3d) | V1G | whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖remove-representation-attribute/🔺️diff/🦀️.rs:8` | remove-representation-attribute (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚫remove-representation-tag/🔺️diff/🦀️.rs:8` | remove-representation-tag (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-object-kind/🔺️diff/🦀️.rs:8` | rename-object-kind (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✒️rename-representation/🔺️diff/🦀️.rs:8` | rename-representation (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖋️rename-vortex-kind/🔺️diff/🦀️.rs:7` | rename-vortex-kind (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏resize-vortex/🔺️diff/🦀️.rs:7` | resize-vortex (block 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔍scale-camera3d/🔺️diff/🦀️.rs:8` | scale-camera3d (block 🧊️3d) | V1G | whole sub-document copied from base (`..base.<sub>.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧬️schema/🧬️mutations/👁️set-brush-preview/🦀️.rs:1` | set-brush-preview (block 🧊️3d) | V1S, V3L, V4N | `let mut next = base.clone(); next.brush_preview.clone_from(..)` returned as the diff: whole transient clone mutated |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🀄️change-active-tile/🦀️.rs:1` | change-active-tile (wfc ◻️2d) | V1S, V3L, V4N | `Wfc*Config { active_tile_id, ..base.clone() }`: whole config clone written and returned as the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️change-camera/🦀️.rs:1` | change-camera (wfc ◻️2d) | V1S, V3L, V4N | `Wfc*Config { camera_x, camera_y, camera_zoom, ..base.clone() }`: whole config clone written and returned |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🔺️diff/🦀️.rs:6` | change-tile-media (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🔺️diff/🦀️.rs:6` | change-tile-weight (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🔺️diff/🦀️.rs:6` | delete-tile (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-slots/🔺️diff/🦀️.rs:7` | drag-slots (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↔️move-slot/🔺️diff/🦀️.rs:6` | move-slot (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🔺️diff/🦀️.rs:6` | pin-slot (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔄️replace-config/🦀️.rs:1` | replace-config (wfc ◻️2d) | V1S, V2R, V4N | `MutationOutcome::new(self.config.clone())`: full-config replacement; inverse `ReplaceConfig { config: base.clone() }` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🔺️diff/🦀️.rs:6` | resize-slot (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-slot-positions/🔺️diff/🦀️.rs:7` | set-slot-positions (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🏁️set-solve/🦀️.rs:1` | set-solve (wfc ◻️2d) | V1S, V2R, V4N | diff is the whole transient (`assignments` / `output_pixels` cloned from payload): full replacement; inverse restores whole field from base |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔓️unpin-slot/🔺️diff/🦀️.rs:6` | unpin-slot (wfc ◻️2d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🀄️change-active-tile/🦀️.rs:1` | change-active-tile (wfc 🧊️3d) | V1S, V3L, V4N | `Wfc*Config { active_tile_id, ..base.clone() }`: whole config clone written and returned as the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🎥️change-camera/🦀️.rs:1` | change-camera (wfc 🧊️3d) | V1S, V3L, V4N | `Wfc*Config { camera_x, camera_y, camera_zoom, ..base.clone() }`: whole config clone written and returned |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🔺️diff/🦀️.rs:6` | change-tile-media (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🔺️diff/🦀️.rs:6` | change-tile-weight (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🔺️diff/🦀️.rs:7` | delete-tile (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✋️drag-slots/🔺️diff/🦀️.rs:7` | drag-slots (wfc 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff; whole remaining list rebuilt from base by filter/collect and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🚚️move-slot/🔺️diff/🦀️.rs:6` | move-slot (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-slot/🔺️diff/🦀️.rs:6` | pin-slot (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🧬️mutations/🔄️replace-config/🦀️.rs:1` | replace-config (wfc 🧊️3d) | V1S, V2R, V4N | `MutationOutcome::new(self.config.clone())`: full-config replacement; inverse `ReplaceConfig { config: base.clone() }` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-slot/🔺️diff/🦀️.rs:7` | resize-slot (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎯️set-slot-positions/🔺️diff/🦀️.rs:7` | set-slot-positions (wfc 🧊️3d) | V1G | whole entity record copied from base (`..x.clone()`) and carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/🏁️set-solve/🦀️.rs:1` | set-solve (wfc 🧊️3d) | V1S, V2R, V4N | diff is the whole transient (`assignments` / `output_pixels` cloned from payload): full replacement; inverse restores whole field from base |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-slot/🔺️diff/🦀️.rs:6` | unpin-slot (wfc 🧊️3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️change-tile-media/🔺️diff/🦀️.rs:7` | change-tile-media (wfc 🔲️grid2d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🔺️diff/🦀️.rs:7` | change-tile-weight (wfc 🔲️grid2d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📏️change-cell-sizes/🔺️diff/🦀️.rs:7` | change-cell-sizes (wfc 🧱️grid3d) | V1G, V2R | `sizes = Some(payload.sizes.clone())` whole axis list carried (doc says id-keyed delta); inverse restores whole list |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️change-tile-media/🔺️diff/🦀️.rs:7` | change-tile-media (wfc 🧱️grid3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚖️change-tile-weight/🔺️diff/🦀️.rs:7` | change-tile-weight (wfc 🧱️grid3d) | V1G | entity cloned from base, one field written, whole record carried in the diff |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-grid/🔺️diff/🦀️.rs:7` | resize-grid (wfc 🧱️grid3d) | V1G, V2R | `cell_sizes_x/y/z: Some(resized_axis(..))` whole axis arrays carried; inverse `change_cell_sizes(axis, base.cell_sizes_*.clone())` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️add-palette-color/🔺️diff/🦀️.rs:7` | add-palette-color (wfc 🖼️bitmap) | V1G, V3L | whole palette cloned from base, entry inserted, carried as `palette: Some(..)` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖍️change-palette-color/🔺️diff/🦀️.rs:6` | change-palette-color (wfc 🖼️bitmap) | V1G, V3L | `let mut palette = base.input.palette.clone(); palette[i] = ..` carried as `palette: Some(..)`: whole palette |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️remove-palette-color/🔺️diff/🦀️.rs:7` | remove-palette-color (wfc 🖼️bitmap) | V1G, V3L | whole palette cloned from base, entry removed, carried as `palette: Some(..)` |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-input/🔺️diff/🦀️.rs:12` | resize-input (wfc 🖼️bitmap) | V1G, V2R | whole resized `input_pixels` buffer carried; inverse `set_input_pixels(0,0,w,h, base.input.pixels.clone())` restores whole buffer |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/🧬️mutations/👁️set-solve/🦀️.rs:1` | set-solve (wfc 🖼️bitmap) | V1S, V2R, V4N | diff is the whole transient (`assignments` / `output_pixels` cloned from payload): full replacement; inverse restores whole field from base |

## V5-ABSORB (absorb soundness)

The absorb contract requires `absorb(d1, d2).apply(base) == d1.apply(base).and_then(|mid| d2.apply(&mid))` whenever sequential application succeeds. Eight diff types merge id-keyed collections. Each one fails create-then-delete, and block also fails patch-then-delete. The merges are not base-free-sound:

| file:line | diff type (artifact) | code | evidence |
|---|---|---|---|
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:345` | Block2dDiff (block ◻️2d) | V5-ABSORB |  identified delta: `removed.extend` + `added.extend` + `patched` without cancellation: create∘delete leaves `added X` + `removed X` (fails apply: removed target missing); patch∘delete leaves `patched X` + `removed X` (fails apply). |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:340` | Block3dDiff (block 🧊️3d) | V5-ABSORB | same identified-delta merge as block 2d (`merge_delta!`): create∘delete and patch∘delete produce unapplicable diffs. |
| `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:432` | Block5dDiff (block 🖐️5d) | V5-ABSORB | same identified-delta merge (`dst.removed.extend` / `dst.added.extend`): create∘delete and patch∘delete produce unapplicable diffs. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:46` | Wfc2dDiff (wfc ◻️2d) | V5-ABSORB | base-free `merge_upserts`: a later remove of an id upserted earlier yields `removed X`; for a created X the base lacks X, so apply fails. The diff does not record create vs patch, so base-free absorb cannot cancel it. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:44` | Wfc3dDiff (wfc 🧊️3d) | V5-ABSORB | same `merge_upserts`: create∘delete yields an unapplicable `removed X`. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:64` | Grid2dDiff (wfc 🔲️grid2d) | V5-ABSORB | same `merge_upserts` for tiles and rules. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧱️grid3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:79` | Grid3dDiff (wfc 🧱️grid3d) | V5-ABSORB | same `merge_upserts` for tiles and rules. |
| `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs:68` | BitmapDiff (wfc 🖼️bitmap) | V5-ABSORB | `merge_pins` (same pattern as merge_upserts): create∘delete of a pin yields an unapplicable `removed`. `input_regions.extend` is ordered concatenation and is sound. |

No test covers this. The absorb law tests (`assert_mutation_diff_absorb_law`, 9 kinds) use only change and move kinds, not create or delete pairs.

## L3 law test status (V4)

- Per-kind fixture `inverse_restores_before` exists for 177 of 187 kinds. It applies the inverse mutations one by one through the central `apply_*_mutation` and compares to base. It is a sequential-chain check, not a diff sum.
- The shared harness `assert_mutation_inverse_law` (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs:572`) is used for 104 kinds. It is also sequential.
- `absorb` appears in tests only for forward diffs. No test sums inverse diffs with `absorb` (the only co-occurrences are imports), so the sum law `Sigma.apply(after) == base` is untested for all 187 kinds.
- V4-LAW-UNTESTED (no inverse test at all): 10 kinds, all inline editor kinds: `set-brush-preview` (block 3d), `change-active-tile` and `change-camera` (wfc 2d and 3d), `replace-config` and `set-solve` (wfc 2d and 3d), `set-solve` (bitmap).
- The 10 hand-written `Mutation` impls are not matched against any test. Their coverage is not verified.

## Framework-level observations (outside the plugin scope)

- L5 is not met by the trait. `MutationDiff::apply(&self, base)` is public (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:104`). No `ApplyCapability` is in that file. The repo-wide searches for `ApplyCapability` and for the policy gate `verify mutation-outcome-law` did not complete, so their absence repo-wide is not verified.
- L4 is met in scope for leaves. All 38 `.apply(` hits are in `🧪️tests` or in diff-type `apply` implementations. `&mut` in leaves is limited to local buffers (`paint-input-stroke`'s `visit`/`cells`).
- Tests call `MutationDiff::apply` directly (`mutation.diff(base).diff().apply(base)`). L4 allows this in tests, but it shows L5 is not enforced by the type system.
- `absorb` is base-free by contract (`📡️replication/🎮️mutation/🦀️.rs:103–119`). The id-keyed merges cannot cancel create-then-delete without recording whether an entry was created or patched, so this is a structural gap, not only a bug in each merge.
