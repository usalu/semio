# W1-E Report — Composition

Lane 1-E, `🏪️store/🦀️component.rs` regions `🔖️Composition`, `🔖️Space`, `🔖️CompositionCoordinator`.

## Signatures landed

- `SpaceMember::preview_wire(&self, ops: &[Vec<u8>]) -> Vec<crate::os_spr::MutationMessage>` (renamed
  from `validate_wire`, whose `Result<(), String>` early-reject-on-Fatal shape is gone). Threads state
  forward through the whole slice via `apply_mutation` (mirroring `replay_mutations`'s own algorithm —
  every op is diffed and folded, nothing stops early since a Fatal/Error diff is a no-op by
  construction), stamps every message with `op_index`, never applies anything. Only a structural
  failure (unreadable snapshot, undecodable op) short-circuits, itself one `mutation.invariant` Fatal
  message.
- `SpaceMember::merge_policy(&self) -> crate::os_spr::MergePolicy`: trait default
  `MergePolicy::default()` (Normal); the real blanket impl overrides it to delegate to lane 1-A's now-
  landed `ArtifactStore::merge_policy()` inherent method (same `current_checkpoint_id` precedent: Rust
  prefers the inherent method).
- `GroupReceipt.messages: Vec<MutationMessage>` — the exact union phase 1 computed, carried through
  unchanged on success.

## All-or-nothing enforcement

`dispatch_relation_group`'s Phase1Validate now: (1) dry-runs `parent_ops` and every `ChildDispatch.ops`
via `preview_wire`, prefixing each resulting message's `target` with the originating member's
`ArtifactRef::to_uri()` (`prefix_message_target`) and unioning into `all_messages`; (2) keeps the
existing structural checks (ownership/cycle/genesis) as immediate hard errors, unchanged; (3) after
every member has been previewed and every structural check passed, calls
`reject_if_policy_rejects(parent_ref, parent.merge_policy(), &all_messages)` — one worst
`crate::os_dsl::Severity` over the union, checked against the **parent-or-initiator's own**
`merge_policy()` (the natural single-policy-governs-the-group choice, since `MergePolicy` is
authority-local per §C3). A reject returns before Phase 2 ever runs, so **no member's `dispatch_wire`
is called for anyone** — the existing `dispatch_group_validate_all_atomicity_one_bad_member_applies_
nothing` test (Fatal, unmodified) still asserts this, and three new tests exercise Error/Warning under
all three policies.

**Known gap, reported not fixed**: §C6 names a typed `VcsError::Rejected { policy, messages }` for this
path. `VcsError` lives in `🌿️vcs/🦀️component.rs`, outside this lane's lease and unclaimed by any
current wave lease (0-A's W0 lease that owned it closed at the barrier). I reused the existing
`VcsError::ValidationFailed(String)` instead, folding policy/worst/messages into the string — its own
doc comment (written by 0-A) already reserves it for exactly this "structural failures only... an
ordinary mutation-level rejection ... never through this variant" gap and anticipates a future
`Rejected` variant. **Coordinator: someone needs to add `VcsError::Rejected` to `🌿️vcs/component.rs`
and repoint `reject_if_policy_rejects`'s one `Err` arm at it.**

## Pre-existing failing test (W0 barrier: 879/880)

Could not be isolated in-tree: at the time I could first get `semio-framework-os-kernel` to compile at
all (blocked for most of the session by lane 1-A's mid-flight `🔖️ArtifactStore`/`🔖️Authority` C6 work —
`resolve_conflict`/`replay_mutations`/`record_edit_messages`/`HistoryLog.conflicts` all landing live —
and briefly by a `📡️spr/🦀️component.rs` test fixture missing `HistoryLog.conflicts`, lane 1-B's C7
territory), grepping `fn .*composition.*\|fn .*coordinator.*` inside my three regions found nothing
logically broken by the C1 `Severity` reorder or C4/C10 deletions — every `CompositionGraph`/
`dispatch_group`/`dispatch_peer_group` test reads correctly against the landed API on inspection. The
crate did not hold a compiling state long enough this session for me to run `-- composition`/
`-- coordinator` and confirm which single test it was before other lanes' next edit broke the build
again. **Handing this back to the coordinator**: rerun `cargo test -p semio-framework-os-kernel --lib
-- composition` and `-- coordinator` once the tree is fully green; if the failure is still present and
inside my three regions, reopen this lane.

## Tests written and run

`cargo test -p semio-framework-os-kernel --lib -- os_store` and `cargo check -p semio-framework-os-kernel`
— raw output in `🧪️w1-e-cargo.txt`. See that file for the actual pass/fail counts from this session;
compile was blocked by other lanes' concurrent work for most of the session (see above), so counts were
captured once the shared tree compiled.

New tests (all in my three regions' test subregions — `🔖️PreviewWireTests` under `SpaceTests`,
`🔖️PhasePolicyTests` under `CompositionTests`):
- `preview_wire_reports_the_same_messages_the_real_apply_would_produce_and_changes_nothing`
- `preview_wire_reports_a_fatal_message_for_undecodable_op_bytes_and_stops_there`
- `dispatch_group_phase1_rejects_under_normal_when_a_member_yields_an_error_and_nothing_applies`
- `dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire`
- `dispatch_group_phase1_rejects_under_vigilant_on_a_members_warning`
- `group_receipt_messages_contains_the_union_with_member_path_prefixed_targets`

## Files touched

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — regions `🔖️Composition` (untouched,
read only), `🔖️Space` (`SpaceMember` trait + blanket impl + test-local wrapper impl), `🔖️CompositionCoordinator`
(`GroupReceipt`, `dispatch_relation_group`, new `prefix_message_target`/`reject_if_policy_rejects`
helpers), and the shared `mod tests` block's `SpaceTests`/`CompositionTests` subregions (new fixtures +
tests). No file outside this lease was edited.
