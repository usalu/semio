# Stale Tests, Job and Derive Failures

Build: `stale-cargo.sh` (root workspace env, `build-domain`, 3 jobs); crates run via `--manifest-path` because they are own workspaces. Prefix below: `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/`.

## Result

| Target | State |
| --- | --- |
| `semio-framework-value-native-decoding` (all targets) | compiles, all pass |
| `semio-framework-value-derive --lib` | 5 passed (both failures fixed) |
| `semio-framework-pack-record-retained --lib` | compiles; 27 of 31 pass (4 open, see below) |
| `semio-framework-job --features testkit` | 1 of 8 fixed (close-context); 7 open (see below) |

## native-decoding (done)

- `🌱️value/🛬️decode/🫴️recipient/🔁️continuation/🧪️tests/🦀️.rs`: `close_turn!` now calls `close_retirement_recipient(body, effect)` through `with_original_invocation`; the decode arm keeps the `requires_original_birth` / `close_original_birth_step` branch; encode closes through `close_retirement_recipient_into` with a local reply slot (`close_encode`); `decode(.., None, ..)` via a `hop!` macro.
- `invalid.rs`: `decode(.., None, ..)`. The envelope test moved here from the integration test because the context refusal needs `control.depth = 1` (private field, same trick as `🛂️allocation/🧪️tests/🪢️unwind.rs`); pending work alone no longer yields a context refusal in `validate_continuation`.
- `♻️retirement/🎟️continuation/🦀️.rs:35`: test-side `u128::from` (work is `u128`, fixture `u64`); not a product bug.
- Product bug fixed: `🛫️encode/🦀️.rs` `commit_closed_births` was dead code, so births of `close_retirement_recipient_into` never reached the cumulative ledger. New `close_committing` wraps the four close/recover entry points in a local reply slot and commits births before completing the caller reply. Not rebuilt downstream (plugin/io/mcp users of `_into`).
- Product bug fixed: `🛬️decode/♻️retirement/🦀️.rs` `close_retirement_recipient` never charged general-effect births to the (local) ledger; now `charge(born)` after `close_factory_ticket_controlled` when the allocation binding is local. Forwarded binding is not covered (its `charge` refuses; needs actual receipt admission).

## pack-record-retained

Ported: `🧪️tests/🔬️unit/🦀️.rs` (record-body `reserve_allocation`/`close_step` via `body_lend`), `🎟️storage/🧪️tests/🦀️.rs` (record, symbol, scalar, list `reserve_capacity_one_funded` via `funded`, controlled `step`), `📖️source/🧪️tests/🦀️.rs` (`step`/`close_step` with effect), `🫴️receiving/🧪️tests/🦀️.rs` (`error.primary()`, `u128::from`). Every `lend` now wraps the call in `with_original_invocation(None, ..)` (22 tests failed with "native retirement requires its original producer invocation" before).

Open (not product-diagnosed):
- `original_storage_tests::..._record_body_composes_real_children_for_every_prefix`: the second `admit_record_storage` (symbol page, 392 B) reports `retained_capacity_bytes = 392` but `observe_backing` sees 0 allocated. Quote `record_storage_admission_demands` carries `copy_bytes: 0` and `depth: 1` for the symbol owner while `source_reserve_demands` carries the real copy/depth; the speculative fix did not change the result and was reverted.
- `original_storage_tests::..._symbols_keep_each_partial_original_under_four_axes`: retirement loop ends with `table.terminal_is_empty() == false` (stalls for one interruption row).
- `original_record_source::tests::original_record_body_paged_source_...`: `owner.is_complete()` false after the 1000-step row.
- `original_record_receiving::tests::..._partial_original_owner_closes_under_independent_release`: `original.terminal_is_empty()` false.

Run the built binary directly with `--nocapture` (a failing test aborts on a Drop panic and loses captured output).

## derive (done, tests stale)

- `✨️derive/🏭️factory/🧪️tests/🦀️.rs`: the syn oracle searched top-level items, but `expand_retained_clone` emits its `impl RetainedClone` inside `const _: () = {..}`; the oracle now walks the const block.
- `✨️derive/📽️projection/🧪️tests/🦀️.rs`: methods are `demands`/`advance_work_demand` (param `maximum_work_units`) and `advance_into` (6 inputs, param `maximum`).

## job (--features testkit)

Fixed:
- close-context (`archive_close_defers_a_narrow_child_turn_...`): product fix in `🧵️job/⏱️context/🦀️.rs` `original_total_retained_grant`: the bound-budget branch now also intersects with `issued_ceiling`, so a narrow child turn reports its own ceiling (the Source's independent total stays in `original_source_total_grant`).

Partly ported, still failing:
- caller-source and child-source: the loan now costs `loan_admission_demand()` (adds `SealedShared::duplicate_copy_bytes`); tests widen the grant with it, expect the admission receipt with those copy bytes (`loan_demand.copy_bytes`), and the child fixture phase `original-cancellation-alias` spends 216 copy bytes. Next failures: caller-source `retire_step(close)` no longer errors while the loan is outstanding (`answer.is_err()` at 19:437); child-source `turn < 31` at 27:438.
- construction work law: `bind` now pays the scope/source/capture overhead (7 work, 7 items); the post-bind loop sees only 2 (test now sums the loop's turns, still 2 vs 8). Needs measuring from before `bind`.

Not diagnosed:
- construction clock sweep (`ready_never_escapes_...`): no sweep row reaches `pending_recovery`; fixture `clockSweep` 160 or the recovery window changed.
- context-owner: `begin_staged_page_funded` x16 never stages a page.
- owned-park: `DUPLICATE_WORK = 3` (shared original alias) but the test lends fuel 1 (`with_original_work_loan(1, ..)`), hence "alias duplicate metadata is not funded"; fixture numbers (`ownedFuel` 5, admission 2, conserved 3 in the TS law) assume cost 1, so the fixture and TS law change together.
- page-return: `retire_step(grant)` with an outstanding supplied page returns 0 items instead of the Source's birth custody (1); `OriginalRetirementPump` is the likely seat.

## Files changed

Product: `🌱️value/🛫️encode/🦀️.rs`, `🌱️value/🛬️decode/♻️retirement/🦀️.rs`, `🧵️job/⏱️context/🦀️.rs`.
Tests and fixtures: native-decoding `🫴️recipient/🔁️continuation/🧪️tests/{🦀️.rs,invalid.rs}`, `♻️retirement/🎟️continuation/🦀️.rs`; derive `🏭️factory/🧪️tests/🦀️.rs`, `📽️projection/🧪️tests/🦀️.rs`; pack `🎟️storage/🧪️tests/🦀️.rs`, `🛬️decode/🧪️tests/🔬️unit/🦀️.rs`, `🛬️decode/📖️source/🧪️tests/🦀️.rs`, `🛬️decode/🫴️receiving/🧪️tests/🦀️.rs`; job `🏭️construction/🧪️tests/🦀️.rs`, `⏱️context/🎟️source/🧪️tests/🦀️.rs`, `⏱️context/🎟️source/👶️child/🧪️tests/{🦀️.rs,🧫️fixtures/🔣️.json}`.
