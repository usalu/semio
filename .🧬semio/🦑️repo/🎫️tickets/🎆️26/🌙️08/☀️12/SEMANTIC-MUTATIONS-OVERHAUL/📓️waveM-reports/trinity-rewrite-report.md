# FacetReport — `🔱️trinity` / `♻️rewrite`

## facet
`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/`

## status
**done, but zero compile/test verification performed by me** — this is the honest caveat the
coordinator asked for. Every file below was written and every call site I could find was fixed,
but I never got a `cargo check` to run to completion for `semio-s-plugin-trinity` before the
coordinator told everyone to stop contending for the build lock, so I cannot claim this compiles.
Treat this as "ready for the coordinator's consolidated pass to requeue if it doesn't compile,"
not as a confirmed pass. Risk areas are called out explicitly under `deviations`.

## mutationsCreated
| slug | verb | entity | superseded |
|---|---|---|---|
| `edit-before-fixture` | `edit` | `before-fixture` | `SetState` (field `before_fixture_json`) |
| `edit-lhs` | `edit` | `lhs` | `SetState` (field `lhs_json`) |
| `edit-rhs` | `edit` | `rhs` | `SetState` (field `rhs_json`) |
| `change-parameter-binding` | `change` | `parameter-binding` | `SetState` (map `parameter_bindings`, upsert) |
| `remove-parameter-binding` | `remove` | `parameter-binding` | `SetState` (map `parameter_bindings`, delete) |
| `change-rule-layout-point` | `change` | `rule-layout-point` | `SetState` (map `rule_layout`, upsert) |
| `remove-rule-layout-point` | `remove` | `rule-layout-point` | `SetState` (map `rule_layout`, delete) |

Derived directly from `RewriteSnapshot`'s 5 persistent fields (3 JSON-body scalars, 2
key-addressed maps) per the coordinator's explicit instruction.

## genericVariantsRemoved
- `SetState { state: RewriteSnapshot }` — a whole-snapshot LWW register wearing a mutation costume,
  dropped outright, no replacement. This was the facet's *only* variant and its *only* mechanism —
  every command in `📜️rule/🦀️component.rs` (`nodeGraphEdit`, `setLhsJson`, `setRhsJson`,
  `setParameter`, `addRuleClause`, `patchNodes`) previously wrapped a whole computed
  `next: RewriteSnapshot` in `SetState`. All of them now compute `next` exactly as before (that
  part of their logic is untouched and still correct) and instead call the new
  `rewrite_snapshot_mutations(state, &next)` diffing helper to emit the real granular mutations.
