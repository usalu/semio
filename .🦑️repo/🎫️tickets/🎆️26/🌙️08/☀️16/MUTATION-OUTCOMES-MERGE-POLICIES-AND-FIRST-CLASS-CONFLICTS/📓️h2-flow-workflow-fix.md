# Lane H2 — flow editor config/presence E0053 + workflow inverse bug

## Job 1 — `semio-s-plugin-flow` E0053 (2 errors)

Converted the two hand-written `impl Mutation<P>` blocks to the landed `MutationOutcome<Diff>` shape,
copying `✏️s/🔌️plugins/🗒️note/…/✏️editor/🎚️config` and `…/👥️presence`'s already-converted shape exactly
(whole-state replacement ⇒ `MutationOutcome::new(..)`, no cheap-equality no-op check exists for these
whole-config/whole-presence snapshots so none was invented):

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
  - `FlowConfigMutation::diff` now returns `protocol::MutationOutcome<FlowConfig>`, wraps the success
    path in `MutationOutcome::new(next)` (and the `Snapshot` early-return in `MutationOutcome::new(config.clone())`).
  - Fixed the 2 test call sites in `flow_config_operation_backwards_restores_the_pre_operation_snapshot`:
    `.diff(&base)` → `.diff(&base).into_parts().0`.
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`
  - `FlowPresenceMutation::diff` now returns `protocol::MutationOutcome<FlowPresence>`, wraps in
    `MutationOutcome::new(presence.clone())`. No test call sites referenced `.diff()` in this file.
  - No other file in the flow plugin calls `.diff()` on `FlowConfigMutation`/`FlowPresenceMutation`
    directly (grepped), so no further call-site fallout inside the lease.

**Attribution of other flow-plugin build/test failures (NOT touched, outside lease):**
- `cargo test -p semio-s-plugin-flow --lib` still fails with 8 errors (1×E0308, 7×E0599 `no method
  named 'apply' … help: 'apply_to'`) all inside
  `…/🧬️schema/🔺️diff/📝️text/🦀️component.rs:165` and `…/🧬️schema/🧬️mutations/📝️text/🦀️component.rs`
  (create/delete/connect/disconnect/duplicate test bodies still calling `.diff(&base).apply(&base)`
  instead of `.apply_to(&base)`, and `FlowDiff::diff` in the diff leaf not yet outcome-wrapped).
  `git log --date=iso -- <file>` shows the last touch on both at `2026-08-16 20:26:15` ("Refactor OS
  store schema mutations and SPR command resolution…"), a live sibling lane's in-flight `📝️text`
  mutation-fan-out that hasn't finished its own call-site fixes yet. Left untouched per lease.

## Job 2 — real bug: `WorkflowMutation::RemoveNode::inverse`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`

**Root cause:** `store::test_support::assert_operation_round_trip` (and the store's real undo path)
apply `operation.inverse(pre)` **reversed** — `let mut inverse = operation.inverse(pre); inverse.reverse();`
then folds application in that reversed order. `RemoveNode`'s inverse returned
`[AddNode, ConnectPorts.., BindParameterField.., BindInput.., BindOutput..]` — after the harness's
reversal, `AddNode` ran **last**, so the cascade re-`connect`/re-`bind` ops applied against a
document that didn't have the node back yet, tripping `WorkflowDiff::ConnectPorts`'s own
`mutation.apply.missing-target` check ("workflow edge target node does not exist"). Same bug shape
existed in `RemoveParameter` (parameter re-added after its `BindParameterField` cascade) and
`RemoveInput` (input re-declared after its `BindInput` cascade) — neither had a failing test yet
only because the existing test happened not to push a binding onto the removed parameter/input
before this session (the test at `remove_operations_backwards_restores_cascade_deleted_dependents`
now exercises all three).

**Fix:** in all three arms (`RemoveNode`, `RemoveParameter`, `RemoveInput`), build the cascade
re-`connect`/re-`bind` ops first and push the primary re-create op (`AddNode`/`AddParameter`/
`DeclareInput`) **last** in the returned `Vec` — so after the caller's `.reverse()`, the primary
re-create op runs first and the cascades (which all depend on it existing) run after. `Vec::new()`
on a missing target is unchanged (already correct per the ticket taxonomy). Test expectation itself
was correct — the bug was purely in the returned order, not what was restored.

**Verified:** `cargo test -p semio-framework --lib` → **137 passed, 0 failed** (was 136 passed, 1
failed before the fix). Full log: `🧪️h2-framework-test-after.txt` (before: `🧪️h2-framework-test-before.txt`).

## Job 3 — `WorkflowMutation`/`RunMutation` contract audit

Checked both hand-written `impl protocol::Mutation<P>` blocks in `🔁️workflow/🦀️component.rs`
(`WorkflowMutation` at line 1435, `RunMutation` at line 2236):

- **Outcome-returning `diff`:** both already return `protocol::MutationOutcome<Self::Diff>`, wrapped
  via `MutationOutcome::new(diff)` — compliant.
- **No `validate` override:** confirmed no `fn validate(&self` exists anywhere in the file. `RunMutation`
  keeps `check_not_sealed` as a plain **inherent** method (not a trait override — the trait method was
  already deleted), used only by `apply_run_operation_checked`, with a doc-comment explicitly recording
  this was `Mutation::validate` before C4/C10 deleted it. Compliant.
- **No leftover CRDT/`MutationKind` vocabulary:** grepped `MutationKind`, `CompositeMutationKind`,
  `merge_strategy`, `conflict_rule`, `reconcile(`, `Severity::Hint` — zero hits in this file.
- **"real verb-family messages where a target can be absent":** both `diff` impls are commented
  `"Mechanical wrap only (…) W0: no Error/Warning/Fatal messages added here yet"`. This is **not**
  workflow-specific slippage — the identical comment, verbatim, with the same ticket id, appears on
  every other hand-written "framework internal" `impl Mutation` the frozen contract lists alongside
  workflow: `🌊️flow/🌿️vcs::FlowMutation`, `🪐️space` (×2), `🏪️store` (`SpaceHistoryMutation` and 2 more
  sites), `♾️infinite/…/🕸️dag::DagMutation`, `💻️os/🎚️config::OpeningConfigMutation`. `📋️contract-freeze.md`
  §Fan-out recipe explicitly scopes this group as getting only "the return-type change" (contrasted
  with the 131 derived dispatch enums and the per-leaf `🧬️mutations/<kind>/` triads, which get full
  verb-family detection). Adding bespoke per-variant messages to `WorkflowMutation`/`RunMutation` now
  would diverge from this repo-wide, contract-frozen W0 boundary and from every sibling enum's current
  state — so nothing was changed here; the two impls already fully satisfy this ticket's contract at
  its current (W0) scope.

## Verification (real numbers)

- `cargo check -p semio-s-plugin-flow` → **0 errors** (26 warnings only). Log: `🧪️h2-flow-check-after.txt`
  (before-fix baseline with the 2 E0053s: `🧪️h2-flow-check-before.txt`).
- `cargo test -p semio-framework --lib` → **137 passed, 0 failed**. Log: `🧪️h2-framework-test-after.txt`.
- `cargo test -p semio-s-plugin-flow --lib` → 8 errors, all outside lease (see Job 1 attribution above).
  Log: `🧪️h2-plugin-flow-test.txt`.
- `bun ./📜️script.ts verify mutation-outcome-law` → `[verify mutation-outcome-law] passed.` (0 breaches).
  Log: `🧪️h2-mutation-outcome-law.txt`.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`

No other files edited. Ticket not closed (per lane rules).
