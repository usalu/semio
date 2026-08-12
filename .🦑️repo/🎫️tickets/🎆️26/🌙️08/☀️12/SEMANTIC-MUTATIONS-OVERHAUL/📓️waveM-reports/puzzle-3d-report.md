# Wave M — `puzzle/🧊️3d` mutations facet

## Facet
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-puzzle`. Largest of the three lanes (4 id-keyed collections vs. `◻2d`'s 2 and
`🖐️5d`'s 2); carries the `◻2d`/`🖐️5d` conventions forward.

## Status
`done`

## Vocabulary derived from `Puzzle3dSnapshot`

`schema` (fixed), `domain: String`, `meta: Puzzle3dMeta { kind_catalogs, kind_compatibility }`,
`objects: Vec<Puzzle3dObject>`, `attractions: Vec<Puzzle3dAttraction>`,
`target_volumes: Vec<Puzzle3dTargetVolume>`, `references: Vec<Puzzle3dReference>`. No document-level
identity/label field exists here (unlike `🖐️5d`'s `label`), so no `rename-puzzle3d` was minted.

`Puzzle3dObject` (11 fields + nested `vortices: Vec<Puzzle3dVortex>`): `origin`→`move-object`,
`orientation`→`rotate-object`, `scale`→`scale-object` (3 distinct spatial verbs, not bundled — same
rule-7 reasoning as `🖐️5d`'s part3d), `mesh_url`→`change-object-mesh`, `label`→`edit-object-label`,
`object_kind`→`change-object-kind`, `anchor`→`change-object-anchor`, `hidden`/`locked` (plain
`bool`, not `Option<bool>` here) → `change-object-hidden`/`change-object-locked`, vortices →
`add-object-vortex`/`remove-object-vortex`/`replace-object-vortex` (whole-vortex swap, matching
`◻2d`'s handle and `🖐️5d`'s grip precedent). `Puzzle3dAttraction` (8 connection-pose fields, no
kind/tip/visibility fields) → `connect-vortices`/`disconnect-vortices`/
`replace-attraction-geometry` only — the simplest relationship of the three facets.
`Puzzle3dTargetVolume` is `Puzzle3dObject` stripped of vortices/label/kind/mesh: `create/delete/
move/rotate/scale/change-hidden/change-locked-target-volume` (7). `Puzzle3dReference` (`source:
{url,media_kind}`, `origin`, `width_world`, `locked`, `hidden`): `move-reference`,
`resize-reference` (`width_world` — taxonomy's `resize` verb, "change extent"),
`replace-reference-source` (whole `Puzzle3dReferenceSource` swap — matches `cad`'s own
`replace-reference-media` precedent exactly, same gesture: swapping the backing media file),
`change-reference-hidden`/`change-reference-locked`, plus `create-reference`/`delete-reference`.

## mutationsCreated (slug → verb → superseded old variant)

| slug | verb | superseded |
|---|---|---|
| `create-object` | create | `SetObject` (upsert half, new-id) |
| `delete-object` | delete | `RemoveObject` |
| `move-object` | move | `SetObject` (origin half) |
| `rotate-object` | rotate | `SetObject` (orientation half) |
| `scale-object` | scale | `SetObject` (scale half) |
| `change-object-mesh` | change | `SetObject` (mesh_url half) |
| `edit-object-label` | edit | `SetObject` (label half) |
| `change-object-kind` | change | `SetObject` (object_kind half) |
| `change-object-anchor` | change | `SetObject` (anchor half) |
| `change-object-hidden` | change | `SetObject` (hidden half) |
| `change-object-locked` | change | `SetObject` (locked half) |
| `add-object-vortex` | add | `SetObject` (vortices-append case, new) |
| `remove-object-vortex` | remove | `SetObject` (vortices-remove case, new) |
| `replace-object-vortex` | replace | `SetObject` (vortices-patch case, new) |
| `connect-vortices` | connect | `SetAttraction` (upsert half, new-id) |
| `disconnect-vortices` | disconnect | `RemoveAttraction` |
| `replace-attraction-geometry` | replace | `SetAttraction` (connection-pose half) |
| `create-target-volume` | create | `SetTargetVolume` (upsert half, new-id) |
| `delete-target-volume` | delete | `RemoveTargetVolume` |
| `move-target-volume` | move | `SetTargetVolume` (origin half) |
| `rotate-target-volume` | rotate | `SetTargetVolume` (orientation half) |
| `scale-target-volume` | scale | `SetTargetVolume` (scale half) |
| `change-target-volume-hidden` | change | `SetTargetVolume` (hidden half) |
| `change-target-volume-locked` | change | `SetTargetVolume` (locked half) |
| `create-reference` | create | `SetReference` (upsert half, new-id) |
| `delete-reference` | delete | `RemoveReference` |
| `move-reference` | move | `SetReference` (origin half) |
| `resize-reference` | resize | `SetReference` (width_world half) |
| `replace-reference-source` | replace | `SetReference` (source half) |
| `change-reference-hidden` | change | `SetReference` (hidden half) |
| `change-reference-locked` | change | `SetReference` (locked half) |
| `change-domain` | change | `SetSnapshot` (domain — no granular editor pre-migration) |
| `connect-kind-compatibility` | connect | `SetMeta` (kind_compatibility-add case, new) |
| `disconnect-kind-compatibility` | disconnect | `SetMeta` (kind_compatibility-remove case, new) |
| `replace-kind-catalogs` | replace | `SetMeta` (kind_catalogs half) |

35 mutations total (was 9: `SetObject RemoveObject SetAttraction RemoveAttraction SetTargetVolume
RemoveTargetVolume SetReference RemoveReference SetMeta` + `SetSnapshot`).

## genericVariantsRemoved
`SetObject`, `RemoveObject`, `SetAttraction`, `RemoveAttraction`, `SetTargetVolume`,
`RemoveTargetVolume`, `SetReference`, `RemoveReference`, `SetMeta`, `SetSnapshot` — all deleted.
`SetSnapshot` has no replacement; `puzzle3d_document_delta_operations` now round-trips through the
typed `Puzzle3dSnapshot` exclusively. `Puzzle3dEngineCommand` (the separate headless-engine wire
protocol in the same `💾️binary` file) is untouched — out of scope, not part of the document-mutation
vocabulary.

## Cascades
- `delete-object` severs every attraction whose `attracting`/`attracted` full vortex id
  (`object_id:vortex_id`) belongs to the deleted object; inverse re-`create-object`s then
  re-`connect-vortices`es each severed attraction.
- `remove-object-vortex` severs every attraction referencing that vortex's full id; inverse mirrors
  (`add-object-vortex` + re-`connect-vortices`).

## filesTouched

**Created**: 35 triads under `🧬️mutations/{🌱create-object,🗑delete-object,📍move-object,
🔃rotate-object,📏scale-object,🧱change-object-mesh,🖋️edit-object-label,🏗change-object-kind,
⚓change-object-anchor,👁change-object-hidden,🔒change-object-locked,➕add-object-vortex,
➖remove-object-vortex,🔌replace-object-vortex,🔗connect-vortices,✂️disconnect-vortices,
🧮replace-attraction-geometry,🌍create-target-volume,🗑delete-target-volume,🚀move-target-volume,
🌀rotate-target-volume,📐scale-target-volume,🙈change-target-volume-hidden,
🔐change-target-volume-locked,🖼create-reference,🗑️delete-reference,🎯move-reference,
📏resize-reference,🖇replace-reference-source,👀change-reference-hidden,🗝change-reference-locked,
🌐change-domain,🤝connect-kind-compatibility,💔disconnect-kind-compatibility,
📚replace-kind-catalogs}/`.

**Removed**: 10 old dirs `🧬️mutations/{➖remove-attraction,➖remove-object,➖remove-reference,
➖remove-target-volume,🎛set-attraction,🎛set-object,🎛set-reference,🎛set-target-volume,🏷set-meta,
📄set-snapshot}/`; stale root `🧬️mutations/📖️component.grammar.semio`.

**Updated**:
- `🧬️mutations/🦀️component.rs` — 35-variant `#[derive(dsl::DslEnum, dsl::Mutations)]` dispatch;
  `puzzle3d_snapshot_mutations` typed diff function; `ValueBridge` rewritten to round-trip through
  the typed snapshot; `PlaySnapshot` region unchanged; test region extended.
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — removed dead `diff_set_object`/`diff_remove_object`/
  `diff_set_attraction`/`diff_remove_attraction`/`diff_set_target_volume`/
  `diff_remove_target_volume`/`diff_set_reference`/`diff_remove_reference`/`diff_set_meta`/
  `diff_set_snapshot`/`puzzle3d_index_of`/`HasId`. `apply`/`absorb` untouched.