- `resetRule` (restore the blank default rule) is the one genuine whole-document reset — rerouted
  to `HostEffect::LoadDocument` via a new `apps::rewrite::reset_document_effect` helper (mirrors
  `note`/`jack`'s own `reset_document_effect` precedent), never through the mutation enum.
- The app's `whole_document_operation` override (`Some(SetState{..})`) was deleted, falling back to
  the trait default `None` — `"document:in"` media import now reports `MediaError::NotImplemented`
  (no import mutation, per the coordinator's locked decision). The `"graph:in"` port import (which
  only ever touched `before_fixture_json`, never the whole document) was kept and rerouted to
  `edit_before_fixture(..)` directly — a real targeted edit, not a replace.

## filesTouched

**Created** (7 new triads × 6 files = 42 files):
`🖼️edit-before-fixture`, `🔍️edit-lhs`, `🎯️edit-rhs`, `🔧️change-parameter-binding`,
`🧹️remove-parameter-binding`, `📐️change-rule-layout-point`, `🗑️remove-rule-layout-point` — each
with `🦠️mutation/{🦀️component.rs,🟦️component.ts}`, `🔺️diff/{…}`, `↩️inverse/{…}`.

**Removed**:
- `🧬️mutations/🎛set-state/` (old facade triad, 3 files: mutation/diff/inverse `.rs`; had no `.ts` siblings).
- `🧬️mutations/📖️component.grammar.semio` (dead top-level grammar; real one is under `📝️text/`).

**Updated**:
- `🧬️mutations/🦀️component.rs` — dispatch enum rewritten to 7 single-tuple variants,
  `#[derive(dsl::DslEnum, dsl::Mutations)]` (this facet's payloads are all plain
  `String`/`PropertyValue`/`LayoutPoint` fields with no foreign-binding blocker, so — unlike
  `jack` — it derives `DslEnum` directly, matching the pre-existing binary-codec file's own
  comment that `RewriteRuleMutation` "already derives `dsl::DslOps` directly, unlike jack's"); added
  the `rewrite_snapshot_mutations(before, after) -> Vec<RewriteRuleMutation>` diffing helper (the
  seam every `next`-computing command now uses); `apply_rewrite_rule_mutation` reduced to a 2-line
  diff-based delegate; renamed `dispatch_rewrite_rule_state(store, state)` →
  `dispatch_rewrite_rule_mutations(store, mutations)`.
- Each of the 7 new `🦠️mutation/🦀️component.rs` leaves — `#[derive(dsl::DslRecord)]` +
  `#[serde(rename_all = "camelCase")]` + `#[dsl(keyword = "…")]` added (needed for the enum-level
  `DslEnum` derive to work); `EditRhs.new_rhs_json` carries `#[dsl(lang = "json")]` matching
  `RewriteSnapshot.rhs_json`'s own annotation; `ChangeRuleLayoutPoint.new_point` carries
  `#[dsl(block)]` (nested `LayoutPoint`, itself already `dsl::DslRecord`).
- `🧬️mutations/💾️binary/🦀️component.rs` — no logic change (pure wrapper, unaffected).
- `🧬️mutations/📝️text/📖️component.grammar.semio` — rewritten from generic `stdio.json` boilerplate
  to the real 7-keyword grammar (registered one).
- `🧬️schema/🔺️diff/🦀️component.rs` (`RewriteDiff`) — removed the
  `artifact: Option<Box<RewriteArtifact>>` field (whole-doc-replace, dead once `SetState` is gone).
- `🧬️schema/🔺️diff/📝️text/🦀️component.rs` — removed `artifact` branches from
  `apply_to_artifact`/`MutationDiff::apply`/`absorb`; deleted the orphaned `diff_set_snapshot`/
  `diff_set_state`; **fixed a real latent bug**: `absorb`'s `parameter_bindings`/`rule_layout`/
  `lod_mode_by_window` map fields used `take!` (whole-map overwrite) — two of my new
  `change-*`/`remove-*` mutations touching *different* keys in the same coalesced batch would have
  clobbered each other on absorb. Replaced with a new `merge_map_delta` helper (per-key
  `BTreeMap::extend`) for those three fields.
- `🧬️schema/🔺️diff/🔗️component.graphql` — removed the `artifact`/`RewriteArtifact` field (JSON/proto/ts siblings left as-is — see `deviations`).
- `📦️glue.rs` — replaced the `set_state` mod block with 7 new ones.
- `🎛️apps/♻️rewrite/🦀️component.rs` — added `reset_document_effect` helper; deleted
  `whole_document_operation` override and the `"document:in"` arm of `import_media`; rewrote
  `"graph:in"` to call `edit_before_fixture(..)` directly; updated 2 stale module-doc comments and
  one test assertion string that named `SetState`.
- `🎛️apps/♻️rewrite/🎮️commands/📜️rule/🦀️component.rs` — all 6 mutating commands
  (`nodeGraphEdit`, `setLhsJson`, `setRhsJson`, `setParameter`, `addRuleClauseCommand`,
  `patchNodes`) rerouted through `rewrite_snapshot_mutations`; `resetRule` rerouted to
  `reset_document_effect`; module doc-comment updated.
- `🎛️apps/♻️rewrite/🎮️commands/👁️view/🦀️component.rs` — no change (never constructed `SetState`).

## sharedFileRequests
None.

## allowlistKeysToRemove
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (no more `SetState`)
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` (no more whole-artifact `artifact` field)
- `✏️s/🔌️plugins/🔱️trinity/🎛️apps/♻️rewrite/🦀️component.rs`, `…/🎮️commands/📜️rule/🦀️component.rs` (no more `SetState` construction)

## gates
**None run to completion.** Per the coordinator's mid-task instruction I abandoned pending cargo
tasks and did not start another for this facet. I have not seen `cargo check`, `cargo test`, or
`bun ./📜️script.ts policy` output for any file in this report. This is the facet most likely to
have a real compile error — see `deviations` for the specific spots I'd check first.

## lawTests
Written into `🧬️mutations/🦀️component.rs`'s test module (none executed by me):
- `assert_op_line_round_trip` — 3 kinds directly (`edit-lhs`, `change-parameter-binding`,
  `remove-rule-layout-point`); the other 4 are exercised transitively via
  `document_text_round_trip_rewrite_rule_store`/`command_envelope_round_trip_holds_for_an_applied_operation`.
- `assert_mutation_inverse_law` — all 7 kinds, across 3 tests (`edit_mutations_inverse_law`,
  `parameter_binding_mutations_inverse_law` incl. a not-previously-present key,
  `rule_layout_point_mutations_inverse_law` incl. a not-previously-present key).
- `assert_mutation_diff_absorb_law` — 1 (`edit-lhs`, two sequential edits) — chosen as the
  representative kind; not repeated per-kind due to time.
- `dispatch_registers_semantic_descriptors` — verifies all 7 kinds' verbs are in `APPROVED_VERBS`
  and `kinds().len() == 7`.

## deviations
- **I typed one directory path incorrectly mid-session** (`🏅️standards` → a corrupted `🏅️标准`
  variant) while creating `🔍️edit-lhs/↩️inverse/🦀️component.rs`, and separately a Cyrillic
  `🧬️схема` instead of `🧬️schema` while creating its `.ts` sibling. Both were caught immediately
  (`find`-verified) and the stray directories `rm -rf`'d before the correct files were written in
  their place — I re-verified the final tree with `find` and it's clean, but flagging this in case
  a stray directory surfaces anywhere else I didn't think to re-check.
- **Zero compile verification, as stated in `status`.** Specific risk areas for the coordinator's
  pass to check first: (1) whether `#[derive(dsl::DslRecord)]` on the 7 new payload structs
  actually accepts a bare `PropertyValue` field (`ChangeParameterBinding.new_value`) the same way
  `RewriteSnapshot`'s own fields do — I inferred this from a code comment describing `PropertyValue`
  as binding via `Shape::Value`, but never saw it proven for a *mutation* payload, only for a
  snapshot; (2) whether `#[dsl(block)]` on `ChangeRuleLayoutPoint.new_point: LayoutPoint` is the
  right attribute (copied from `CreateStep.step: SequenceStep` in the sequence reference); (3)
  whether `dispatch_typed`-level command tests in `🎛️apps/♻️rewrite/🦀️component.rs` (e.g.
  `add_and_delete_rhs_set_clause`, `set_parameter_emits_one_op_and_is_undoable`) still pass now
  that mutation counts can legitimately be >1 for a command that used to always emit exactly one
  `SetState`.
- Diff facet's `🔣️component.json`/`🛰️component.proto`/`🟦️component.ts` description files left with
  the stale `artifact` field (only `🔗️component.graphql` was updated) — non-gating, time-boxed,
  same as the other two facets in this wave.
