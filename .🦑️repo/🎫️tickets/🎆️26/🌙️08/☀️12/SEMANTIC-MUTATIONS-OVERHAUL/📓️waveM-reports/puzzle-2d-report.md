# Wave M — `puzzle/◻2d` mutations facet

## Facet
`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-puzzle`. This facet establishes the vocabulary conventions carried into `🖐️5d`
and `🧊️3d` in the same lane.

## Status
`done`

## Vocabulary derived from `Puzzle2dSnapshot`

`schema` (fixed constant), `camera` (session-only, never mutated), `nodes: Vec<Puzzle2dNode>`,
`edges: Vec<Puzzle2dEdge>`, `meta: Puzzle2dMeta { manifest_id, kind_compatibility, kind_catalogs }`.

`Puzzle2dNode` has 14 real fields beyond `id` plus a nested `handles: Vec<Puzzle2dHandle>`
sub-collection; `Puzzle2dEdge` has 13 fields beyond `id`, 8 of which (`gap/shift/rise/rotation/
turn/tilt/x/y`) are one coherent connection-pose. Field grouping follows the precedent set by
`cad`'s `replace-object-geometry` (mesh_url+extent+solid_handle+primitives bundled as one
structured "geometry" field) rather than a mechanical one-mutation-per-struct-field expansion:
`shape/radius/width/height` on a node and the 8 connection-pose fields on an edge each become one
`replace-*-geometry`; a handle's 8 presentation fields become one `replace-node-handle`; every
other field (kind/text/icon/scale/visible/locked/root/anchor on nodes, edge_kind/tips/visible/
locked on edges) is a narrow single-field `change`/`edit`/`scale` mutation.

## mutationsCreated (slug → verb → superseded old variant)

| slug | verb | superseded |
|---|---|---|
| `create-node` | create | `SetNode` (upsert half, new-id case) |
| `delete-node` | delete | `RemoveNode` |
| `move-node` | move | `SetNode` (x/y half) |
| `replace-node-geometry` | replace | `SetNode` (shape/radius/width/height half) |
| `change-node-kind` | change | `SetNode` (node_kind half) |
| `edit-node-text` | edit | `SetNode` (text half) |
| `change-node-icon` | change | `SetNode` (icon_kind half) |
| `scale-node` | scale | `SetNode` (scale half) |
| `change-node-visible` | change | `SetNode` (visible half) |
| `change-node-locked` | change | `SetNode` (locked half) |
| `change-node-root` | change | `SetNode` (root half) |
| `change-node-anchor` | change | `SetNode` (anchor half) |
| `add-node-handle` | add | `SetNode` (handles-append case, new) |
| `remove-node-handle` | remove | `SetNode` (handles-remove case, new) |
| `replace-node-handle` | replace | `SetNode` (handles-patch case, new) |
| `connect-handles` | connect | `SetEdge` (upsert half, new-id case) |
| `disconnect-handles` | disconnect | `RemoveEdge` |
| `replace-edge-geometry` | replace | `SetEdge` (connection-pose half) |
| `change-edge-kind` | change | `SetEdge` (edge_kind half) |
| `change-edge-tips` | change | `SetEdge` (source_tip/target_tip half) |
| `change-edge-visible` | change | `SetEdge` (visible half) |
| `change-edge-locked` | change | `SetEdge` (locked half) |
| `change-manifest-id` | change | `SetMeta` (manifest_id half) |
| `connect-kind-compatibility` | connect | `SetMeta` (kind_compatibility-add case, new) |
| `disconnect-kind-compatibility` | disconnect | `SetMeta` (kind_compatibility-remove case, new) |
| `replace-kind-catalogs` | replace | `SetMeta` (kind_catalogs half) |

26 mutations total (was 6: `SetNode RemoveNode SetEdge RemoveEdge SetMeta SetSnapshot`).

## genericVariantsRemoved
`SetNode`, `RemoveNode`, `SetEdge`, `RemoveEdge`, `SetMeta`, `SetSnapshot` — all deleted from
`Puzzle2dMutation`. `SetSnapshot` has NO replacement; the app's `puzzle2d_document_delta_operations`
(and the play app calling it) now emit only the 26 semantic kinds above, round-tripping through the
typed `Puzzle2dSnapshot` instead of a JSON-splicing fallback.

## Cascades
- `delete-node` severs every edge whose `source`/`target` is a handle owned by the deleted node;
  its inverse re-`create-node`s the captured node then re-`connect-handles`es each severed edge.
- `remove-node-handle` severs every edge referencing that handle's full id; inverse mirrors the
  same pattern (`add-node-handle` + re-`connect-handles`).

## filesTouched

