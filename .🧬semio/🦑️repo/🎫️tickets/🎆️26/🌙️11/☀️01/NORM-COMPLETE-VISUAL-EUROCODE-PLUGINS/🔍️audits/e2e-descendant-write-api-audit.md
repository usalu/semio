# Descendant Write and Atomic Bundle API Review

Read-only current-source inspection. No tests, builds or edits launched.

## Mutation Execution API

Framework native trait source: 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs. protocol::Mutation<P> is metadata/codec: Sized+Sync+ToValue+FromValue, associated Diff and descriptors; it has no apply or diff method. Do not implement sequential preview by assuming mutation.apply(document).

Cold reference decision source: same mutation owner /❄️reference/🦀️.rs. ColdMutation<P> declares associated ReferenceDiff:ColdMutationDiff<P>, diff(&self,base:&P)->MutationOutcome<ReferenceDiff>, inverse(&self,base:&P)->Result<Vec<Self>,ValueError>.

Actual apply_operation<P,Mutation>(state:&P,operation:&Mutation,op_index:u32) returns (MutationApplyResult<P>,Vec<MutationMessage>), requiring P:Clone and Mutation:Mutation<P>+ColdMutation<P>. It computes operation.diff(state).stamp_op_index(op_index), extracts diff/messages, calls apply_diff_cold, and retires the reference diff. Structured apply refusal and messages remain separate. Consequently a source helper that starts executing mutations must propagate both error channels and retirement semantics, and would impose new bounds absent from current generic bundle helpers. No executed-domain success can be inferred merely from constructing a payload.

## Current Generic Bundle Signatures

Shared ✏️edit-rules/🦀️.rs under norm app-surface:

    resolve_set_fields<D,M,F>(document:&D,writes:&[FieldWrite],resolve:F)->Result<Vec<M>,String>
    D:ToValue; F:FnMut(&D,&NormEdit)->Result<Vec<M>,String>

    handle_set_fields<D,M,F>(...)->Result<Emit<M,NoConfigMutation>,Fault>
    same bounds

Single handle_set_field only requires FnOnce(&D,&NormEdit)->Result<Vec<M>,String>; no D bounds. Current resolve_set_fields validates writes, materializes tree once, coerces each value against that original tree, and calls resolve(original_document,edit) for every write. It does not evolve a shadow document. Thus two independently reconstructed replacements of one parent can overwrite the first sibling change when the emitted mutations are later applied in order.

## All Located Call Sites and Bound Propagation

- edit-rules helper: handle_set_fields delegates to resolve_set_fields then commit_snapshot_fields, one Emit.
- direct-manipulation/📍️placement-drag/🦀️.rs: resolve_placed_fields<D,M,F> requires D:ToValue and same FnMut resolver; generates placement edits first against original document, then calls resolve_set_fields(document,plain,...). handle_placed_fields carries identical bounds and commits one Emit. New Clone/FromValue/Mutation bounds would propagate here.
- shared 🧪️tests/🔬️live-acceptance/🦀️.rs macro: checks resolve_set_fields(&doc,&writes,&mut resolve) returns nonempty. Its real family instantiations would inherit any new bound.
- ⌨️command-args-and-dispatch/🧪️tests/🔬️unit/🦀️.rs: bundle tests use D=DslValue and M=String; the resolver formats operations and is deliberately not a Mutation implementation. Adding a Mutation bound breaks this existing generic contract.

No other resolve_set_fields/handle_set_fields symbol usages were found by the norm source scan.

## Existing Tests and Gaps

Command-args-and-dispatch unit tests name a_write_bundle_is_one_emit_with_one_mutation_per_leaf_and_typed_like_the_leaf, a_failing_write_fails_the_whole_bundle and set_fields_parses_and_validates_the_bundle. They cover one Emit, numeric coercion, duplicate paths, nonfinite values and count limit. They do not cover two child changes sharing a replacement parent.

An existing apparent red test should not be assumed passing: its resolver ignores the document and always returns Ok for SetField, but a_failing_write_fails_the_whole_bundle expects an unknown members[id=zz] path to fail. Current get_value_at_path(...).map_or_else uses the proposed value on lookup failure and therefore swallows that missing-path error before the accepting resolver. This is source reasoning; the test was not executed in this audit.

Canonical path implementation: app-surface/🧭️value-path/🦀️.rs defines get_value_at_path and set_value_at_path, shared with edits/remedies/overlays. Its existing test owner is app-surface/🧪️tests/🔬️unit/🦀️.rs, including set_value_at_path_resolves_id_selectors and nested_rows_are_set_inserted_and_removed_through_parent_selectors. Reuse this path grammar and its selector handling instead of splitting descendant strings on dots without preserving bracket selectors.

## Required New Invariants

Keep exact leaf rule precedence, choose only the nearest existing typed parent replacement rule, preserve siblings and selectors, and reject a path outside that parent or absent typed descendant. A bundle must see earlier validated writes when constructing a later replacement for the same parent, while publishing no partial Emit if any member fails. Cover two siblings of one parent, nested parent selectors, unrelated parents, exact-leaf-plus-parent interaction, invalid second write, numeric coercion and duplicate/ancestor-overlap policy explicitly. A shadow tree must not be presented as native mutation execution unless actual typed domain admission has also been performed. No arbitrary whole-snapshot replacement or compatibility path is necessary.
