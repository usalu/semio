# Settled construction admission (semio-framework-action-bus)

## Confirmed bug
`admit_original_construction_settled` only ran its quote gate when the admission was already bound
(`!terminal_is_empty`). The first call therefore charged the Source binding (1 item + 56 bytes) and
only then checked the quote against the reduced grant: a partial charge followed by refusal, so a
grant equal to the quote could never admit. The job-receiving helper compensated with +1 item and
the binding bytes.

## Contract and fix
- Quote returned to callers = full cost of a settled construction, including the binding:
  new `semio_framework_job::original_construction_settled_quote::<O>(demand, work) -> (demand, fuel)`
  (job/🏭️construction). The action-bus function uses it for both ceiling check and per-turn gate.
- Gate now runs before any charge (`output.is_none() && !committed`), so refusal never charges.
- job-receiving `admission_grant` uses the quote directly (no +1 / binding compensation).

## Tests (action-bus admission tests)
`settled_construction_admits_exactly_its_quote_and_refuses_one_below_without_state_change`:
warmed decoder, grant == quote settles in one turn; quote-1 items and quote-1 copy bytes return
`Ok(false)`, leave output empty and admission terminal-empty, with zero heap bytes.
Result: `test -p semio-framework-action-bus` 33 passed.

## Findings and open item
- A cold decoder needs extra turns for return-table birth and scope setup (2 items, ~2600 bytes,
  charged outside the quote) and the fixture caller services return births from the grant before the
  step runs (7 items on one turn), so those turns legitimately report pending.
- Extra copy bytes beyond the quote (+5200) made the warmed single-turn admission refuse in a probe
  (more grant, less admission): scope claim sizing, not yet explained.
- `test -p semio-plugin-job-receiving`: 1 passed, 8 failed. With grant == quote the cold admission
  stays `Ok(false)` forever with nothing charged (scope plan not ready: the receiving scope needs
  setup + twice its control above the body quote, ~6-8 items, which the settled quote does not
  cover once the scope credit is cold). Adding that headroom to the quote did not fix it either, so
  the scope plan (job/📬️outcome/🪙️budget/📦️stock/🎟️scope `OriginalReturnScopePlan::quote`) needs
  its own investigation. All [DEBUG] probes were removed from job-receiving and the framework.

## Update: cold decoder contract (second pass)
- Prerequisites are explicit, not part of the quote: decoder Source loan (`admit_step`) and return
  table (`admit_original_return_table`, then the fixture's birth service). `original_construction_settled_quote`
  covers bind + constructor + candidate + capture + Source outcome + first receiving scope headroom
  (scope setup 2 items / ~2.6 KB, plus twice the scope control) + settle metadata. Doc says so.
- `OriginalReturnScopePlan::quote` now claims the full quote or nothing (it used to claim a partial
  amount and report ready, which gave the body less than its quote and failed the construction).
- Action-bus test (cold decoder with table admitted): grant == quote settles in one turn; quote-1
  items / copy refuse with no state change and no heap. `test -p semio-framework-action-bus`: 33 passed.
- Open: the fixture's return-birth service takes a large share of a turn's grant (up to ~9 items,
  ~6 KB) in the first turn(s) after the table is admitted, and only runs with a bigger grant; an
  exact-quote turn is refused (pending) until that birth finishes. Finishing the birth in a
  decoder-prep helper made the later `close` spin (closing a decoder with a born table), so that
  is not solved. job-receiving (8/9 failing), host-configuration ("decoder retains receiving
  stock") and the command crate's producer tests still depend on that prerequisite and were not
  turned green. Depth (quote reports the caller's demand depth) and work bounds were not changed.
- Callers to revisit once the birth/teardown prerequisite is solved: job-receiving `admission_grant`
  (already uses the quote), scrub-host test `grant()` depth floor 4096, gesture-issuer drain work
  bound 1<<20, command runtime tests, host-configuration test decoder prep.
