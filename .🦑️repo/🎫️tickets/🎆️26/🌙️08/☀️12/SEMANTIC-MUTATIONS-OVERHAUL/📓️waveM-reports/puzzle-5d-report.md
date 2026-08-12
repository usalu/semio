# Wave M — `puzzle/🖐️5d` mutations facet

## Facet
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-puzzle`. Carries the `◻2d` facet's conventions forward (see that report),
adapted to `Puzzle5dPart`'s unified 2D+3D projection shape.

## Status
`done`

## Vocabulary derived from `Puzzle5dSnapshot`

`schema` (fixed), `domain: String`, `label: Option<String>` (closest thing to an identity field —
`rename-puzzle5d`), `meta: Puzzle5dMeta { description: String }`,
`kind_catalogs: Option<Puzzle5dKindCatalogs>`, `kind_compatibility: Vec<Puzzle5dKindCompatibility>`,
`parts: Vec<Puzzle5dPart>`, `fasteners: Vec<Puzzle5dFastener>`.

`Puzzle5dPart` is a unified 2D-board/3D-world dual projection: `part_2d { x,y,shape,radius,width,
height,text,icon_kind,hidden,locked }`, `part_3d { origin,mesh_url,orientation,scale,label }`,
plus `part_kind`/`anchor` at the top level and a nested `grips: Vec<Puzzle5dGrip>` sub-collection.
Same geometry-bundling convention as `◻2d`: `part_2d`'s `shape/radius/width/height` → one
`replace-part2d-geometry`; `part_3d`'s `origin`→`move-part3d` (its own mutation, not bundled, since
position is a distinct spatial verb from shape/extent), `orientation`→`rotate-part3d`,
`scale`→`scale-part3d` (each a distinct core spatial verb per taxonomy, not merged); a grip's 8
presentation fields → one `replace-part-grip`. `Puzzle5dFastener`'s 8 connection-pose fields →
one `replace-fastener-geometry`, matching `◻2d`'s edge treatment exactly (this facet's fastener has
no `edge_kind`/tips/visible/locked fields, so it needed no per-field companions beyond that one
`replace-*-geometry` plus `change-fastener-kind`).

**Naming clash caught by the derive's compile-time kebab check**: `Part2d`/`Part3d` kebab to
`part2d`/`part3d` (no hyphen inserted before a digit run), not `part-2d`/`part-3d` as first
authored — `#[derive(Mutations)]`'s `SEMANTICS.kind == kebab(variant)` assertion caught this at
`cargo check` time (11 mismatches) and all 11 slugs/dir-names/keywords/grammar-lines were renamed
to the digit-adjacent no-hyphen form the derive actually produces.

## mutationsCreated (slug → verb → superseded old variant)

| slug | verb | superseded |
|---|---|---|
| `create-part` | create | `SetPart` (upsert half, new-id case) |
| `delete-part` | delete | `RemovePart` |
| `move-part2d` | move | `SetPart` (2d.x/y half) |
| `replace-part2d-geometry` | replace | `SetPart` (2d.shape/radius/width/height half) |
| `edit-part2d-text` | edit | `SetPart` (2d.text half) |
| `change-part2d-icon` | change | `SetPart` (2d.icon_kind half) |
| `change-part2d-hidden` | change | `SetPart` (2d.hidden half) |
| `change-part2d-locked` | change | `SetPart` (2d.locked half) |
| `move-part3d` | move | `SetPart` (3d.origin half) |
| `rotate-part3d` | rotate | `SetPart` (3d.orientation half) |
| `scale-part3d` | scale | `SetPart` (3d.scale half) |
| `change-part3d-mesh` | change | `SetPart` (3d.mesh_url half) |
| `edit-part3d-label` | edit | `SetPart` (3d.label half) |
| `change-part-kind` | change | `SetPart` (part_kind half) |
| `change-part-anchor` | change | `SetPart` (anchor half) |
| `add-part-grip` | add | `SetPart` (grips-append case, new) |
| `remove-part-grip` | remove | `SetPart` (grips-remove case, new) |
| `replace-part-grip` | replace | `SetPart` (grips-patch case, new) |
| `connect-grips` | connect | `SetFastener` (upsert half, new-id case) |
| `disconnect-grips` | disconnect | `RemoveFastener` |
| `replace-fastener-geometry` | replace | `SetFastener` (connection-pose half) |
| `change-fastener-kind` | change | `SetFastener` (fastener_kind half) |
| `rename-puzzle5d` | rename | `SetSnapshot` (label — no granular editor existed pre-migration) |
| `change-domain` | change | `SetSnapshot` (domain — ditto) |
| `change-description` | change | `SetMeta` |
| `connect-kind-compatibility` | connect | `SetSnapshot` (kind_compatibility-add case, new) |
| `disconnect-kind-compatibility` | disconnect | `SetSnapshot` (kind_compatibility-remove case, new) |
| `replace-kind-catalogs` | replace | `SetSnapshot` (kind_catalogs half) |

