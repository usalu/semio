# 📓️ exec-shooting-remodel-note

Status: **WRITTEN BUT UNVERIFIED** (no Rust compile or test result yet, see "Test status"). Fixtures, JSON schemas and the python oracles are verified by independent scripts (below).

## Design applied (all three plugins)

- Every diff is a sparse typed delta. No whole-entity/snapshot/list carrier remains. `between(` and `.apply(` are gone from leaves; every non-leaf apply site calls `protocol::apply_diff`.
- Optional-slot assignments use `{value: T}` wrappers (`ShootingAssigned`, `NoteAssigned`, `RemodelingAssigned`) so `Some(None)` survives the wire.
- Ordered row sequences (shooting edits + keyed patches, note block rows, remodel keyed rows + content rows) carry `absorb` that coalesces same-key rows (add∘remove cancels, patch∘patch merges per field, move∘move keeps the last, remove∘add stays a replace) via an adjacent-row peephole, with canonical key order so the sum equals the negative diff.
- Each diff type has a concrete `DiffAlgebra::inverse(base)` (negative diff) plus `between`/`is_empty`.
- Leaf inverses are concrete mutation lists built from payload + base, replayed in reverse; they sum (via `absorb`) to the negative diff. A per-leaf `inverse_diffs_sum_to_the_negative_diff` test (shooting 31, note 33) calls `assert_mutation_inverse_sum_law`; remodel is covered by a `laws()` helper (inverse law + sum law) in the mutations unit test and by the window test.
- Wave-2 ruling (original index): delete/remove inverses reinsert at the ORIGINAL index (shooting `create-*` and note `create-block` carry optional `index`, absent = append). Middle-row law tests: shooting `deleting_a_middle_row_is_undone_in_place`, `create_at_an_index_inserts_there`; note `middle_row_inverses_restore_the_original_index`; remodel `middle_member_inverses_restore_the_original_position` (collections are key-ordered, so the key position IS the original position). `replace-<entity>` inverting to the same kind with the base value is used for replace-presence.

## Per kind: before -> after

### Shooting (31 kinds + config/presence)
| Kinds | Before | After |
|---|---|---|
| create-asset, create-shot, create-saved-camera | whole-list carrier | `ShootingEdit::Add{index,item}` row; inverse = delete |
| delete-asset, delete-shot, delete-saved-camera | whole-list carrier | `Remove{id}` row; inverse = create at original index |
| reorder-assets/shots/saved-cameras | whole-list carrier | `Move{id,index}` rows; inverse = moves back |
| rename-asset, change-asset-url, drag-assets, rotate-assets, scale-assets | clone-and-write of assets | keyed `ShootingAssetPatch` (typed Option fields, `ShootingAssigned` for orientation/scale); inverse = patch restoring named slots (gestures not bit-exact for unset orientation/scale, see open issues) |
| rename-shot, change-shot-width/height/format/shape | clone-and-write | keyed `ShootingShotPatch`; inverse = restoring patch |
| rename-saved-camera, replace-shot-camera | clone-and-write | keyed `ShootingSavedCameraPatch`/`ShootingShotPatch.camera_id`; replace-shot-camera gained a missing-camera error and a no-op warning |
| change-scene (+ ambient, sun x4, shadow = 7 kinds) | whole scene record | `ShootingScenePatch` (17 optional fields); inverse = restoring scene patch |
| set-active-shot, set-active-asset | scalar | scalar assignment, inverse = previous id |
| Editor config (6 leaves) | whole `ShootingConfig` record | new `ShootingConfigDiff` (sparse); per-kind setter inverses; ReplaceConfig inverse = list of setters; no-op warnings |
| Presence (replace-presence) | whole record | new `ShootingPresenceDiff`; inverse = same kind with base value |

