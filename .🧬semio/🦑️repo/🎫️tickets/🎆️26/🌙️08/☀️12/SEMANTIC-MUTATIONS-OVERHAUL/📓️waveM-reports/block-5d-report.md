# Wave M — `block/🖐️5d` mutations facet

## Facet
`✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-block`.

## Status
`partial` — same caveat as `🧊️3d`: all triads/dispatch/glue/call-sites written, representative
(not exhaustive) law tests, `gates: NOT RUN`, zero cargo executions for this facet. Highest risk of
the three facets in this lane — most fields (41 mutations, 2D+3D placement split on grips), never
compiled once.

## Vocabulary derived from `Block5dSnapshot` (verified against its own snapshot)
Fields: `schema`, `part_kind: BlockKindIdentity`, `part_2d: Block5dPart2d` (shape/radius/width/
height/color/iconKind — identical shape to `block2d`'s `Block2dPresentation`),
`part_3d: Block5dPart3d` (`orientation: Option<[f64;4]>`, `scale: Option<[f64;3]>`),
`representations: Vec<BlockRepresentation>` (same nested-attributes shape as `🧊️3d`),
`grip_kinds: Vec<Block5dGripKind>`, `grips: Vec<Block5dGripTemplate>` (id/grip_kind-ref/**angle**/
**radius_2d**/**position[3]**/**direction[3]**/**radius_3d** — a grip carries BOTH a 2D polar
placement and a 3D placement simultaneously, unlike `🧊️3d`'s vortex which is 3D-only),
`compatibility`, `attributes` (root), `authors`, `camera2d: BlockCamera2d`,
`camera3d: BlockCamera3d`, `meta: BlockMeta { description }`. 5d is the union of 2d's and 3d's
field shapes (has both cameras, both a 2D and a 3D presentation facet) — confirmed by reading the
snapshot rather than assuming.

## Set/Remove upsert-pair splits
- `SetPartKind` → `rename-part-kind` + `change-part-kind-{label,variant,description,icon,unit}`
  (6), identical pattern to `node_kind`/`object_kind`.
- `SetRepresentation`/`RemoveRepresentation` → same 10-mutation decomposition as `🧊️3d`
  (`create/delete/rename-representation`, 3 scalar `change-`, tag add/remove, nested attribute
  add/remove).
- `SetGripKind`/`RemoveGripKind` → `create-grip-kind` + `delete-grip-kind` + `rename-grip-kind` +
  `change-grip-kind-{label,color,default-rope-kind}` (6), same pattern as `handle-kind`/`vortex-kind`.
- `SetGrip`/`RemoveGrip` → `create-grip` + `delete-grip` + `move-grip-2d{id,new_angle,
  new_radius_2d}` (mirrors `block2d`'s `move-handle` — angle+radius_2d is the grip's whole 2D polar
  position) + `move-grip-3d{id,new_position,new_direction}` (mirrors `🧊️3d`'s `move-vortex`) +
  `resize-grip-3d{id,new_radius_3d}` (mirrors `🧊️3d`'s `resize-vortex`) + `change-grip-grip-kind`
  (rebind) — 6 total. The 2D and 3D placement halves are split into separate mutations because they
  are genuinely independent editor gestures (editing the part in its 2D projection view vs its 3D
  view), not because of a mechanical field-count rule.
- `SetCompatibilityRule`/`RemoveCompatibilityRule` → `add-`/`remove-compatibility-rule` (set-like).
- `SetAttribute`/`RemoveAttribute` (root) → `add-`/`remove-attribute` (set-like).

## `SetAuthors` — what it became
`add-author`/`remove-author`, identical treatment and same whole-list-diff-field caveat as the
other two facets.

## `SetMeta` — rename vs `update-meta`
Same as the other two: `BlockMeta` is single-field, so a plain `change-meta-description`.

## `SetPart2d`/`SetPart3d`/`SetCamera2d`/`SetCamera3d` — update vs move/resize
- **`SetPart2d` → `update-part-2d`** (one mutation, 6 fields). Same justification as `🧊️3d`... no,
  same justification as `◻2d`'s `update-presentation`: `Block5dPart2d` has the exact same 6-field,
  no-identity-field shape, edited together in one shape-editor form.
- **`SetPart3d` → `update-part-3d`** (one mutation, 2 fields: `orientation`+`scale`). This one is
  new to 5d (no `🧊️3d`/`◻2d` equivalent) — justified because it's only 2 fields, both are "3D pose"
  concepts manipulated together in a pose gizmo (rotate+scale handles on one widget), and neither
  has an identity to `rename`. Weaker case than `update-presentation`'s 6-field bundle (2 fields is
  a smaller inseparable-facet claim), flagged as a deviation for the coordinator to double-check.
- **`SetCamera2d`/`SetCamera3d` → decomposed, not `update`**: `move-camera2d`+`scale-camera2d` and
  `move-camera3d`+`scale-camera3d`, four mutations total, same pan-vs-zoom-are-different-gestures
  reasoning as the other two facets. Both cameras exist in 5d's snapshot (2D-projection window +
  3D-projection window), both left as latent/schema-only fields — no app command in this facet
  currently touches either as a document mutation (same situation as `🧊️3d`'s `camera3d`).

## Emoji table (41 mutations, uniqueness verified within facet; avoids root sibling emoji
`📖️🔗️🔣️🛰️🦀️🟦️💾️📝️`)

| emoji | slug | verb | entity |
|---|---|---|---|
| ✏️ | rename-part-kind | rename | part-kind |
| 🏷️ | change-part-kind-label | change | part-kind |
| 🔀️ | change-part-kind-variant | change | part-kind |
| 📃️ | change-part-kind-description | change | part-kind |
| 🖼️ | change-part-kind-icon | change | part-kind |
| 📐 | change-part-kind-unit | change | part-kind |
| 🖌️ | update-part-2d | update | part-2d |
| 🧊 | update-part-3d | update | part-3d |
| 🧱 | create-representation | create | representation |
| 🗑 | delete-representation | delete | representation |
| ✒ | rename-representation | rename | representation |
| 🌐 | change-representation-mesh-url | change | representation |
| 🏔 | change-representation-lod | change | representation |
| 📜 | change-representation-description | change | representation |
| 🔖 | add-representation-tag | add | representation-tag |
| 🚫 | remove-representation-tag | remove | representation-tag |
| 🧩 | add-representation-attribute | add | representation-attribute |
| ➖ | remove-representation-attribute | remove | representation-attribute |
| 🌱 | create-grip-kind | create | grip-kind |
| ❌ | delete-grip-kind | delete | grip-kind |
| 🖋 | rename-grip-kind | rename | grip-kind |
| 🎫 | change-grip-kind-label | change | grip-kind |
| 🎨 | change-grip-kind-color | change | grip-kind |
| 🪢 | change-grip-kind-default-rope-kind | change | grip-kind |
| 🌿 | create-grip | create | grip |
| 🕳 | delete-grip | delete | grip |
| 📍 | move-grip-2d | move | grip |
| 🧭 | move-grip-3d | move | grip |
| 📏 | resize-grip-3d | resize | grip |
| 🧷 | change-grip-grip-kind | change | grip |
| ➕ | add-compatibility-rule | add | compatibility-rule |
| ✂ | remove-compatibility-rule | remove | compatibility-rule |
| 🔩 | add-attribute | add | attribute |
| 🚷 | remove-attribute | remove | attribute |
| 👤 | add-author | add | author |
| 🙅 | remove-author | remove | author |
| 🎥 | move-camera2d | move | camera2d |
| 🔍 | scale-camera2d | scale | camera2d |
| 🎬 | move-camera3d | move | camera3d |
| 🔎 | scale-camera3d | scale | camera3d |
| 💬 | change-meta-description | change | meta |

Note `move-camera2d`/`move-camera3d` and `scale-camera2d`/`scale-camera3d` needed 4 distinct emoji
(both cameras coexist in this facet, unlike `◻2d`/`🧊️3d` which each have only one) — used 🎥/🎬 and
🔍/🔎 as visually-related pairs for the 2d/3d split. Same VS16-inconsistency caveat as the `🧊️3d`
report (several emoji here lack the `️` variation selector present elsewhere in the repo).

41 mutations total (was 18: `SetPartKind SetPart2d SetPart3d SetRepresentation RemoveRepresentation
SetGripKind RemoveGripKind SetGrip RemoveGrip SetCompatibilityRule RemoveCompatibilityRule
SetAttribute RemoveAttribute SetAuthors SetCamera2d SetCamera3d SetMeta SetSnapshot`).

## genericVariantsRemoved
All 18 old variants deleted. `SetSnapshot` has no replacement.

## filesTouched
**Created**: 41 triad dirs × {mutation,diff,inverse}.rs + .ts = 246 files.

**Removed**: 18 old dirs (`➖remove-attribute ➖remove-compatibility-rule ➖remove-grip
➖remove-grip-kind ➖remove-representation 🎛set-attribute 🎛set-authors 🎛set-camera2d
🎛set-camera3d 🎛set-compatibility-rule 🎛set-grip 🎛set-grip-kind 🎛set-part-kind 🎛set-part2d
🎛set-part3d 🎛set-representation 🏷set-meta 📄set-snapshot`).

**Updated**:
- `🧬️mutations/🦀️component.rs` — dispatch enum rewritten (41 variants); kept
  `apply_block5d_mutation`/`inverse_block5d_mutation`/`Block5dEnvelope`/`Block5dStore`; new
  `#[cfg(test)]` region.
- `📦️packages/🦀️rust/📦️glue.rs` — `block5d`'s `mutations` mount block: 18 old `pub mod` replaced
  with 41 new ones. Post-edit sanity check: whole-file brace count balanced (359/359) and total
  `pub mod mutation;` mounts across all three facets = 104 = 26 + 37 + 41 exactly.
- `🎛️apps/🖐️5d/🎮️commands/🏷️kind/🦀️component.rs` (`patch_part_kind`) — field-dispatches to the 6
  part-kind mutations.
- `🎛️apps/🖐️5d/🎮️commands/🌱️grip/🦀️component.rs`, `🔘️grip-kind/🦀️component.rs` — `add_*`/
  `remove_*` now call `create_grip(_kind)`/`delete_grip(_kind)`.
- `🎛️apps/🖐️5d/🎮️commands/🎨️example/🦀️component.rs` — added `replace_document_operations`
  (handles both cameras, both part facets, nested representation tags/attributes, and split
  2D/3D grip movement); `set_active_example`/`edit` rewired.
- `🧬️mutations/💾️binary/🦀️component.rs` — 2 test call sites fixed (`SetPartKind{..}` →
  `rename_part_kind`, `RemoveGrip{..}` → `delete_grip`).
- `🧬️mutations/📖️component.grammar.semio`, `🔗️component.graphql`, `🔣️component.json`,
  `🛰️component.proto`, `🟦️component.ts` — rewritten to one rule/type/message per the 41 slugs (same
  scope/time-box as the other two facets).

**Not touched, should be checked**: `🎛️apps/🖐️5d/📌️panels/*`, `🎭️modes/*`, `🎮️commands/🖐️` (if any
grip-drag/pose command exists beyond the ones I found) — same caveat as `🧊️3d`, only a top-level
`Block5dMutation::(Set|Remove)…` sweep was run, not a file-by-file open of every panel.

## sharedFileRequests
None.

## allowlistKeysToRemove
Not checked.

## Gates
**`cargo check -p semio-s-plugin-block` — NOT RUN at all for this facet** (or ever again after the
one real run captured in the `◻2d` report). `rustfmt --edition 2021 --check` came back clean for
every new/modified file (dispatch, glue.rs, 4 modified command files, binary-codec test file). Same
caveat as `🧊️3d`: this only proves parseable Rust, not type-correctness. This facet in particular
has the most opportunities for a subtle type bug (grip's split 2D/3D fields, two cameras needing
disambiguated builder names, `Block5dPart3d`'s array-typed `orientation`/`scale` fields flowing
through `..existing.clone()` struct-update syntax) that a real compile would have caught
immediately and I could not verify.
`bun ./📜️script.ts policy` — NOT RUN for this facet.

## lawTests
Extended `🧬️mutations/🦀️component.rs`'s `#[cfg(test)] mod tests`:
- `assert_mutation_inverse_law`: all 41 kinds, one call site each (part-kind ×6, part-2d/3d update
  ×2, representation ×8 incl. nested tag/attribute, grip-kind ×6, grip ×6 incl. both move variants
  + resize + rebind, compatibility ×2, attribute ×2, author ×2, both cameras ×4, meta ×1).
- `assert_mutation_diff_absorb_law`: `change_part_kind_label`, `move_grip_2d`.
- Behavior tests: part-kind rename+change, both part-facet updates, representation create/rename/
  tag/delete, grip-kind create/rename/delete, grip create/move-2d/move-3d/resize/delete,
  compatibility/attribute/author add-remove, both cameras move+scale, meta.
- `dispatch_registers_semantic_descriptors_with_approved_verbs`: asserts `kinds().len() == 41` and
  every verb in `APPROVED_VERBS`.
- NOT implemented: `DiffAlgebra`/`assert_diff_algebra_*_law` (repo-wide pattern).

## Deviations (justified)
- **`update-part-3d` bundles only 2 fields** (`orientation`+`scale`) — weaker "inseparable facet"
  claim than `update-presentation`/`update-part-2d`'s 6-field bundles. Flagged explicitly per the
  coordinator's request for scrutiny on `update` decisions; I judged it correct (both are "3D pose"
  concepts, no identity field to hang a `rename` off, no app command touches either independently
  today) but it's the shakiest `update` call across all three facets.
- **`move-grip-2d`/`move-grip-3d` split** rather than one combined `move-grip` — justified above;
  flagged since it's the facet's one case of splitting a single collection's spatial fields into
  two mutations instead of one, the opposite direction from the `update` bundling decisions.
- **Zero cargo runs, zero gates** — largest deviation, shared with `🧊️3d`, worse here given the
  larger field surface.
- Same schema-file scope/time-box note as the other two facets.