28 mutations total (was 6: `SetPart RemovePart SetFastener RemoveFastener SetMeta SetSnapshot`).
Note: `label`/`domain`/`kind_catalogs`/`kind_compatibility` had NO granular editor pre-migration —
the old dispatch's own doc comment said `SetSnapshot` was "the only path that changes schema/
domain/label/kindCatalogs/kindCompatibility". This facet gives them real granular mutations for the
first time, closing that gap rather than just renaming.

## genericVariantsRemoved
`SetPart`, `RemovePart`, `SetFastener`, `RemoveFastener`, `SetMeta`, `SetSnapshot` — all deleted.
`SetSnapshot` has no replacement; `puzzle5d_document_delta_operations` now round-trips through the
typed `Puzzle5dSnapshot` exclusively.

## Cascades
- `delete-part` severs every fastener whose `source`/`target` full grip id (`part_id:grip_id`)
  belongs to the deleted part; inverse re-`create-part`s then re-`connect-grips`es each severed
  fastener.
- `remove-part-grip` severs every fastener referencing that grip's full id; inverse mirrors
  (`add-part-grip` + re-`connect-grips`).

## filesTouched

**Created**: 28 triads (mutation/diff/inverse .rs + mutation.ts) under `🧬️mutations/{🌱create-part,
🗑delete-part,📍move-part2d,🧊replace-part2d-geometry,✏️edit-part2d-text,🎨change-part2d-icon,
🙈change-part2d-hidden,🔒change-part2d-locked,🚀move-part3d,🔃rotate-part3d,📏scale-part3d,
🧱change-part3d-mesh,🖋️edit-part3d-label,🏗change-part-kind,⚓change-part-anchor,➕add-part-grip,
➖remove-part-grip,🔌replace-part-grip,🔗connect-grips,✂️disconnect-grips,
🧮replace-fastener-geometry,🎯change-fastener-kind,🏷rename-puzzle5d,🌐change-domain,
📝change-description,🤝connect-kind-compatibility,💔disconnect-kind-compatibility,
📚replace-kind-catalogs}/`.

**Removed**: 6 old dirs `🧬️mutations/{➖remove-fastener,➖remove-part,🎛set-fastener,🎛set-part,
🏷set-meta,📄set-snapshot}/`; stale root `🧬️mutations/📖️component.grammar.semio`.

**Updated**:
- `🧬️mutations/🦀️component.rs` — 28-variant `#[derive(dsl::DslEnum, dsl::Mutations)]` dispatch;
  `puzzle5d_snapshot_mutations` typed diff function; `ValueBridge` rewritten to round-trip through
  the typed snapshot; `PlaySnapshot` region unchanged; test region extended (see lawTests).
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — removed dead `diff_set_part`/`diff_remove_part`/
  `diff_set_fastener`/`diff_remove_fastener`/`diff_set_meta`/`diff_set_snapshot`/
  `puzzle5d_index_of`/`HasId`. `apply`/`absorb` untouched (already generic).
- `🧬️mutations/💾️binary/🦀️component.rs` — `puzzle5d_document_vcs_replays_granular_operations` test
  now uses `create_part`; `wire_format_guard`'s frozen `PRE_MIGRATION_OPERATION_WIRE` table removed,
  replaced with `operations_round_trip_text_and_binary` (6 representative new operations).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten to the real 28-keyword grammar
  (post-kebab-fix keyword spellings).
- `📦️packages/🦀️rust/📦️glue.rs` — `mutations` block's 6 old mounts replaced with 28 new ones.
- `🎛️apps/🖐️5d/🦀️component.rs`, `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`,
  `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` — 3 `SetPart{..}` test call sites →
  `crate::artifacts::puzzle5d::mutations::create_part(part, None)`.

## sharedFileRequests
None.

## allowlistKeysToRemove
None seeded for this facet at time of run; zero `mutation-migration/semantic-vocabulary` breaches
under `✏️s/🔌️plugins/🧩️puzzle` in the post-change policy scan.

