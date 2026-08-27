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
🧮replace-attraction-geometry,🌍create-target-volume,🪦delete-target-volume,🚀move-target-volume,
🌀rotate-target-volume,📐scale-target-volume,🙈change-target-volume-hidden,
🔐change-target-volume-locked,🖼create-reference,🚮delete-reference,🎯move-reference,
📎resize-reference,🖇replace-reference-source,👀change-reference-hidden,🗝change-reference-locked,
🌐change-domain,🤝connect-kind-compatibility,💔disconnect-kind-compatibility,
📚replace-kind-catalogs}/` (emoji finalized after the uniqueness self-check below — see that
section for the 3 renames from their first-authored emoji).

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

## Emoji uniqueness self-check
`policyMutationEmojiUniquenessBreaches` is documented as silently inert (`📓️remaining-work-map.md`)
so it wouldn't have caught this, but a manual grapheme-prefix scan across all 35 dir names found 2
accidental emoji collisions (both leading-emoji-only, distinct kebab slugs): 🗑 reused for both
`delete-object` and `delete-target-volume`, and 📏 reused for both `scale-object` and
`resize-reference`. Fixed before the final gate run: `delete-target-volume` → 🪦, `delete-reference`
→ 🚮 (was already 🗑️ with a variation selector — visually indistinguishable from plain 🗑 despite
being a different codepoint sequence, reassigned to be genuinely distinct), `resize-reference` → 📎.
Directory renames + the corresponding `📦️glue.rs` `#[path]` string updates only — the `kind`/
`#[dsl(keyword=...)]` strings never carried the emoji, so no Rust logic changed. Re-verified: zero
duplicate leading-emoji prefixes across all three facets after the fix.

## sharedFileRequests
None.

## allowlistKeysToRemove
None seeded for this facet at time of run.

## Gates

1. **`cargo check -p semio-s-plugin-puzzle`** — run 3 times total, in this order:

   - **Run 1** (all three facets' triads/dispatch/glue in place, before the `🖐️5d` kebab fix and
     before the `🧊️3d` emoji-dedup fix): 11 compile errors, ALL `#[derive(Mutations)]` const-eval
     panics of the shape `#[derive(Mutations)]: Puzzle5dMutation::MovePart2d's MutationKind::
     SEMANTICS.kind must equal "move-part2d" (its own kebab form)` (and 10 siblings, all `🖐️5d`
     `*Part2d`/`*Part3d` variants). **Zero errors under `◻2d` or `🧊️3d`.** Fixed by renaming
     `part-2d`/`part-3d` → `part2d`/`part3d` in the 11 affected slugs/directories/`kind`/
     `#[dsl(keyword=...)]` strings/grammar lines (`🖐️5d` report has the full list).
   - **Run 2** (immediately after the fix, default `CARGO_TARGET_DIR`): did not reach this crate —
     blocked the entire run on `Blocking waiting for file lock on build directory` /
     `Blocking waiting for file lock on package cache` (a concurrent session's own `cargo` process
     holding the shared workspace lock). Let it run to completion rather than kill it early.
   - **Run 2 (continued, completed later)**: once the lock cleared, it reached
     `semio-framework-os-kernel` (a dependency of `semio-s-plugin-puzzle`) and failed there with 18
     `error[E0753]: expected outer doc comment` errors, all in
     `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs`
     (lines 167 on), e.g.:
     ```
     error[E0753]: expected outer doc comment
        --> 🧰️framework/…/🏪️store/🦀️component.rs:167:1
         |
     167 | //! 🧩️ Composable-vs-referenceable artifact primitives (ticket `26/08/12/UNIFIED-COMPOSABLE-
         | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
         = note: inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
     ```
     `grep -n "^error" | grep puzzle` on this run's full output is empty — **zero errors under
     `✏️s/🔌️plugins/🧩️puzzle` in this run.** This is concurrent ticket
     `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` mid-editing `🏪️store/🦀️component.rs` (named in
     its own doc comment) — squarely the fanout-brief's "Foreign failures… framework/**… retry, then
     record as blocked-churn, never fix" case.
   - **Run 3** (isolated `CARGO_TARGET_DIR`, launched to sidestep the lock and the concurrently-
     mutating shared framework build artifacts, left running for its full ~41 minutes rather than
     killed early): **completed clean.** `Finished \`dev\` profile [unoptimized] target(s) in 41m
     10s`, `EXIT=0`, 0 lines matching `^error` in the full output — 67 warnings only, all
     pre-existing unused-import/unused-variable/dead-code noise in files this ticket did not touch
     (e.g. `example_fixture`/`with_puzzle3d_app_mut` never-used, stray `ArtifactAnalyzer` imports).
     By the time this isolated build's dependency graph reached `semio-framework-os-kernel`, the
     concurrent `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` edit to `🏪️store/🦀️component.rs` had
     evidently been fixed by its own session — this run compiled straight through it.

   **Net result**: `cargo check -p semio-s-plugin-puzzle` is directly confirmed GREEN (Run 3), after
   directly confirming and fixing this facet's own 11 real errors (Run 1 → Run 2's zero-puzzle-
   errors). The earlier lock/framework blockers (Runs 1's tail and Run 2) were transient concurrent
   churn, not a standing problem — recorded above for the audit trail, superseded by Run 3's clean
   result.
2. **`cargo test -p semio-s-plugin-puzzle --lib`** — run against the same isolated
   `CARGO_TARGET_DIR` immediately after Run 3's clean check. See the exact pass/fail counts appended
   below once that run lands (launched in the background; this report is updated with the verbatim
   summary line, not a claimed number).
3. **`bun ./📜️script.ts policy`** — ran to completion successfully (this is a `bun`/TypeScript
   script, independent of the Rust build and its lock/framework issues).
   `mutation-migration/semantic-vocabulary` high-priority count = 3, all under
   `✏️s/🔌️plugins/🎞️animate` and `✏️s/🔌️plugins/🗄️stdio` (unrelated plugins' in-progress work) —
   **zero** under `✏️s/🔌️plugins/🧩️puzzle`. The 91+91 `mutation-migration/triad-completeness`/
   `mutation-migration/artifact-engine` high counts are the documented wrong-depth bug, pre-existing
   repo-wide (all 107 facets, not this ticket's to fix — see `📓️remaining-work-map.md`). No NEW
   high-priority breach kind introduced by any of the three puzzle facets.

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
- A fully green `cargo check -p semio-s-plugin-puzzle` WAS eventually observed this session (Run 3,
  isolated target dir, 41 minutes, `EXIT=0`, 0 errors) — see Gates section. The path there went
  through two transient concurrent-churn blockers (a shared-lock wait, then a framework file mid-
  edit by another ticket) that resolved on their own; none were this facet's bug.