**Created** (26 triads × {mutation,diff,inverse}.rs + mutation.ts = 104 files) under
`🧬️mutations/{🌱create-node,🗑delete-node,📍move-node,🧊replace-node-geometry,🏗change-node-kind,
✏️edit-node-text,🎨change-node-icon,📏scale-node,👁change-node-visible,🔒change-node-locked,
🌟change-node-root,⚓change-node-anchor,➕add-node-handle,➖remove-node-handle,🔌replace-node-handle,
🔗connect-handles,✂️disconnect-handles,🧮replace-edge-geometry,🏷change-edge-kind,🖇change-edge-tips,
👀change-edge-visible,🔐change-edge-locked,🆔change-manifest-id,🤝connect-kind-compatibility,
💔disconnect-kind-compatibility,📚replace-kind-catalogs}/`.

**Removed** (6 old dirs, 3 files each): `🧬️mutations/{✂️remove-edge,➖remove-node,🏷set-meta,
📄set-snapshot,📍set-node,🔗set-edge}/`. Removed stale root `🧬️mutations/📖️component.grammar.semio`
(disconnected "mesh-op" placeholder content, unreferenced after this rewrite).

**Updated**:
- `🧬️mutations/🦀️component.rs` — dispatch enum shrunk from hand-rolled `Mutation<Puzzle2dSnapshot>`/
  `Mutation<Value>` match logic to a 26-variant `#[derive(dsl::DslEnum, dsl::Mutations)]` list;
  added `puzzle2d_snapshot_mutations` (typed before/after field-diff, mirrors `sequence`'s
  `sequence_snapshot_mutations`); `ValueBridge` region rewritten to round-trip through the typed
  snapshot (`serde_json::from_value`/`to_value`) instead of hand-splicing JSON per mutation kind —
  this also deleted `puzzle2d_upsert_value_item`/`puzzle2d_remove_value_item`/
  `apply_puzzle2d_operation_to_value`/`puzzle2d_reorder_value_collection`/`puzzle2d_value_item_id`/
  `puzzle2d_value_item_index`/`puzzle2d_collect_value_collection_delta`/
  `canonicalize_puzzle2d_fixture_collections` as dead code; `PlaySnapshot` region unchanged.
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — removed dead `diff_set_node`/`diff_remove_node`/
  `diff_set_edge`/`diff_remove_edge`/`diff_set_meta`/`diff_set_snapshot`/`puzzle2d_index_of`/
  `HasId` (unused by the new triads, which address directly via `base.nodes.iter().find`).
  `apply`/`absorb`/`apply_nodes_delta`/`apply_edges_delta` untouched (already fully generic over
  the sparse delta shape, no changes needed).
- `🧬️mutations/💾️binary/🦀️component.rs` — `puzzle2d_document_vcs_replays_granular_operations` test
  now uses `create_node`; `wire_format_guard` module rewritten: the frozen pre-migration byte table
  (`PRE_MIGRATION_OPERATION_WIRE`, for `setNode`/`removeNode`/…/`setSnapshot`) is gone — that wire
  shape is banned outright, not preserved — replaced with `operations_round_trip_text_and_binary`
  asserting `print_op`/`parse_op`/`encode_op`/`decode_op` round-trip for 6 representative new
  operations.
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten from a disconnected generic
  "mesh-op" grammar to the real 26-keyword grammar.
- `📦️packages/🦀️rust/📦️glue.rs` — `mutations` block's 6 old `pub mod {remove_edge,remove_node,
  set_meta,set_snapshot,set_node,set_edge}` replaced with 26 new `pub mod <slug>` blocks, one per
  triad, each mounting `mutation`/`diff`/`inverse`.
- `🎛️apps/◻2d/🦀️component.rs` — 1 test call site (`Puzzle2dMutation::SetNode{..}` →
  `crate::artifacts::puzzle2d::mutations::create_node(node, None)`).
- `🧬️schema/📸️snapshot/📝️text/🦀️component.rs`, `🧬️schema/📸️snapshot/💾️binary/🦀️component.rs` —
  same `SetNode` → `create_node` fix in their `command_envelope_round_trip_holds_for_an_applied_
  operation` tests.

## sharedFileRequests
None — `📦️glue.rs` is owned by this lane for the whole plugin, edited directly.

## allowlistKeysToRemove
None found seeded for this facet in `POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` at the time of this run
(zero `mutation-migration/semantic-vocabulary` breaches under `✏️s/🔌️plugins/🧩️puzzle` in the
post-change scan — see Gates).

## Gates
See the `🧊️3d` report for the full verbatim run-by-run evidence (shared crate, gated once for all
three facets). Summary: `cargo check -p semio-s-plugin-puzzle` run #1 (before any facet gate)
reported ONLY 11 errors, all `Puzzle5dMutation` kebab mismatches (`🖐️5d` facet) — zero errors
anywhere under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d`. After two transient concurrent-churn
blockers (shared build-lock contention, then an unrelated framework file mid-edit by ticket
`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`) resolved on their own, an isolated-target-dir
`cargo check -p semio-s-plugin-puzzle` run completed **clean**: `Finished dev profile
[unoptimized] target(s) in 41m 10s`, `EXIT=0`, 0 errors, warnings only. `cargo test -p
semio-s-plugin-puzzle --lib` — see `🧊️3d` report for the exact pass/fail counts (same crate/run).
`bun ./📜️script.ts policy` ran clean of NEW high-priority breaches for this facet.