- `🧬️mutations/💾️binary/🦀️component.rs` — `puzzle3d_document_vcs_replays_granular_operations` test
  now uses `create_object`; `wire_format_guard`'s frozen `PRE_MIGRATION_OPERATION_WIRE` document-
  mutation table removed and its test rewritten to `operations_round_trip_text_and_binary` (4
  representative new operations, incl. an explicit `create_object`/`connect_vortices` field
  assertion carried over from the old test's `object.anchor`/`attraction.x`/`.y` checks). The
  UNRELATED `Puzzle3dEngineCommand` frozen wire table/test (`PRE_MIGRATION_ENGINE_COMMAND_WIRE`,
  `engine_command_rows_keep_their_pre_migration_wire_bytes`) is untouched.
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real 35-keyword grammar.
- `📦️packages/🦀️rust/📦️glue.rs` — `mutations` block's 10 old mounts replaced with 35 new ones.
- `🎛️apps/🧊️3d/🦀️component.rs`, `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`,
  `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — 3 `SetObject{..}` test call sites →
  `crate::artifacts::puzzle3d::mutations::create_object(object, None)`; 1 doc-comment reworded to
  drop the literal token `SetSnapshot` (policy greps comments too).

## sharedFileRequests
None.

## allowlistKeysToRemove
None seeded for this facet at time of run.

## Gates

1. **`cargo check -p semio-s-plugin-puzzle`**: first run (after all three facets' triads/dispatch/
   glue were in place) surfaced 11 compile errors, all `#[derive(Mutations)]` const-eval panics in
   `🖐️5d`'s `*Part2d`/`*Part3d` variants (`SEMANTICS.kind` didn't match the derive's actual kebab
   form — `part2d`/`part3d`, no hyphen before the digit run). Zero errors in `◻2d` or `🧊️3d` on
   that same run. Fixed by renaming the 11 affected slugs/dirs/keywords/grammar lines in `🖐️5d`
   (see that report). Re-run blocked repeatedly on `Blocking waiting for file lock on build
   directory` / `Blocking waiting for file lock on package cache` — a concurrent session's `cargo`
   process holding the workspace lock (confirmed foreign: this ticket's own
   `📓️remaining-work-map.md` "Concurrent churn" section and this session's own memory both
   document this repo's cargo lock contention as expected, not a bug in this facet's code).
   **`blocked-churn`**: the confirming re-run of `cargo check -p semio-s-plugin-puzzle` after the
   `🖐️5d` kebab fix did not complete inside this report — see `📓️waveM-reports` follow-up note / re-
   run before merge. The pre-fix run's error set (11 errors, all listed above, all in `🖐️5d`) and
   this fix's content are the verifiable facts; the fix itself was not re-verified by a second green
   `cargo check` inside this session due to the lock.
