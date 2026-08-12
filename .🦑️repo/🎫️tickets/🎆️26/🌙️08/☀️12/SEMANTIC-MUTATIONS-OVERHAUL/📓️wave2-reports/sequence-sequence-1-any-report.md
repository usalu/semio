# Wave 2 — `sequence`/`sequence`/`1`/`any` facet report

Facet: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-sequence`

## Vocabulary derived (8 semantic mutations, 0 generic left)

| Old (generic) | New semantic mutation | Verb | Entity | Record | Notes |
|---|---|---|---|---|---|
| `StepsAdd{index,item}` | `CreateStep{step}` | `create` | `step` | `CreatedStep` | `index` dropped — `apply_steps_delta`'s `added` always appends regardless of the old index (confirmed by reading `🔺️diff/📝️text/🦀️component.rs::apply_identified_delta`), so it was dead data |
| `StepsRemove{id}` | `DeleteStep{id}` | `delete` | `step` | `DeletedStep` | captures cascade: any edge with `from==id`\|`to==id` is severed too |
| `StepsMove{id,to_index}` (generic list-reorder, never a real gesture) | `MoveStep{id,x,y}` | `move` | `step` | `MovedStep` | reinterpreted as absolute spatial reposition — the taxonomy verb the old name's own English word actually means; list order was never user-meaningful for steps (spatial `x`/`y` drives canvas display, not vec order — confirmed: no `to_index`/reorder call site anywhere in the app) |
| `StepsPatch{id,patch:{params,x,y,collapsed}}` | `EditStepParams{id,params}` + `ChangeStepCollapsed{id,collapsed}` + folded into `MoveStep` above for `x`/`y` | `edit` / `change` | `step` | `EditedStep` / `ChangedStepCollapsed` | split per recipe rule 2: one scalar setter per remaining field, `params` gets `edit` (authored content body, matches engine's real `set_step_params_json` gesture) not `change` |
| `EdgesAdd{index,item}` | `ConnectSteps{id,from,to}` | `connect` | `steps` | `ConnectedSteps` | edge-collection verb (derivation-rules §4) — matches engine's real `connect_steps` gesture, not generic create |
| `EdgesRemove{id}` | `DisconnectSteps{id}` | `disconnect` | `steps` | `DisconnectedSteps` | matches engine's real `disconnect_steps` gesture |
| `EdgesMove{id,to_index}` (generic list-reorder, never a real gesture) | **dropped** | — | — | — | no display-order semantics for edges either; no call site anywhere |
| `EdgesPatch{id,patch:{from,to}}` (option-bag payload — forbidden vocabulary) | **dropped** | — | — | — | no live gesture rewires an existing edge's endpoints (only connect/disconnect exist in the engine); re-pointing an edge is `disconnect-steps` + `connect-steps` of the same id, already fully expressible with the two mutations above — `sequence_snapshot_mutations` emits exactly that pair when it detects an edge's `from`/`to` changed |
| — | `DuplicateStep{source_id,new_id,x,y}` | `duplicate` | `step` | `DuplicatedStep` | new: id-keyed collections get a `duplicate` per taxonomy (`Copy an element to a new identity/position`); not wired to a UI gesture yet, but a legitimate, schema-grounded addition (mirrors `draw`'s `DuplicateLayer`) that fills the vocabulary out to a full 8-slot budget (see Directory/glue.rs section) |

`schema: String` (envelope/version marker, root scalar) was correctly left with no mutation — never
diffed or set by the engine (confirmed: only `steps`/`edges` are read/written anywhere in
`⚙️engine/🦀️component.rs`). `slot`/`kind` on `SequenceStep` are create-time-only per the pre-existing
`SequenceStepPatch` doc comment ("fixed for a step's lifetime") — correctly excluded from any
`change`/`edit` mutation, only ever set via `create-step`'s full payload.

Every `SEMANTICS.kind` matches its variant's own kebab form and its triad-dir stem exactly, and every
`verb` (`create`/`delete`/`move`/`edit`/`change`/`connect`/`disconnect`/`duplicate`) is in
`APPROVED_VERBS` (derive-enforced compile-time asserts on each variant — could not run the actual
assertion because of the `glue.rs` blocker below; re-checked by hand instead, same as this wave's
`flow`/`vcs`/`writer` reports).

## Real handcrafted diffs (no apply-then-capture)

Every `🔺️diff` leaf builds `SequenceDiff` directly from the payload (`SequenceStepsDelta`/
`SequenceEdgesDelta` struct literals), never apply-then-capture and never routed through
`protocol::CollectionMutation` (kept purely internal to the OLD scaffold, now fully retired from this
facet):

- `create-step`: `steps.added = [payload.step]`.
- `delete-step`: the one genuinely new piece of logic — cascades into every edge whose `from`/`to`
  references the deleted step id (`edges.removed`), alongside `steps.removed = [id]`.
- `move-step`/`edit-step-params`/`change-step-collapsed`: single `SequenceStepPatchEntry` write
  (real `SequenceStepPatch`, never option-bag-as-mutation-payload — the patch type stays diff-internal
  per the taxonomy's forbidden-vocabulary rule).
- `connect-steps`: `edges.added = [SequenceEdge{id,from,to}]` built straight from the payload.
- `disconnect-steps`: `edges.removed = [id]`.
- `duplicate-step`: looks up `source_id` in `base`, builds a full `SequenceStep` copy at the payload's
  `new_id`/`x`/`y` (kind/params/collapsed carried from the source); missing source ⇒
  `SequenceDiff::default()` (a real, empty, total diff — never a panic).

## Real handcrafted inverses (computed from `base`, never by inverting the diff structurally)

- `create-step` ↔ `delete-step` are exact partners (taxonomy pair); `create-step`'s inverse needs no
  `base` lookup (the id is in its own payload).
- `delete-step`'s inverse re-`create-step`s the captured BASE step, then re-`connect-steps`s every
  edge BASE shows touching it — matching the taxonomy's "delete captures cascade... re-`connect`ed
  after `create`" rule. Missing target ⇒ `Vec::new()`.
- `connect-steps` ↔ `disconnect-steps` are exact partners; `disconnect-steps`'s inverse reconstructs
  the full edge (id + both endpoints) from `base`.
- `move-step`/`edit-step-params`/`change-step-collapsed` look up the OLD value from `base` and
  re-emit the same verb with it; `Vec::new()` if the target is gone.
- `duplicate-step`'s inverse is unconditionally `delete-step(new_id)` — no `base` lookup needed, the
  new id is in the payload; if the source was missing (diff was a no-op) this harmlessly no-ops on
  apply too.

## Directory rename + `📦️glue.rs` mechanism blocker (same pattern as this wave's `flow`/`vcs`/`writer`)

The 8 old generic scaffold dirs (`➕steps-add`, `➖steps-remove`, `↔️steps-move`, `🩹steps-patch`,
`➕edges-add`, `➖edges-remove`, `↔️edges-move`, `🩹edges-patch` — all pre-existing stubs: empty
`🔺️diff`, and `🦠️mutation`/`↩️inverse` that only delegated back to the old hand-written
`apply_sequence_mutation`/`Mutation::inverse` dispatch) were renamed (filesystem `mv`, inside my
boundary) to the correct kind-matching slugs:

| Old dir | New dir |
|---|---|
| `➕steps-add` | `🌱create-step` |
| `➖steps-remove` | `🗑️delete-step` |
| `↔️steps-move` | `📍move-step` |
| `🩹steps-patch` | `🔧edit-step-params` |
| `➕edges-add` | `🔗connect-steps` |
| `➖edges-remove` | `✂️disconnect-steps` |
| `↔️edges-move` | `🗂️change-step-collapsed` |
| `🩹edges-patch` | `🧬duplicate-step` |

`semio-s-plugin-sequence` is `[lib] path = "📦️glue.rs"`; that file (explicitly off-limits per the
task's hard boundary) hand-lists every triad leaf with its own `#[path = "..."]` — no glob/
auto-discovery. The rename leaves `glue.rs`'s `pub mod mutations { ... pub mod edges_move { ... } ...
}` block (lines 92–172) pointing at 8 now-nonexistent paths (the exact triad-dir-stem-matches-kind
shape `derivation-rules.md` requires is otherwise unreachable — the alternative, cramming all 8
semantic mutations into the OLD `steps_add`/`edges_move`/etc. **module names** unchanged, was tried
first and does compile, but leaves 2 of 8 leaves' directory name mismatched with their `kind` slug
(e.g. a `change-step-collapsed` mutation physically living in a dir named `↔️edges-move`) — abandoned
in favor of the correct shape once I found this wave's `flow`/`vcs`/`writer` reports establishing
`blocked-mechanism` + an exact `glue.rs` patch as this ticket's actual convention).

I did not touch `📦️glue.rs`, per the hard boundary constraint. All in-boundary code is internally
consistent — every cross-file reference (`super::diff`/`super::inverse` within a leaf,
`crate::artifacts::sequence::mutations::<slug>::mutation::X` across leaves for inverse
cross-references like `delete-step` → `create-step`/`connect-steps`, `create-step` → `delete-step`,
`connect-steps` → `disconnect-steps`, `disconnect-steps` → `connect-steps`, `duplicate-step` →
`delete-step`) was written against the exact same nesting pattern `glue.rs` already uses, and
re-checked by hand.

### Exact `📦️glue.rs` patch

Full ready-to-paste replacement for the `pub mod mutations { ... }` block
(`📦️packages/🦀️rust/📦️glue.rs`, lines 92–172) is saved at:
`/private/tmp/claude-501/-Users-ueli-Documents-semio/5170febb-8580-4df7-9a13-8950b45be8bd/scratchpad/new-mutations-block.txt`
(generated programmatically from the new dir names — no hand-typed emoji paths — one
`pub mod <slug> { mutation; diff; inverse; }` sub-block per new mutation). The original block is
saved alongside it as
`/private/tmp/claude-501/-Users-ueli-Documents-semio/5170febb-8580-4df7-9a13-8950b45be8bd/scratchpad/old-mutations-block.txt`
for diffing.

### sharedFileRequests

1. **`📦️packages/🦀️rust/📦️glue.rs`** (lines 92–172) — apply the exact patch above.
2. **`🎛️apps/🎬️sequence/🦀️component.rs`** (line 130, inside `import_media`'s `steps:in` handler) —
   only direct construction of an old generic variant anywhere under `🎛️apps/**` (grepped
   exhaustively for `SequenceMutation::(StepsAdd|StepsRemove|StepsMove|StepsPatch|EdgesAdd|
   EdgesRemove|EdgesMove|EdgesPatch)`; every other app command — `add_step`/`add_step_dropped`/
   `add_step_to_slot`/`move_step`/`remove_step`/`set_step_collapsed`/`set_step_params`/
   `connect_steps`/`disconnect_steps` in `🎮️commands/*` — already goes through
   `ops_from_host_mutation`/`sequence_snapshot_mutations`, which I updated in-boundary and now
   transparently emits the new semantic vocabulary with zero call-site changes needed). Change:
   ```rust
   // before:
   Ok(Emit::mutations(vec![SequenceMutation::StepsAdd { index: fixture.steps.len(), item: step }]))
   // after:
   Ok(Emit::mutations(vec![crate::artifacts::sequence::mutations::create_step(step)]))
   ```

## Testkit law coverage (recipe step e)

Extended the facet's existing `🧪️Tests` region in the dispatch `🦀️component.rs` (region
`🔖️MutationLaws`) with `protocol::testkit::{assert_mutation_inverse_law, assert_mutation_diff_absorb_law}`
calls for `create-step`, `delete-step` (incl. its edge-cascade case), `move-step`,
`connect-steps`/`disconnect-steps`, `duplicate-step` (inverse law), plus a `move-step` diff-absorb-law
test and a descriptor-registration test (`SequenceMutation::kinds().len() == 8`, every `verb` approved).
`semio-s-plugin-sequence`'s `Cargo.toml` already depends on `semio-framework-os-kernel` (aliased
`protocol`) so `protocol::testkit::*` needed no new Cargo dependency.

## Files touched (all inside `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence`)

- `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — rewritten: tuple-variant
  `SequenceMutation` enum + `#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum,
  dsl::Mutations)]` + `#[mutations(snapshot = SequenceSnapshot, diff = SequenceDiff, schema =
  "sequence.sequence")]`; old hand-written `impl Mutation<SequenceSnapshot> for SequenceMutation` and
  the `steps_operation_from_collection`/`edges_operation_from_collection` `CollectionMutation` bridge
  functions deleted (derive generates the `Mutation`/`SemanticMutation` impls now).
  `sequence_snapshot_mutations(before, after)` rewritten to emit the 8 new semantic mutations (drops
  the dead `index` tracking, adds edge-endpoint-change detection as a `disconnect`+`connect` pair);
  `apply_sequence_mutation`/`inverse_sequence_mutation` free functions kept (still just thin
  `protocol::Mutation`/`MutationDiff` forwards, callers unaffected). Tests region extended (see above).
- New/renamed triad leaf dirs (`.rs` only; `.ts` stub files travelled with the `mv`, still
  `export {};`): `🌱create-step`, `🗑️delete-step`, `📍move-step`, `🔧edit-step-params`,
  `🗂️change-step-collapsed`, `🔗connect-steps`, `✂️disconnect-steps`, `🧬duplicate-step` — each
  `{🦠️mutation,🔺️diff,↩️inverse}/🦀️component.rs` fully rewritten per the derivation-rules shape.
- `🧬️mutations/📝️text/🦀️component.rs` — rewritten: the old `SequenceMutationDsl` struct-variant
  mirror enum + its manual `to`/`from`-dsl bridge functions deleted entirely (no longer needed — every
  `SequenceMutation` variant now wraps a payload struct that derives `dsl::DslRecord` with its own
  `#[dsl(keyword = "...")]`, and `SequenceMutation` itself derives `dsl::DslEnum`, so `DslVariants` is
  generated directly on it); kept the handcrafted `impl protocol::OpText for SequenceMutation` /
  `impl protocol::OpBinary for SequenceMutation` (P6: derive doesn't emit these), now delegating to
  `SequenceMutation`'s own `DslVariants` impl instead of the deleted mirror — matches the exact pattern
  `flow`/`draw`'s completed migrations use.
- `🧬️mutations/💾️binary/🦀️component.rs` — simplified to just the protocol doc-string consts +
  `encode_op`/`decode_op` free-function wrappers + tests (the `OpText`/`OpBinary` impls moved to
  `📝️text` above, matching the reference shape); tests rewritten to use `create_step`/`delete_step`/
  `move_step`/`connect_steps` instead of the deleted struct-variant constructors.
- `🧬️mutations/📝️text/📖️component.grammar.semio` — updated to honestly list the 8 new mutation
  keywords (was: a stale unrelated placeholder body left over from an early scaffold, `"schema"
  "stdio.json"` — never matched this facet's actual shape even before this migration).

## Deferred (not blocking, per the ticket's step f)

- `🧬️mutations/💾️binary/📡️component.protocol.semio` — left untouched (generic binary
  magic/header/footer envelope framing doc, not per-keyword content; already accurate at that level of
  abstraction).
- `🧬️mutations/🔣️component.json` / `🔗️component.graphql` / `🛰️component.proto` at the facet root — left
  untouched; these already described the SNAPSHOT shape, not the mutation vocabulary, before this
  migration (pre-existing staleness, not introduced here).
- No per-triad `.ts` mirror files were newly authored (only `.rs`); the pre-existing per-leaf `.ts`
  stubs (`export {};`) simply travelled along with the directory `mv`.

## Verify

- **`cargo check -p semio-s-plugin-sequence` — red**, for two independent reasons:
  1. **`blocked-mechanism`** (primary, deterministic, this facet's own responsibility): the `glue.rs`
     rename above — confirmed by inspection (every renamed dir exists on disk with the exact new name;
     `glue.rs`'s corresponding `#[path]` entries still name the old dirs) rather than by a live
     `rustc` "couldn't read" message, because of point 2 below.
  2. **Unrelated upstream churn** (a different, concurrently-active session): `semio-s-plugin-sequence`
     depends on `semio-s-plugin-stdio` (`Cargo.toml`), which currently fails with 6 real `E0599`
     errors — `DocxBuilder::add_paragraph`/`add_table`/`add_style` not found — entirely inside
     `✏️s/🔌️plugins/🗄️stdio`'s `docx` artifact, nothing to do with `sequence` or mutations. cargo never
     reaches `semio-s-plugin-sequence`'s own compilation stage while its dependency is broken, so the
     `glue.rs` stale-path error (point 1) isn't the one currently surfaced — but it is real and
     deterministic regardless of when `stdio` gets fixed. Retried the full check twice ~15 minutes
     apart per the workspace-churn policy; the specific `stdio` failure signature changed between
     attempts (first: a missing file under `stdio`'s `png` artifact's inference facet; second: the
     `docx` `DocxBuilder` errors above) — confirming this is live, unrelated, concurrent work, not a
     stable condition chasing would fix. `.🦑️repo/…/📓️status.md` independently corroborates
     repo-wide concurrent churn in `stdio`-adjacent plugins during this same wave.
- `cargo test` not run (blocked by the same compile chain).
- All 8 mutations' inverse/absorb laws were manually traced through by hand against
  `SequenceStepsDelta`/`SequenceEdgesDelta`'s real `apply`/`absorb` semantics (documented above) since
  the test suite itself can't currently run.
