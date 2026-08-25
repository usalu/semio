# P9-A1 Owned ABI Kernel Independent Audit

## Verdict

**RED — do not accept P9-A1.** The focused suite and structural gates pass, but the retained writer violates cancellation safety after page admission. The independently compiled adversarial law proves that it copies the retained page after `cancel()`. Two further source-static findings also violate the required bounded, grant-aware execution model: unbudgeted full-table scans and a page-sized staging allocation before work admission. Generation wrap additionally makes a retained old handle alias after a full `u32` cycle.

This audit made no production-source, Cargo, lockfile, dependency, Nx, script, launch, browser, or product changes. The only files created by this audit are this report and the ticket-local adversarial-law source/binaries.

## Scope Inspected

- `🧰️framework/🔨️modules/🌉️abi/🦀️component.rs`
- `🧰️framework/🔨️modules/🌉️abi/🧬️schema/🔣️component.json`
- `🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📒️ledger.tsv`
- `🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📐️limits.tsv`
- exact added mount/re-export lines in `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
- P9-A scout and the claimed implementation report

## Executed Gates

| Gate | Result | Evidence |
| --- | --- | --- |
| Direct debug Rust test | GREEN | `rustc --edition=2021 -D warnings --test`; 12 passed, 0 failed. |
| Direct optimized Rust test | GREEN | `rustc --edition=2021 -D warnings -O --test`; 12 passed, 0 failed. |
| Schema/ledger/limit parser | GREEN | Bun parsed JSON; 15 definitions, 8 ledger records, 9 max/+1 laws; version/tag order and request operation LE field verified. |
| Focused formatter | GREEN | `rustfmt --check` returned zero. |
| A1 external ABI/runtime deny-list | GREEN | Zero `JsValue`, `Promise`, `Function`, `web_sys`, `js_sys`, `wasm_bindgen`, or serde-wasm-bindgen matches in A1. |
| Added-glue deny-list and scoped diff check | GREEN | Zero prohibited added glue matches; `git diff --check` returned zero. |
| Public `AbiPort` surface inspection | GREEN | `try_send` and `poll` carry only `AbiMessage`, `AbiWorkBudget`, owned rejection/poll types, and `AbiErrorCode`; no external browser/runtime type leaks. |
| Independent cancel-after-admission law | **RED** | Ticket-local `🧪️p9a1-independent-cancel-law.rs` compiled with `-A dead_code -D warnings`; 12 embedded tests passed and the added law failed exactly as below. |

The no-edition direct command fails before tests because this source relies on Rust 2021 prelude `TryInto`; the edition-aware commands above are the valid direct compilation for the mounted crate.

## Release Blockers

### 1. Cancelled writer copies an admitted page

`AbiPageWriter::cancel` only sets `self.cancelled` at `component.rs:728-730`. `write_step` at `:692-715` never tests that state; it consults only the caller-supplied budget at `:703`. Therefore an owner can admit a page, call `cancel()`, and still advance the cursor and append page bytes using an otherwise valid budget.

The independently executed law performs exactly that sequence and failed:

```text
left:  Ok(PageComplete(0))
right: Err(Cancelled)
```

This contradicts P9-A1's required cancellation-aware retained cursor, including non-advancement and no work after cancellation. It is not covered by the existing `cancel_before_and_after_seal_are_terminal_and_non_advancing` test because that test cancels only before offering or after a seal with no pending page.

### 2. Handle reuse is not ABA-safe across generation wrap

At `component.rs:923-926`, a reusable slot applies `wrapping_add(1).max(1)`. The slot is eventually reissued with the same `u32` generation after a complete cycle, so an old `AbiHandle { slot, generation }` becomes equal to a new owner handle. The comparison at `:966-974` cannot distinguish that alias. An ABA-safe table must retire a max-generation slot or use a non-wrapping identity scheme.

### 3. Required one-grant cursor discipline is not met

- `AbiHandleTable::open` performs `iter_mut().enumerate().find(...)` at `component.rs:923`; `AbiReplyLedger::{admit,accept,lose}` perform `iter[_mut]().find(...)` at `:1007`, `:1023`, and `:1035`. Each has no `AbiWorkBudget` and can scan all 64 or 256 entries in one call.
- `AbiPageReader::read_step` changes retained state and calls `self.staging.reserve(self.target_len)` at `:807-810` before `budget.permit` at `:812`. A zero-credit, cancelled, expired, or interrupted call can therefore allocate a full 64 KiB page before it is rejected.

These are hidden multi-element work/allocation paths outside a grant, contrary to the stated rule that expensive work consume at most one semantic unit/grant. The successful tests demonstrate only byte-cursor behavior, not absence of these scans or pre-grant allocation.

## Confirmed Positive Coverage

The supplied executable suite covers empty/single/max and selected max+1 rejection, fixed deterministic ledger bytes, malformed tag/length/UTF-8/missing optional marker, unknown/stale/ABA handle cases in the non-wrap range, duplicate ACK, interruption/deadline credit refusal for a writer, cancel before and after a no-pending seal, lost/late/duplicate reply behavior, callback interruption, and interrupted/idempotent close. The independent Bun parser also verifies declared schema records and all nine `max + 1` arithmetic rows.

Those checks do not close the three blockers above. In particular, fixture arithmetic does not execute every limit constructor, and the existing cancellation law omits the pending-page state.

## Required Repair And Re-Audit

1. Make every writer cursor/close admission check its retained cancellation state before any copy or cursor mutation; add the failed hostile law as a permanent test.
2. Retire handle slots before generation wrap (or replace the identity design) and add an executable wrap-boundary law without a multi-billion-iteration test.
3. Replace unbudgeted linear handle/reply lookup with bounded one-unit cursor/index work, and move reader allocation/admission behind a successful budget grant; add zero-credit/cancel/interruption allocation and work-unit laws.
4. Re-run both direct Rust modes, parser, formatter, deny-list, scoped diff, and the expanded hostile matrix after the repair.