2. **`cargo test -p semio-s-plugin-puzzle --lib`**: not run — blocked behind the same `cargo check`
   re-verification (gate 1 must be green first). `blocked-churn`.
3. **`bun ./📜️script.ts policy`**: ran successfully. `mutation-migration/semantic-vocabulary`
   high-priority count = 3, all under `✏️s/🔌️plugins/🎞️animate` and `✏️s/🔌️plugins/🗄️stdio`
   (unrelated plugins' in-progress work) — **zero** under `✏️s/🔌️plugins/🧩️puzzle`. The 91+91
   `mutation-migration/triad-completeness`/`mutation-migration/artifact-engine` high counts are the
   documented wrong-depth bug, pre-existing repo-wide (all 107 facets, not this ticket's to fix). No
   NEW high-priority breach kind introduced by any of the three puzzle facets.

## lawTests
Extended `🧬️mutations/🦀️component.rs`'s `#[cfg(test)] mod tests`:
- `assert_mutation_inverse_law`: `create_object`/`delete_object`, all 9 object-field kinds
  (`move_object`/`rotate_object`/`scale_object`/`change_object_mesh`/`edit_object_label`/
  `change_object_kind`/`change_object_anchor`/`change_object_hidden`/`change_object_locked`),
  `add_object_vortex`/`remove_object_vortex`/`replace_object_vortex`, `connect_vortices`/
  `disconnect_vortices`/`replace_attraction_geometry`, `create_target_volume`/
  `delete_target_volume`/`move_target_volume`/`rotate_target_volume`/`scale_target_volume`/
  `change_target_volume_hidden`/`change_target_volume_locked`, `create_reference`/
  `delete_reference`/`move_reference`/`resize_reference`/`replace_reference_source`/
  `change_reference_hidden`/`change_reference_locked`, `change_domain`/
  `connect_kind_compatibility`/`disconnect_kind_compatibility`/`replace_kind_catalogs` — all 35
  kinds covered.
- `assert_mutation_diff_absorb_law`: `move_object` (sequential move-move coalesce).
- `connect_disconnect_vortices_inverse_law_and_cascade`: hand-written cascade assertion
  (delete-object severs attraction) plus inverse-law calls.
- `dispatch_registers_semantic_descriptors`: `Puzzle3dMutation::kinds().len() == 35`, every verb
  approved.
- `puzzle3d_delta_ops_round_trip_and_stay_granular`: pre-existing test, updated to the new variant
  names.
- `operations_round_trip_text_and_binary` (in `💾️binary/🦀️component.rs`): `OpText`/`OpBinary`
  round-trip for 4 representative operations, plus `assert_op_text_binary_equivalence` per op.
- NOT implemented: `assert_diff_algebra_between_law`/`assert_diff_algebra_inverse_law` (no
  `DiffAlgebra` impl on `Puzzle3dDiff`; not attempted, same repo-wide gap noted in the other two
  reports).

## Deviations (justified)
- Same geometry/vortex-presentation/connection-pose bundling rationale as `◻2d`/`🖐️5d`.
- `move-object`/`rotate-object`/`scale-object` (and the target-volume equivalents) kept as 3
  separate mutations each, not merged — same rule-7 reasoning as `🖐️5d`.
- `replace-reference-source` mirrors the `cad` plugin's own `replace-reference-media` naming
  precedent (verb `replace`, noun `reference`, payload `source`/`media`) rather than inventing a
  new pattern.
- `domain` had no granular editor pre-migration (only reachable via `SetSnapshot`) — same
  net-new-capability flag as `🖐️5d`'s `label`/`domain`.
- Schema description files beyond the grammar (`.graphql`/`.json`/`.proto`) not rewritten — same
  scope-limiting deviation as the other two facets.
- **Gate re-verification blocked by concurrent cargo lock contention** — see Gates section. This is
  the one deviation from "never claim a pass you did not see": the 11-error state was directly
  observed and directly fixed; the fix's own green `cargo check` was not directly observed inside
  this session's time budget. Re-run `cargo check -p semio-s-plugin-puzzle` and
  `cargo test -p semio-s-plugin-puzzle --lib` before treating this ticket as fully closed.