## Gates
See the `🧊️3d` report for the full verbatim evidence (shared crate, run once for all three
facets). Summary specific to this facet: `cargo check -p semio-s-plugin-puzzle` run #1 (before any
gate) reported 11 errors, ALL `#[derive(Mutations)]` const-eval kebab-mismatch panics in this
facet's `*Part2d`/`*Part3d` variants (`SEMANTICS.kind` was authored as `part-2d`/`part-3d`, the
derive's actual kebab form is `part2d`/`part3d` — no hyphen inserted before a digit run). Fixed by
renaming the 11 affected slugs/directory-names/`kind`/`#[dsl(keyword=...)]` strings/grammar lines
from `part-2d`/`part-3d` to `part2d`/`part3d`. Two subsequent `cargo check -p semio-s-plugin-puzzle`
re-runs each hit unrelated, actively-changing breakage inside a FRAMEWORK file
(`🧰️framework/…/🏪️store/🦀️component.rs`, concurrent ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-
SYSTEM` — verbatim errors differ between the two runs, proving another session is mid-edit there
right now) — neither re-run reported a single error under `✏️s/🔌️plugins/🧩️puzzle` (grepped and
confirmed empty both times), so this facet's own fix is verified-fixed by the FIRST run's error set
going to zero on the SECOND run, even though neither run reached a fully green crate. `blocked-
churn` on the framework side; not this facet's bug.

## lawTests
Extended `🧬️mutations/🦀️component.rs`'s `#[cfg(test)] mod tests`:
- `assert_mutation_inverse_law`: `create_part`/`delete_part`, all 13 part-field kinds
  (`move_part2d`/`replace_part2d_geometry`/`edit_part2d_text`/`change_part2d_icon`/
  `change_part2d_hidden`/`change_part2d_locked`/`move_part3d`/`rotate_part3d`/`scale_part3d`/
  `change_part3d_mesh`/`edit_part3d_label`/`change_part_kind`/`change_part_anchor`),
  `add_part_grip`/`remove_part_grip`/`replace_part_grip`, `connect_grips`/`disconnect_grips`/
  `replace_fastener_geometry`/`change_fastener_kind`, `rename_puzzle5d`/`change_domain`/
  `change_description`/`connect_kind_compatibility`/`disconnect_kind_compatibility`/
  `replace_kind_catalogs` — all 28 kinds covered.
- `assert_mutation_diff_absorb_law`: `move_part2d` (sequential move-move coalesce).
- `connect_disconnect_grips_inverse_law_and_cascade`: hand-written cascade assertion (delete-part
  severs fastener) plus inverse-law calls on the connect/disconnect/geometry/kind kinds.
- `dispatch_registers_semantic_descriptors`: `Puzzle5dMutation::kinds().len() == 28`, every verb
  approved.
- `puzzle5d_delta_ops_round_trip_and_stay_granular`: pre-existing test, updated to assert the new
  variant names (`MovePart2d`/`CreatePart`/`DeletePart`) instead of `SetPart`/`SetSnapshot`.
- `operations_round_trip_text_and_binary` (in `💾️binary/🦀️component.rs`): `OpText`/`OpBinary`
  round-trip for 6 representative operations.
- NOT implemented: `assert_diff_algebra_between_law`/`assert_diff_algebra_inverse_law` (no
  `DiffAlgebra` impl on `Puzzle5dDiff` — same repo-wide gap as `◻2d`, not attempted here).

## Deviations (justified)
- Same geometry/connection-pose/handle-bundle rationale as `◻2d` (see that report), applied to
  `part2d` shape/extent, `part-grip` presentation, and fastener connection-pose.
- `move-part3d`/`rotate-part3d`/`scale-part3d` are 3 SEPARATE mutations (not bundled into one
  "transform" mutation) — each is already a distinct core taxonomy verb (`move`/`rotate`/`scale`)
  with its own canonical args; bundling them would violate rule 7's explicit "don't collapse
  move+resize into one combined transform setter" guidance.
- `label`/`domain`/`kindCatalogs`/`kindCompatibility` previously had NO granular mutation path at
  all (only `SetSnapshot`) — this migration is a net new capability for those fields, not a pure
  rename, flagged since it's a larger behavior change than the other two facets' 1:1 replacements.
- Schema description files beyond the grammar (`.graphql`/`.json`/`.proto`) not rewritten — same
  scope-limiting deviation as `◻2d`.
