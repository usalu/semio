# 📓️ exec-shooting-remodel-note

Status: Rust **WRITTEN BUT UNVERIFIED** (no green cargo result yet, see "Wave 3"). TypeScript twin (remodel) green under vitest; fixtures, JSON schemas and the mutation-outcome-law gate verified by scripts (below).

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

## Wave 3 and 4 (2026-10-08 evening)

Exact commands and counts (all from the repo root unless a `cd` is shown):

| Check | Command | Result |
|---|---|---|
| Remodel TS twin | `cd ✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript && bun ./📜️script.ts test` | **4 files, 1373/1373 tests passed** (98 fixture vectors x 13 assertions each: apply, byte-exact diff re-emit, outcome, lanes, round trips; plus shared vector, examples) |
| TS type check | `node_modules/.bin/tsc -p <scratch tsconfig>` over remodel `🧬️mutations/🟦️.ts`, `🚪️io/📝️text/🔺️diff/🔣️json/🟦️.ts`, note and shooting `🔺️diff/🟦️.ts` | 0 errors |
| Shooting TS facet vs fixtures | `bun 🗑️generated/shooting-remodel-note/parse_diffs.ts <🔺️diff/🟦️.ts> parseShootingDiff <fixtures>` | 31/31 parse |
| Note TS facet vs fixtures | same with `parseNoteDiff` | 18/34 parse; the other 16 are pre-existing: note's `noteNumber` wants `{bits}` while note fixtures carry plain floats (15) and one presence fixture is parsed with the wrong schema |
| JSON schemas vs fixtures | `jsonschema` Draft7 (`🗑️generated/shooting-remodel-note/validate_pair.py`) | shooting 31/31 doc + 6/6 config + 1/1 presence; note 33/33 + presence 1/1; remodel 98/98 |
| Gate | `bun ./📜️script.ts verify mutation-outcome-law` (about 4 min) | final run 22:50 against the newest gate script (stricter: every fn under `🧬️schema/**/🔺️diff` is scanned): 890 breaches repo-wide, **0 in shooting/remodel/note** |
| Rust | `🗑️generated/shooting-remodel-note/check-all.sh` through the gate (`cargo check --tests` per crate) | **BLOCKED on foundation** (see below) |

### What changed in wave 3 and 4

- **Remodel TS twin converted.** `ValueSpec` gained an internally tagged union kind (`tagged`) in the snapshot spec and both JSON codecs. `🔺️diff/🟦️.ts` rewritten (types, specs, builders, `applyRemodelingDiff` over rows/members/content/params/results, `RemodelingDiffApplyError`). `🧬️mutations/🟦️.ts` `remodelingMutationOutcome` now emits the same sparse rows as the Rust leaves (guard order and messages unchanged). Two stale fixtures fixed by hand: `🧫️fixtures/🏁️commit-reconstruction/*` (base64 chunks and float buffers to canonical octets and `inline`/`content` buffers) and the orphaned `update-camera` test (`🔍️refines2` had no Rust test; test dirs `🔍️refines`/`🔍️refines2` now point at their own fixtures, the stray `🔍️refines-the-9fd25a` copy is removed, the root module path updated).
- **Shooting schema** `🧬️schema/🔺️diff/🔣️.json` (and config/presence) rewritten by hand, schema-first, compact `type: [x, null]` form; `parseShootingDiff` added to the TS facet. Remodel `🔣️.json` was generated once by a scratch script in the ticket folder and is validated against all 98 fixtures; remodel GraphQL/proto rewritten by hand.
- **Frozen outcome codes**: apply codes renamed to the allowed vocabulary (`duplicate-target` to `mutation.apply.duplicate-id`, insertion `invalid-index` to `invalid-add-index`, `invalid-target`/truncation to `invalid-base`); every outcome code in the three plugins is one of the 11 frozen codes.
- **Diff text/binary I/O**: shooting and remodel `🚪️io/{📝️text,💾️binary}/🔺️diff` switched from `diff_text!`/`diff_binary!` (needs `DslRecord`, which sparse row diffs cannot derive) to the JSON/wire-value impls note already uses.
- **Gate R15**: 35 remodel leaves got their own `inverse_diffs_sum_to_the_negative_diff` test; `commit-reconstruction` got `an_applied_commit_inverts_to_the_negative_diff` (its fixtures are refusals only). Remodel `middle_member_inverses_restore_the_original_position` and note `middle_row_inverses_restore_the_original_index` added.
- **Gate R14**: persisted window configs no longer have a whole-snapshot `Snapshot` kind: remodel frames `SetFrameCursor`, report `SetReportTable`, model `SetCamera` + `SetLayers` (descriptor per variant; fixture `🔬️window-ownership` converted, `addressed_camera`/`addressed_layers` for the two model commands); note composite window config `SetCamera`. The note transient mutation (ephemeral lane, whole-root transfer) moved into `🪟️window/🫧️transient/🦀️.rs`.
- **Wave 4 (example/load = load)**: remodel `set-active-example` now emits only `reset_document_effect` (new in the editor root, `Effect::LoadDocument`); the example's frames travel as load content (asset handles plus durable artifacts) and `replace_document_operations`/`example_media_operations` are deleted. Note `set-active-example` and `load-document-json` already emit only `reset_document_effect` (confirmed, no mutation rows). Shooting `set-active-example`, `import-snapshot-json`, `reset-snapshot` emit only `reset_document_effect` (confirmed).
- Missing `pub` on `removal_roots`, `placements`, `dragged_positions` (sibling-module visibility) and a leftover `outcome.apply_to` in note's `apply_note_mutation_outcome` (now `store::apply_outcome`) fixed; these were the only errors in my code the compiler reached at 17:07.

