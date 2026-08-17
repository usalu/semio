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
`reject_if_policy_rejects(parent.merge_policy(), &all_messages)` — one worst
`crate::os_dsl::Severity` over the union, checked against the **parent-or-initiator's own**
`merge_policy()` (the natural single-policy-governs-the-group choice, since `MergePolicy` is
authority-local per §C3). A reject returns before Phase 2 ever runs, so **no member's `dispatch_wire`
is called for anyone** — the existing `dispatch_group_validate_all_atomicity_one_bad_member_applies_
nothing` test (Fatal; its `ValidationFailed` assertion updated to `Rejected`, see below) still asserts
this, and three new tests exercise Error/Warning under all three policies.

**Resolved mid-session**: §C6 names a typed `VcsError::Rejected { policy, messages }` for this path.
`VcsError` lives in `🌿️vcs/🦀️component.rs`, outside this lane's lease — I first shipped
`reject_if_policy_rejects` against the existing `VcsError::ValidationFailed(String)` as a documented
stopgap. Partway through the session another lane landed the real `VcsError::Rejected { policy,
messages }` variant (confirmed via a real test panic: `Rejected { policy: Normal, messages: [...] }`)
exactly as §C6 specifies. I repointed `reject_if_policy_rejects` (and the 4 tests/1 pre-existing test
whose assertions named `ValidationFailed`) at the real typed variant — no more gap, no coordinator
follow-up needed here.

**Second-order fix this surfaced**: the real `VcsError::Rejected` also means each member's OWN
`dispatch_wire` (Phase 2, going through lane 1-A's now-landed `ArtifactStore::dispatch`) independently
enforces THAT member's own `merge_policy()` — a SEPARATE check from my coordinator's group-level
`reject_if_policy_rejects` gate (which only consults the parent's policy). My first `LaissezFaire`-
accepts test set only the parent's policy, leaving the child at its `Normal` default; Phase 2 then
correctly rejected the child's own Error-level op via ITS OWN policy, failing my test with a real,
informative panic. Fixed by setting `LaissezFaire` on BOTH members — documented in both
`reject_if_policy_rejects`'s doc comment and the test's own comment, since it is a real, load-bearing
interaction a caller must know about (a lenient parent policy does not override a stricter child's).

## Pre-existing failing test (W0 barrier: 879/880)

**Resolved by other lanes, not by this lane.** `cargo test -p semio-framework-os-kernel --lib --
composition` → `6 passed; 0 failed`. `cargo test ... -- coordinator` → `0 passed; 0 failed` (no test
name contains that substring). Full lib suite: **935 passed; 0 failed; 0 ignored** — up from W0's
879/880, and every test in my three regions is included and green. The crate spent most of this
session unable to compile at all (lane 1-A's mid-flight `🔖️ArtifactStore`/`🔖️Authority` C6 work —
`resolve_conflict`/`replay_mutations`/`record_edit_messages`/`HistoryLog.conflicts`/`VcsError::Rejected`
all landing live during my session — plus a transient `📡️spr/🦀️component.rs` test-fixture gap, lane
1-B's C7 territory) and under heavy build-lock contention from many concurrent lane sessions (~40+
`cargo` processes observed at once); by the time it held a compiling state long enough to run
`-- composition`/`-- coordinator`, the single pre-existing failure was already gone.

## Tests written and run

Final run, this session, from `/Users/ueli/Documents/semio` — raw output in `🧪️w1-e-cargo.txt`:
- `cargo test -p semio-framework-os-kernel --lib -- os_store` → **139 passed; 0 failed; 0 ignored; 796 filtered out**
- `cargo check -p semio-framework-os-kernel` → **0 errors, 9 pre-existing warnings** (unrelated dead-code/unused-qualification)
- `cargo test -p semio-framework-os-kernel --lib -- composition` → **6 passed; 0 failed**
- `cargo test -p semio-framework-os-kernel --lib -- coordinator` → **0 passed; 0 failed** (no matching name)
- `cargo test -p semio-framework-os-kernel --lib` (full suite) → **935 passed; 0 failed; 0 ignored**

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