### Note (33 leaves + window/presence)
| Kinds | Before | After |
|---|---|---|
| create-block, delete-block, delete-blocks, move-block-to-container, drag-blocks, duplicate-block(s), move-block | whole block-list / subtree carrier | `NoteBlockRow` rows (`add{parentId,index,block}`, `remove`, `move`, `patch`); inverses use `removal_roots`, `dragged_positions`, `placements`, original index restored |
| rename-block, change-block-visible/locked, resize-block, change-block-font-size | clone-and-write | `NoteBlockPatch` sparse slots; restoring patch |
| edit-block-text, edit-block-math, ink stroke/width | whole child replaced | typed `NoteBlockPatch` slots (content, tex, displayMode, points, strokeWidth, color) |
| table x4 | whole grid | `NoteTableEdit` ordered ops (insert/remove row/column; payload gained optional `cells`/`name` so the inverse is concrete) |
| assets x3 | whole map | `NoteAssetRow` insert/replace/remove, `coalesce_asset_rows` |
| canvas x6, rename-note, pencil, eraser | scalar in snapshot | `NoteAssigned<T>` scalar slots |
| Presence / window config+transient | whole-state `MutationDiff` impls | `NotePresenceDiff`, `NoteCompositeWindowConfigDiff`/`NoteCompositeWindowTransientDiff` (sparse, no-op warnings, ConfigRecord) |

### Remodel (36 kinds + 3 window configs)
| Kinds | Before | After |
|---|---|---|
| create/delete stream, camera, rig extrinsic, GCP, asset | whole-lane list/map carrier | `RemodelingRows` keyed rows (`insert`/`replace`/`remove`/`patch`, key-ordered) |
| change-stream-sync, replace-stream-source, add/remove stream frame, add/remove GCP observation | whole stream/GCP | `MediaStreamPatch`/`GroundControlPointPatch` (`RemodelingMembers{removed,added}` multiset with rank order) |
| update-*-params x8 | whole `ReconstructionParams` | `RemodelingParamsDiff` per-facet slot (macro `facet_algebra!`) |
| replace sparse/dense/mesh/trajectory/tracks/geo/qc | whole results | `RemodelingResultsDiff` per-slot (`RemodelingAssigned` for optionals) |
| append-content, remove-content | whole durable store | `RemodelingContentRow` `append{header?}` / `truncate{from}`; inverse = truncate/append |
| commit-reconstruction | results + assets + content | results slots + asset rows via `rebind_asset`; inverse rewritten |
| Window configs (frames, report, model) | whole-record impls | sparse `*ConfigDiff` structs, `ConfigRecord` impls; ownership test generalized + sum law |

## Files touched (grouped)