### Rust verification status

- 11:25 to 17:10: `cargo check --tests` reached my crates only once (17:07, note): 9 errors, 6 mine (visibility, `apply_to`), all fixed; the other 3 and everything in shooting (root module aliases `op`/`pack`/`spr`, `semio_framework_plugin::{IoPayload,...}` removed, `default_snapshot` in a test) and remodel (stdio-semio image import/export) are pre-existing or peer-owned and untouched.
- Since 20:01 `foundation.status` is RED (value retirement and wgpu `close_step` API in flight), so per the build rule no cargo was run. Re-run when GREEN: `until head -1 "$T/🗑️generated/coord/foundation.status" | grep -q '^GREEN'; do sleep 60; done; "$T/🚦️gate.sh" shooting-remodel-note -- "$T/🗑️generated/shooting-remodel-note/check-all.sh"`; expect first-pass type errors in the new row/diff code that rustc could not reach before.

### Open issues after wave 3 and 4

1. Rust compile and every Rust test remain unrun (foundation RED).
2. Shooting plugin crate root still has stale `pilot_languages()` and removed `semio_framework_plugin` I/O names at HEAD (half-migrated I/O ticket, not mine).
3. Note TS facet floats are `{bits}` while note fixtures use plain floats (pre-existing).
4. Grammar/protocol files under `🚪️io/*/🔺️diff/` (shooting, remodel) still describe the old text/binary diff form; the codecs are JSON/wire-value now.

## Wave 5 (F-11, AMB-1, AMB-3, gate rules of 22:39)

- **Inverses read base, never simulate.** `negative`/`negative_list` are gone; each delta type has `inverse_rows` (shooting also `inverse_list`) that maps every row to its mirror by reading `base`: shooting list deltas, note block and asset rows, remodel keyed rows and content rows. Patch inverses are slot-wise reads from the base row (`ShootingPatchAlgebra<T>::restoring(&row)`, note `restoring(&block)`, remodel `restore_patch`). No `let mut ... = base.to_vec()/clone()` remains in an inverse.
- **Positional rows (AMB-1).** Shooting `remove` carries `index`, `move` carries `from`/`to`; note block `remove` carries `parentId`/`index`, `move` carries `fromParentId`/`fromIndex` plus the destination, table `removeRow`/`removeColumn` carry the removed cells (and column name). Apply refuses a row whose recorded position or content does not match (`mutation.apply.order-mismatch`). `between` for the ordered lists is built without simulation: rows outside the longest in-order subsequence leave from the back (base indices) and enter in the target order (final indices). Fixtures converted from their before-snapshots (shooting 6, note 5; `verify_shooting.py` 31/31, `verify_note.py` 33/34 with the known artifact), JSON schemas, TS/GraphQL/proto facets updated, absorb peepholes and unit tests adapted (move∘move merges to first source/last destination and vanishes when they meet; move∘remove removes from the first source).
- **Gate vocabulary.** Diff-type apply helpers renamed `apply_rows` (appliers are exempt), `negative` to `inverse_rows`; the apply wrappers (`apply_note_mutation(_outcome)`, `apply_remodeling_mutation`, `bridge_step`) moved from `🧬️schema/🧬️mutations/🦀️.rs` to `🧬️schema/🧬️mutations/🚪️io/🦀️.rs` (io boundary, re-exported under the same names); note's `sort_patch_runs(&mut ..)` call in the inverse replaced by a value-returning wrapper.
- **Limits of the per-row reads**: an inverse is exact for canonical diffs (one row per key; for content at most `[truncate, append]` per artifact; for note block patches and table edits no interleaving of rows that touch the same block). Diffs built by `absorb` and by the leaves are canonical.
- Still open: Rust not compiled (foundation RED at 22:41), so none of the Wave 3 to 5 Rust edits has met a compiler.
