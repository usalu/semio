# Decoder prerequisite and teardown contract

## Root causes (measured, action-bus cold decoder, wide grant)
- Source loan turn: 1 item / 216 B; work issuer turn: 1 item / 32 B; return-table birth is two phases: authority admission (7 items, 2520 B, 928 B capacity) then one atomic chain (50 items, 15858 B, 5984 B capacity).
- After the table exists, every turn of that decoder pays the origin refresh from its own grant before the business runs (9 items, 6064 B, even for an idle turn). An exact business quote therefore left the business under its quote (pending forever) whenever the grant was just wide enough for the refresh.
- "decoder retains receiving stock": a settled construction that retained capacity leaves the decoder's scope stock marked retirement-required; nothing retired it, so teardown errored.
- Teardown "spin": retirement turns also pay the refresh, so a grant sized to the business alone starved the closing steps.

## Contract
- `original_return_refresh_turn_quote()` and `original_return_table_birth_quote()` (native-turn birth) are static, derived from the same chain derivation (`Ledger`) the live birth gate uses, over a cold budget.
- `original_decoder_prerequisite_quote()`: grant and fuel that admit each prerequisite turn (Source loan, work issuer, table authorities, table chain), one turn each, all-or-nothing per phase. Fixture helpers: `OriginalJobFixtureCaller::admit_decoder_step` / `admitted_decoder`.
- `original_construction_settled_turn_quote::<O>(demand, work)`: business quote plus refresh; what a caller funds one turn with. The in-turn gate keeps the business quote.
- `OriginalReturnBudget::retire_scope` + `release_original_scope`: the decoder retires its own scope stock at teardown (it closes after every owner it served), settling the unspent hold and its only Source observation. No error, no adopt/settle ceremony for owners.
- `original_decoder_teardown_quote(&decoder)` per closing turn and `ORIGINAL_DECODER_TEARDOWN_STEPS` (scope 2 + state 1 + Source loan 5) bound teardown.

## Fudges removed
job-receiving `admission_grant` (turn quote, no floors), host-configuration grant/decoder prep, scrub-host `grant()` depth floor, command runtime-test floors and decoder prep, gesture-issuer drain work (`next_computational_work`).

## Tests
action-bus: `cold_decoder_prerequisites_are_all_or_nothing_at_their_quote`, `settled_construction_admits_exactly_its_turn_quote_and_refuses_one_below_without_state_change` (cold decoder, quote settles in one turn, quote-1 no state change, teardown within the quoted steps).

## Status (end of this pass)
- Green: `test -p semio-framework-action-bus` (34, incl. the two new tests); `semio-plugin-host-configuration` cancellation test (teardown fixed); `semio-plugin-command --lib` 6/10; job-receiving 3/9; scrub-host 2/3.
- `check --all-targets` clean for job-receiving, host-configuration, command, scrub-host, plugin tool-machine (root workspace). Framework workspace: action-bus/tool-machine only pre-existing warnings; the job crate's own test sources have pre-existing errors in `⏱️context/🎟️source/👶️child/🧪️tests` and a `#[global_allocator]` conflict (not touched).
- Fixture: `OriginalJobFixtureCaller::retire` now funds the Source closure demand itself (`closure_grant`); this removed every "root closure exhausted" stall.

## Open (owner internals, not the decoder contract)
- job-receiving pack/copies/transaction: a first admit/advance turn fails with "original projection must be driven or returned through its resident frame" (source-admission frame in a projection phase) under quote-sized grants; needs the owner's per-step demands quoted (they drive steps larger than the construction quote).
- command: 4 tests (producer close needs the body-sized cursor steps; `body` is read from the admit grant).
- host-configuration first test: a turn shows heap (2,2) against receipt (2,0): an unreceipted 2-byte release inside the host owner (not decoder related).
- scrub-host: one denial assertion still compares against an unrefreshed receipt.
