STATUS (final): see 'Final state' at the end. Earlier line: VERIFIED by run: 1 (reviewfix7-104), 3 (reviewfix7-109), 11a (reviewfix7-105). 7 compiled in lib but its test cannot run: the whole original_wire_tests suite fails pre-existing (reviewfix7-108, admission never reaches Ready). 8 and 14 VERIFIED (reviewfix7-110, 7 passed: new active-id test plus the existing inverse and inverse-sum laws). 5/6 (patch for store4) and 17 (test-support, never compiled) static only.

# reviewfix7 — verdicts on 1107-audit-review-9.md (S = store, A = action-bus)

| # | Verdict | Evidence / action |
|---|---|---|
| 1 | REAL | `♻️retirement:68-70` refused any populated wake; the Channel branch (:144) moves `outbound_wake` into `self.wake`, so after queues detach every step failed at `demands()` (:101). Production installs the wake (actor-runner :386). Real close does reach this stage only once the actor's remote is dropped (queue detach needs strong_count 1). FIXED-unverified: a populated wake is a paid release (`wake_release_bytes`: Arc<Wake> allocation + the callback Arc allocation when this is its last owner); the step refuses (zero progress) if the quote changed since `demands()`. Law: integration test `installed_wake_retirement` (store/🔗️backbone/🧪️wake, [[test]] in Cargo.toml; the in-crate test module is stale against the effect API with 40 pre-existing errors, run reviewfix7-101). Run reviewfix7-103 failed first with a second, deeper fault: the owner had no `next_computational_work`, so every `close_factory_ticket` on it was refused 'original retirement lacks a work quote' (fixed: quote added). reviewfix7-104: 1 passed. The wake-refusal branch itself was not seen failing before the fix (the quote fault hid it); it is removed by reading. |
| 2 | REAL, not fixed here | Zero-progress Progress while the actor holds its remote is the designed wait (detach needs sole ownership); `close_alias_step` has no callers, so nothing in the repo ever releases the remote alias explicitly; the actor drops it only with its own teardown. Needs the sync-actor owner to call the alias release before/at store close and a typed `Blocked{reason}` (W4). Cross-lane (sync actor), recorded only. |
| 3 | REAL | `lifted/♻️retirement:8` dropped `local_owner` unpaid. FIXED-unverified: seed is `(child_id, target, Option<ArtifactChildLocalOwnerAlias>)`, so the existing paid `AliasRetirement` releases it (field made `pub(crate)`). Law: integration test `retirement_seed` (store/🪆️child/🧪️retirement-seed, [[test]] in Cargo.toml; the lib test module is stale, reviewfix7-107): strong count stays 2 after seeding, was 1. reviewfix7-109: 1 passed. |
| 4 | PARTLY REAL, not fixed | The settled path is a deliberate single-turn sequence (quote +3 items, three stages); it refuses with a typed OwnershipLimit rather than loop. It does violate W2 wording (`for 0..3`, constant `work:1`) but redesign belongs to the action-bus chain owner (callers2); left alone. |
| 5 | REAL | `source:111` takes the lease before `pay`. Patch staged for store4 (claimed file): `1105b-tools/reviewfix7/patches/canonical-source-pay-before-take.md`. |
| 6 | REAL (reachability unproven) | `unreachable!` after pay in phases 2/3; same patch file. |
| 7 | REAL | An unregistered tool ended in `ValueError::invariant`. FIXED-unverified: `InvalidValue` "original wire names no registered factory". Test `original_registration_of_an_unregistered_tool_is_a_typed_invalid_value_refusal` is written but UNPROVEN: reviewfix7-108 shows all 4 original_wire_tests fail before reaching it (admission never completes in 100 turns; peers' [DEBUG] output present). |
| 8 | REAL | Removing the active alternative left a dangling active id. FIXED-unverified in `📜️space-history`: step clears the active id when it names the removed row, `touches` reports `activeAlternativeId`, `undo_rows` tracks the running active id (undo restores through the existing SetActive row). Test `removing_the_active_alternative_clears_the_active_id`. Snapshot decode still accepts a dangling id (not touched: store3 is mid-port in the lift files). |
| 9 | FALSE POSITIVE | `ControlledRetirement::new` never allocates; it errs only for a type whose `controlled_retirement_supported()` is false (static). Not reachable for `PagedBytes`. |
| 10 | DESIGN | `LocalStorageBackbonePort` is a documented best-effort mirror over an in-memory fallback; not changed. |
| 11 | a) REAL, b) REAL-LOW | a) `register_once` pushed duplicate rows for a factory listing a key twice. FIXED-unverified (`DuplicateKey`), test `register_once_rejects_duplicate_keys_within_one_factory_without_partial_registration` passes (reviewfix7-105). b) `RegisteredActionBusAlias::bus()` `expect`: reached through `Deref` in plugin instance staging; not changed (callers2/plugin). |
| 12 | FALSE POSITIVE (dead code) | `🪆️child/🧵️owner` and `👁️read` are not declared by any `mod`/`#[path]`; they are not compiled. They are orphans and should be deleted (legacy SnapshotRetirementStep API, old `read_owner` field). |
| 13 | NOT A BUG / unverified | Drop panic is the repo convention for unretired custody; discarded `factory.retire` result is carried by the reply slot. Left to store4. |
| 14 | REAL-LOW | FIXED-unverified: `SpaceHistoryDiff::inverse` carries running active/length instead of rescanning (O(n)); `undo_rows` takes them as arguments. |
| 15 | FALSE POSITIVE | Brief ruling R2/W4: Pending from a synchronous recipient is recovery-only and maps to a typed refusal. |
| 16 | FALSE POSITIVE | The `copied_items:1` no-op step is the progress the terminal-empty ladder needs; not an over-charge. |
| 17 | REAL | `mutation_report_json` replayed the inverse in listed order. FIXED-unverified: reversed like the engine and like `assert_inverse` (:161). inverseSteps output is unchanged; only `inverseSnapshot` for multi-step inverses changes. Risk: a non-Rust oracle that mirrors listed-order replay would now differ; none found by name in the store tree (full-tree grep timed out). |