- Shooting: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/` crate root `🦀️.rs`; subset `🏅️standards/🔖️1/🪆️subsets/✳️any/`: `🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto,🧪️tests}`; all 31 `🧬️schema/🧬️mutations/*/{🔺️diff,↩️inverse,🧪️tests}`; `🧬️mutations/🧪️tests/🔬️unit`; `🚪️io/🦀️.rs`; `✏️editor/🎚️config/**` (new `🧬️schema/🔺️diff`), `✏️editor/👥️presence/**`; 31 `🧫️fixtures/🧬️mutations/*/*/🔺️diff/🔣️.json` + config/presence fixtures; python oracle `🧪️tests/🎥️mutate-shooting-1/🐍️.py`.
- Note: subset `✳️any/🧬️schema/{🦀️.rs,🔺️diff/**,🧬️mutations/🦀️.rs}`; leaves under `🧱️block`, `📝️text`, `🧮️math`, `🖋️ink`, `📊️table`, `🖼️asset`, `🎨️canvas`, `📜️document`, `✳️any`; `✳️any/✏️editor/👥️presence/**`, `✏️editor/🪟️window/{🦀️.rs,🔺️diff/**}`; 34 fixtures; payload JSON schemas for table leaves; `🔺️diff/{🔣️.json,🟦️.ts,🔗️.graphql,🛰️.proto}`.
- Remodel: subset `✳️any/🧬️schema/🔺️diff/{🦀️.rs,🔣️.json,🔗️.graphql,🛰️.proto,🧪️tests}`; all 36 `🧬️mutations/*/{🔺️diff,↩️inverse}`; `🧬️mutations/🦀️.rs` (`rebind_asset`), `🧬️mutations/🧪️tests/🔬️unit`; `🚪️io/🦀️.rs` + io test; three window config files + window-ownership test; 98 fixtures.
- Scratch (inside ticket): `🗑️generated/shooting-remodel-note/*` (fixture converters, verifiers, schema generators). Disposable.

## Verification done (no cargo)

- `rustfmt` parse of all 181 changed `.rs` files in the three plugins: no syntax errors.
- Independent Python simulators replay each converted fixture diff onto its before-snapshot and compare to the after-snapshot: shooting 31/31, remodel 98/98, note 33/34 (the 34th mismatch is a verifier artifact of null vs absent).
- JSON schemas validated against committed diff fixtures with `jsonschema` Draft7: shooting 39/39 (artifact, config, presence), note 36/36 (+presence), remodel 98/98 (the only misses are other schemas picked up by the glob).
- Python oracles (`mutate-shooting-1`, note `mutate-note-1-*`, remodel `mutate-remodeling-1`) operate on snapshots; only shooting's changed (create index honored, insert helper). Note/remodel oracles unaffected (table payload additions are optional).

## Test status

- `cargo check --tests` for the three crates: **not yet green or red for my code**. Every attempt stops earlier in `semio-framework-replication` (peer in-progress change, `RetainedCloneGrant` vs `Grant` mismatches in `📡️wire/🏠️local-interaction/🌳️root/…`), so none of my crates was reached. Command: `cd <crate dir> && "$T/🚦️gate.sh" shooting-remodel-note -- cargo check --tests --message-format=short`, output in `🗑️generated/shooting-remodel-note/*-check.txt`. Attempts: 11:25 (killed 137, memory), 11:58 and 12:13 (same 12 replication errors, shooting and note; remodel identical by construction). Re-run with `🗑️generated/shooting-remodel-note/check-all.sh` through the gate once `semio-framework-replication` compiles again; expect first-pass compile errors in my crates since nothing has been compiled yet.
- Tests written, not run: shooting unit (edits order, malformed rows, absorb coalescing, absorb==sequential, negative restores, between, middle row, create at index, gestures sum law), 31 per-leaf sum-law tests, config/presence sum-law + no-op tests; note law tests for diff algebra + per-leaf sum-law (33) + middle-row; remodel `laws()` unit tests incl. middle-member, window ownership sum-law.

## Open issues

1. Remodel TypeScript twin NOT converted: `🧬️schema/🔺️diff/🟦️.ts` (`REMODELING_DIFF_SPEC`, `applyRemodelingDiff`), `🧬️schema/🧬️mutations/🟦️.ts` (`remodelingMutationOutcome`), `🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts` and the vitest oracle `🧬️schema/🧪️tests/🧩️suite/🟦️.ts` still implement the old whole-lane diff and will fail against the new fixtures. It needs a tagged-union `ValueSpec` kind (the codec has none) plus a rows applier; a separate wave.
2. Note TS facet `🔺️diff/🟦️.ts` was rewritten by hand against `../🟦️.ts` helpers but is untyped-checked (no tsc run).
3. Gesture inverses (drag/rotate/scale assets) are exact on the diff algebra but cannot restore an unset (`None`) orientation/scale through the gesture kinds themselves; no absolute-setter kind was added.
4. `replace-presence` inverts to the same kind with the base value (accepted by the wave-2 ruling).
5. Remodel create/delete-asset inverse needs the previous content complete in the durable store.
6. Note `duplicate-blocks` inverse is order-sensitive for non-ascending payloads (payload order, base index + 1 as before).
7. Remodel diff fixtures at HEAD were stale (plain floats vs `{bits}` snapshots); regenerated from after-snapshots.
8. JSON diff schemas now emit explicit `null` for unset optional slots (matches the existing fixture convention; schemas made nullable accordingly).
9. Auto-formatter may rewrite touched files concurrently.
