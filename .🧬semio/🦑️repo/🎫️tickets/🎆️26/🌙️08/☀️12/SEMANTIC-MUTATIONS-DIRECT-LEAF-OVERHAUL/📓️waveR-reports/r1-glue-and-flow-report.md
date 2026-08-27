# Wave-R1 — glue.rs repair (writer/vcs/flow/sequence) + flow's 8 `CollectionMutation` diff rewrites

Scope: Part 1 (repair dangling `#[path]` mounts in 4 crates' `📦️glue.rs`) and Part 2 (rewrite
flow's 8 `CollectionMutation`-based `🔺️diff` leaves to construct sparse deltas directly).

## Part 1 — glue.rs repair

### `✒️writer` — `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs`

Replaced the dangling `set_text`/`set_snapshot` blocks (pointing at deleted `✍️set-text`/
`📄set-snapshot` dirs) with 4 real triad mounts, matching the dispatch's imports
(`change_language, change_uri, edit_text, rename_writer`):

- `pub mod rename_writer { ... }` → `🧬️mutations/🏷️rename-writer/{🦠️mutation,🔺️diff,↩️inverse}`
- `pub mod change_uri { ... }` → `🧬️mutations/🔗change-uri/{🦠️mutation,🔺️diff,↩️inverse}`
- `pub mod change_language { ... }` → `🧬️mutations/🌐change-language/{🦠️mutation,🔺️diff,↩️inverse}`
- `pub mod edit_text { ... }` → `🧬️mutations/✏️edit-text/{🦠️mutation,🔺️diff,↩️inverse}`

All three leaves existed on disk for each of the 4 dirs; all three mounted.

**Extra fix (outside the glue.rs boundary, but inside the mutations facet, discovered during
deep verification — see "Verification methodology" below):** writer's own
`🧬️mutations/🦀️component.rs` dispatch file had a latent `E0252` bug that a plain glue-only fix
would not have caught, because the module-resolution error at `glue.rs` always aborted the build
before rustc ever reached this file. Line 11 did
`use crate::artifacts::writer::schema::mutations::{change_language, change_uri, edit_text,
rename_writer};` — a self-import of the SAME dispatch module pulling in the very functions that
lines 39–42 already `pub use`-export (`pub use rename_writer::mutation::{rename_writer,
RenameWriter};`, missing the `super::` prefix that vcs/sequence/flow's equivalent files all use).
This produced two bindings for each of `rename_writer`/`change_uri`/`change_language`/`edit_text`
in the value namespace → "the name `X` is defined multiple times". Fixed by:
- Deleting line 11 entirely.
- Changing the enum body from `RenameWriter(rename_writer::mutation::RenameWriter)` (module-path
  style) to `RenameWriter(RenameWriter)` (bare-type style, matching sequence's clean pattern).
- Adding the missing `super::` prefix to all four `pub use` lines (39–42), matching vcs's and
  sequence's already-working pattern (`pub use super::add_tag::mutation::{add_tag, AddTag};`).

This is a one-file, mechanical, in-mutations-facet fix — not a glue.rs edit, not app-layer, not
framework churn.

### `🌿️vcs` — `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs`

Real triad dirs on disk: `✏️rename-vcs`, `🏷️add-tag`, `📝change-notes`, `🔢change-counter`,
`🗑️remove-tag`, `🚦change-status` — all 6 with complete `{🦠️mutation,🔺️diff,↩️inverse}` triads.
Old glue mounted 6 modules but at STALE dir names (`📛set-title`, `📝set-notes`, `🔢set-counter`,
`🚦set-status` — none of which exist) plus 2 correctly-named-but-incomplete ones (`add_tag`,
`remove_tag`, mounted mutation+inverse only, missing `diff`). Per the task's flagged extra: all 6
mutation leaves call `super::diff::diff`, so ALL 6 needed a `diff` mount, not just add/remove-tag.
Replaced the whole block with 6 correctly-named, fully 3-deep mounts:
`add_tag`, `rename_vcs`, `change_notes`, `change_counter`, `remove_tag`, `change_status` — each
now mounts `mutation` + `diff` + `inverse`.

### `🌊️flow` — `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs`

Replaced the 4 dangling old-generic mounts (`set_snapshot`, `set_layout`, `synapses`, `widgets` —
pointing at the 4 deleted scaffold dirs) with the 9 real per-mutation mounts the dispatch enum
requires: `create_widget`, `delete_widget`, `reorder_widgets`, `replace_widget`,
`connect_widgets`, `disconnect_widgets`, `reorder_synapses`, `update_synapse_endpoints`,
`move_widgets` — each mounting all three leaves from its own triad dir (all 9 dirs complete on
disk). First automated attempt (bulk string-index splice) mis-located the closing brace of the
outer `pub mod mutations { ... }` and silently deleted ~35 unrelated lines of the file's `io`
section; caught immediately via `cargo check`'s "unexpected closing delimiter" error, diffed
against `git show HEAD:<path>` to recover pristine content, and redone with an exact line-number
splice (verified byte-identical for everything after the replaced block via `diff`).

### `🎬️sequence` — `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs`

Real triad dirs: `🌱create-step`, `🗑️delete-step`, `📍move-step`, `🔧edit-step-params`,
`🗂️change-step-collapsed`, `🔗connect-steps`, `✂️disconnect-steps`, `🧬duplicate-step` — matching
the dispatch's 8-variant enum and its `pub use super::<slug>::mutation::{...}` imports exactly.
Old glue mounted the pre-migration generic names (`edges_move`, `steps_move`, `edges_add`,
`steps_add`, `edges_remove`, `steps_remove`, `edges_patch`, `steps_patch`) pointing at 8 deleted
dirs (`↔️edges-move`, `↔️steps-move`, `➕edges-add`, `➕steps-add`, `➖edges-remove`,
`➖steps-remove`, `🩹edges-patch`, `🩹steps-patch`). A prior wave2 session's abandoned scratch file
(`new-mutations-block.txt`, found still present in that session's old scratchpad dir and
cross-checked against the live dispatch file's imports and the live directory listing before
trusting it) supplied the exact target block, which matched the current facet state exactly.
Replaced 1:1, one triad mount per real dir.

### Verification methodology (important caveat)

All 4 crates hit the SAME pre-existing, out-of-scope blocker before reaching any mutations-facet
code: `🎛️apps/<plugin>/📌️panels/📄️document/🦀️component.rs` does not exist on disk (renamed to
`📄️artifact` by a different concurrent session per flow's wave2 report). Because Rust's module
`#[path]` resolution happens during parsing (before type-checking), and rustc appears to abort the
whole-crate build on the FIRST such resolution error it hits (not collect-and-continue across the
whole file), a plain `cargo check` against the untouched files reports ONLY this one error and
gives **no signal at all** about whether the mutations-facet code inside actually type-checks.

To get an honest signal (per "you MUST NOT say a test is passing when you didn't run it"), I
temporarily commented out just the `#[path]` attribute + `pub mod document;` line for each crate's
`glue.rs`, ran `cargo check`, and then restored the two lines to byte-identical original content
(diffed to confirm) before moving to the next crate. This is a read-only verification technique —
no net change was left in any file. Results:

- **flow**: with the parse-blocker neutralized, real type-checking proceeded and surfaced exactly
  6 errors, ALL pre-existing/unrelated churn (confirmed none touch `🧬️mutations` or `🔺️diff`):
  `FLOW_DOCUMENT_SCHEMA` not found (app-layer fallout of the same panel rename), `MdSnapshot` has
  no field `body` (stdio churn), 2× `JsonValue`/`serde_json::Value` type mismatch (stdio churn).
  **Zero errors attributable to the mutations facet — this is the real confirmation for Part 2.**
- **writer** (before the dispatch-file fix above): surfaced the `E0252` self-import bug described
  above, plus ~20 pre-existing app-layer (`WriterMutation::SetText`/`SetSnapshot` — expected, this
  is exactly the "Shared-file reconciliation needed" debt writer's own wave2 report already
  flagged and deferred) and stdio-io churn (`PageDoc`, `DocxDocument.paragraphs`, `MdSnapshot.body`,
  `JsonValue`) errors — none of which are mine to fix per the ticket's explicit scope.
- **vcs / sequence**: not independently re-verified with the disable trick this way (see below —
  blocked by escalating framework churn before I could), but their dispatch files were inspected
  by hand and both already use the correct `super::`-prefixed pattern (see "vcs/sequence dispatch
  files already correct" below); I have no reason to expect the same class of bug there.

### Escalating concurrent churn blocked full re-verification after the writer fix

After fixing writer's self-import bug, three follow-up verification attempts (spaced retries, per
this ticket's churn-retry policy) hit **worsening, clearly-unrelated framework breakage**, each a
dependency several layers below any of these 4 plugin crates:

1. First retries: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/🦀️component.rs:5415,5476`
   — `error[E0063]: missing field 'member_edits' in initializer of 'UndoGroup'` (`semio-framework-plugin`
   fails to compile at all — blocks every plugin crate in the repo, not just these 4).
2. Final official-gate run (paste below): `🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs:309`
   — `error: functions tagged with #[proc_macro_derive] must currently reside in the root of the
   crate` (`semio-framework-schema-derive`, a proc-macro crate, now fails to compile — an even
   earlier, more fundamental break than the `UndoGroup` one, confirming this is an actively
   in-progress framework refactor by another session, not a one-off transient blip).

Both are squarely inside `🧰️framework/**`, explicitly called out in this ticket's hard rules as
"not yours — report it, don't chase it." I did not touch either file. This blocks a from-scratch
official `cargo check` on all 4 crates equally right now (paste of the final attempt, run against
the untouched — non-disabled — repo state):

```
$ cargo check -p semio-s-plugin-writer   # (and -vcs, -flow, -sequence — identical failure)
error: functions tagged with `#[proc_macro_derive]` must currently reside in the root of the crate
   --> 🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust/📦️glue.rs:309:1
error: could not compile `semio-framework-schema-derive` (lib) due to 1 previous error; 1 warning emitted
```

Full output saved: `waveR-final-official-gate.txt` in this ticket folder (top level, alongside
this reports subfolder).

### What I can and cannot certify

- **Certified clean (real type-check obtained)**: flow's Part 2 diff-leaf rewrites — zero errors
  attributable to the mutations facet, confirmed via the disable-and-check technique before the
  framework churn escalated.
- **Certified structurally correct, not re-verified after the last framework-derive break**: all 4
  crates' glue.rs mounts (file paths on disk confirmed to exist for every leaf mounted; module
  names confirmed to match each dispatch file's `use`/`super::` references by hand); writer's
  dispatch self-import fix (confirmed to exactly match vcs's/sequence's already-passing pattern,
  and confirmed it eliminates the exact `E0252` diagnostic in one clean intermediate run before the
  framework broke further).
- **Not mine, unresolved, blocking further verification**: the two `🧰️framework/**` errors above.

## Part 2 — flow's 8 `CollectionMutation` diff leaf rewrites

Helper file: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`.
Inlined the matching arm of `widgets_delta_from_collection_mutation`/
`synapses_delta_from_collection_mutation` directly into each leaf, substituting the payload's own
fields, then deleted both helper fns (zero remaining callers — confirmed via
`grep -rn` across the whole `🌊️flow` plugin tree) and removed the now-unused
`protocol::CollectionMutation` import from that file. `diff_set_snapshot` was NOT deleted — it
still has one caller (`a_whole_artifact_diff_wins_over_every_collection_diff` test in the same
file), and it is the artifact's whole-snapshot-replace diff builder (unrelated to the banned
`SetSnapshot` mutation, which no longer exists in `FlowMutation` — this is the artifact's own
non-mutation reset-diff builder, exactly as the flow wave2 report already established).

| Leaf | Before (helper call) | After (direct construction) |
|---|---|---|
| `➕️create-widget` | `widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Add { index, item: payload.widget.clone() })` | `FlowWidgetsDelta { added: vec![payload.widget.clone()], ..Default::default() }` |
| `🗑️delete-widget` | `widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Remove { id: payload.id.clone() })` (cascade logic unchanged) | `FlowWidgetsDelta { removed: vec![payload.id.clone()], ..Default::default() }` (cascade logic unchanged) |
| `🔁️replace-widget` | `widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Patch { id, patch: payload.widget.clone() })` | `FlowWidgetsDelta { patched: vec![FlowWidgetPatchEntry { id: payload.id.clone(), patch: payload.widget.clone() }], ..Default::default() }` |
| `🔀️reorder-widgets` | `widgets_delta_from_collection_mutation(&base.widgets, &CollectionMutation::Move { id, to_index })` | inlined the Move-arm's index recompute over `base.widgets` (via `protocol::Identified::id()`) directly into the leaf, `FlowWidgetsDelta { reordered: Some(ids), ..Default::default() }` |
| `🔀️reorder-synapses` | `synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Move { id, to_index })` | inlined the Move-arm's index recompute over `base.synapses` (plain `.id` field) directly into the leaf, `FlowSynapsesDelta { reordered: Some(ids), ..Default::default() }` |
| `🔗️connect-widgets` | `synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Add { index, item: synapse })` | `FlowSynapsesDelta { added: vec![synapse], ..Default::default() }` |
| `✂️disconnect-widgets` | `synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Remove { id: payload.id.clone() })` | `FlowSynapsesDelta { removed: vec![payload.id.clone()], ..Default::default() }` |
| `🔄️update-synapse-endpoints` | `synapses_delta_from_collection_mutation(&base.synapses, &CollectionMutation::Patch { id, patch: synapse })` | `FlowSynapsesDelta { patched: vec![FlowSynapsePatchEntry { id: payload.id.clone(), patch: synapse }], ..Default::default() }` |

Every leaf's `pub protocol::CollectionMutation` import was removed (each leaf now imports only its
own `FlowDiff`/delta-struct types), and every leaf's doc-comment was rewritten to describe the
real direct construction instead of naming `CollectionMutation` (per the policy rule that greps
raw file content including comments).

Helper functions deleted: `widgets_delta_from_collection_mutation`,
`synapses_delta_from_collection_mutation` (both had zero remaining callers after the 8 leaves were
rewritten — confirmed via `grep -rn` over the whole `🌊️flow` plugin tree). `diff_set_snapshot` was
kept (still has a live caller, is not `CollectionMutation`-shaped, is not the banned mutation).

### Test results

`cargo test -p semio-s-plugin-flow --lib` — **not run to completion**. The crate cannot compile
right now, in this exact repo state, due to the `🧰️framework/🔨️modules/🧬️schema/✨️derive` proc-macro
break documented above (a dependency of `semio-s-plugin-flow` itself, several layers upstream, not
touched by this ticket). I am reporting this honestly rather than claiming a test run that did not
happen. The one real type-check I DID obtain (via the temporary panels/document neutralization,
before this proc-macro break appeared) showed zero errors from any file inside
`🗿️artifacts/🌊️flow`, including the 8 rewritten diff leaves and the helper file — this is strong
compile-time evidence for the rewrite's correctness, but is not a substitute for an actual
`cargo test` pass, which could not be obtained.

## allowlistKeysToRemove

Repo-relative paths of files now free of `SetSnapshot`/`NoMutation`/`CollectionMutation` tokens
(confirmed via `grep -n` on each file individually — all clean):

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-widgets/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-synapses/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️connect-widgets/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-widgets/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/🔺️diff/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs`

(The writer dispatch fix and the 4 glue.rs files are wiring-only changes, not vocabulary changes —
none introduced or removed policy-scanned tokens, so they are not part of this list.)

## Files touched

**Part 1:**
- `✏️s/🔌️plugins/✒️writer/📦️packages/🦀️rust/📦️glue.rs` (mutations block rewired)
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` (self-import bug fix)
- `✏️s/🔌️plugins/🌿️vcs/📦️packages/🦀️rust/📦️glue.rs` (mutations block rewired, diff mount added to all 6)
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/📦️glue.rs` (mutations block rewired)
- `✏️s/🔌️plugins/🎬️sequence/📦️packages/🦀️rust/📦️glue.rs` (mutations block rewired)

**Part 2:**
- 8 flow diff leaves (listed above under allowlistKeysToRemove)
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️component.rs` (helper fns deleted, unused import removed)

**Scratch/logs (ticket folder, top level):**
- `waveR-writer-check.txt`, `waveR-vcs-check.txt`, `waveR-flow-check.txt`, `waveR-flow-check2.txt`,
  `waveR-sequence-check.txt`, `waveR-final-official-gate.txt`