## Files edited
- S/🔗️backbone/♻️retirement/🦀️.rs, S/🔗️backbone/♻️retirement/🧪️tests/🎟️admit/🦀️.rs
- S/🪆️child/📥️lifted/♻️retirement/🦀️.rs (+test), S/🪆️child/🧬️retained-clone/🦀️.rs (one visibility)
- A/📨️wire/🌱️original/🦀️.rs, A/📨️wire/🌱️original/🧪️tests/🦀️.rs, A/🦀️.rs (register_once), A/🧪️tests/🔬️unit/🦀️.rs
- S/🧬️schema/📜️space-history/🦀️.rs, S/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🧪️tests/🔬️unit/🦀️.rs
- S/🧰️test-support/🦀️.rs

## Cargo.toml edits (no dependency change)
- store/🔗️backbone and store/🪆️child: one `[[test]]` table each (name+path, emoji directory). Lock not expected to change.
## Leftovers
- Outbox/admit/retirement unit tests in the backbone lib are stale against the effect API (40 compile errors, reviewfix7-101); not mine, not touched.
- Patch for store4: 1105b-tools/reviewfix7/patches/canonical-source-pay-before-take.md

## Final state
VERIFIED (ran, rc 0): 1 backbone wake + work quote (integration test `installed_wake_retirement`, reviewfix7-104); 3 child seed (`retirement_seed`, -109); 8 and 14 schema (-110, 7 passed); 11a register_once (-105).
COMPILED, test not run: 5 and 6 canonical-edit source patch is applied (pay-before-take, two unreachable! became typed refusals); the lib compiled in reviewfix7-112, whose only errors are a stale test in canonical-json `🔤️text/🧪️tests` (retirement.step(grant)), not mine. 7 action-bus wire refusal compiled (lib), test blocked by the failing `original_wire_tests` suite (-108).
UNVERIFIED: ported backbone in-crate tests (new helper `store/🔗️backbone/🧪️effect`, `crate::test_effect::{funded,step}`; ported admit, retirement and outbox tests; in-crate wake test added). Runs -111 (14 errors, fixed afterwards: nested path and Debug), -114 (blocked by peer job-context errors), -115/-116 queued and stopped by me. 17 test-support reversed replay never compiled (-113 blocked by job-context peer errors).
Rerun commands (from C; each takes a slot):
  1105b-tools/runtime/tst.sh reviewfix7-120 "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/📦️packages/🦀️rust/Cargo.toml" native -p semio-os-store-backbone --lib
  1105b-tools/runtime/tst.sh reviewfix7-121 "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/📦️packages/🦀️rust/Cargo.toml" native -p semio-os-store-test-support --lib --no-run
  1105b-tools/runtime/tst.sh reviewfix7-122 "🧰️framework/🔨️modules/🎯️action-bus/📦️packages/🦀️rust/Cargo.toml" native -p semio-framework-action-bus --lib -- original_wire_tests
No [DEBUG] lines were added by me; no temp files outside the ticket folder. Chains in 1105b-tools/reviewfix7/*.sh.