## lawTests
Extended `🧬️mutations/🦀️component.rs`'s `#[cfg(test)] mod tests`:
- `assert_mutation_inverse_law`: `create_node`/`delete_node` (incl. missing-target ⇒ empty inverse
  path via the law itself), `move_node`, all 8 remaining node-field kinds
  (`replace_node_geometry`/`change_node_kind`/`edit_node_text`/`change_node_icon`/`scale_node`/
  `change_node_visible`/`change_node_locked`/`change_node_root`/`change_node_anchor`),
  `add_node_handle`/`remove_node_handle`/`replace_node_handle`, `connect_handles`/
  `disconnect_handles`/`replace_edge_geometry`/`change_edge_kind`/`change_edge_tips`/
  `change_edge_visible`/`change_edge_locked`, `change_manifest_id`/`connect_kind_compatibility`/
  `disconnect_kind_compatibility`/`replace_kind_catalogs` — all 26 kinds covered, pass count = 26
  distinct call sites (some kinds exercised twice, e.g. `create_node` both empty-base and
  cascade-dependent contexts).
- `assert_mutation_diff_absorb_law`: `move_node` (sequential move-move coalesce).
- `delete_node_severs_and_reconnects_edges`: hand-written cascade assertion (not a generic law
  helper — no cascade law exists in testkit) proving the severed edge disappears after delete and
  the full round-trip (`assert_mutation_inverse_law`) still holds.
- `assert_op_text_binary_equivalence` not called directly here (facet has no such testkit helper
  import in this file); binary/text round-trip is instead covered in
  `🧬️mutations/💾️binary/🦀️component.rs`'s `operations_round_trip_text_and_binary` via
  `print_op`/`parse_op`/`encode_op`/`decode_op` directly (equivalent coverage, different call
  shape — the facet's existing test module already used this pattern pre-migration).
- `dispatch_registers_semantic_descriptors`: asserts `Puzzle2dMutation::kinds().len() == 26` and
  every kind's verb is in `protocol::APPROVED_VERBS`.
- NOT implemented: `assert_diff_algebra_between_law`/`assert_diff_algebra_inverse_law` — these
  require `DiffAlgebra` on `Puzzle2dDiff`, which this facet (like nearly every other facet in the
  repo per `📓️remaining-work-map.md`'s "rule 6 … never implemented" note) does not implement.
  Flagged as a deviation, not attempted, to stay in scope.

## Deviations (justified)
- **Geometry/handle/connection-pose bundling** (rule 2's "large structured field" exception applied
  more broadly than single fields): `replace-node-geometry` bundles 4 fields (`shape/radius/width/
  height`), `replace-node-handle`/`replace-part-grip`-style whole-handle swap bundles 8 handle
  fields, `replace-edge-geometry` bundles 8 connection-pose fields. Justified by the `cad` facet's
  own precedent (`replace-object-geometry` bundles `mesh_url+extent+solid_handle+primitives`) —
  without this grouping the facet would mint ~12 more single-field mutations for values that are
  always edited together in one property-panel/drag gesture, never independently.
- **`change-edge-tips` bundles 2 fields** (`source_tip`+`target_tip`) — same reasoning as
  `move-node`'s x+y bundling (one coherent "endpoint markers" facet, not two independent scalars).
- **ValueBridge simplification**: the pre-migration `puzzle2d_document_delta_operations` had a
  granular-with-`SetSnapshot`-fallback design; this rewrite makes the typed
  `puzzle2d_snapshot_mutations` diff exact and total (no fallback needed, matches taxonomy's "no
  whole-document mutation" mandate), which is a strictly larger change to that function than a
  literal field-for-field variant swap — flagged since it goes beyond the minimal diff a reviewer
  might expect, though it was necessary (the old fallback literally could not compile without
  `SetSnapshot`).
- Schema description files beyond the grammar (`🔗️component.graphql`, `🔣️component.json`,
  `🛰️component.proto` at the `🧬️mutations/` level) were **not** rewritten — left as pre-existing
  generic content. Time-boxed out of scope for this pass; flagged for a follow-up, consistent with
  `📓️remaining-work-map.md` marking rule 5 (grammar coverage) as never fully implemented
  project-wide either.
