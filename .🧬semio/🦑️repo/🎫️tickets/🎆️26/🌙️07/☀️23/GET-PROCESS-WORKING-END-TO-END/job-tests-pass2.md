# semio-framework-job testkit: second pass

Result: `cargo test --manifest-path 🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust/Cargo.toml --features testkit --no-fail-fast` passes in full (25 result lines ok, 0 failed). Log: `🗑️generated/stale-job3.txt`. Paths below are relative to `🧰️framework/🔨️modules/🧵️job/`.

## Product fixes

- `⏱️context/🎟️source/👶️child/🦀️.rs` `admit_step`: after `prepare` moves the cancel alias and ledger into the roots, the next step saw `cancel.is_none()` and re-admitted both. Both re-admissions are now guarded by `admission.terminal_is_empty()`.
- Same file, `close_step`: the close-time `publish` of a half-admitted child forgot `borrowed_cancel = true` (the admit path sets it). Closing then tried to return the cancel alias as a plain parent and failed with `Cancellation(Blocked(SharedAlias))`.

## Stale laws (test, fixture, schema and TS twin changed together)

- caller-source (`⏱️context/🎟️source/🧪️tests/🦀️.rs`)
  - `retire_step(close)` with a loan outstanding is now `Progress(0)`. `try_owner_into` answers `Ok(None)` while aliases are live and `retirement_is_pinned` stays false.
  - The loan's first close step is the `closing` flag: `{items 1, bytes size_of::<bool>()}`, and it is the last receipt.
  - The loan then needs further close steps (alias return, slot release), so the test loops to terminal.
  - After the failing close the receipt is `metadata()`, not the admission receipt (216 bytes).
- child-source (`⏱️context/🎟️source/👶️child/🧪️tests`)
  - Admission is 7 steps: loan, cancel alias, ledger (128 capacity), roots (`prepare`), sealed backing, publish, ready.
  - The fixture and schema now carry a `quote` per phase and `cuts` 0..7. Size-dependent copy and capacity (`SourceRoots` is about 13.7 KB) come from the product quotes (`loan_admission_demand`, `prepare_demand`, `backing_demand`, `publish_demand`), not literals.
  - The stock, initial grant, close grant and fuel are widened from the same quotes. The owned-retirement admission uses `remaining_grant()` as its copy ceiling.
  - TS twin sums: items 7, copy 0, capacity 128.
- owned-park (`⏱️context/⏸️park`)
  - `SealedShared::DUPLICATE_WORK` is 3. The first lend takes `alias` fuel, and the close loop funds `alias` per turn.
  - The fixture now has `admissionWork: 4`, `ownedFuel: 3` and `remainingFuel: 1` (schema updated).
  - The TS twin conserves `initialFuel - admissionWork - guestWork`.
- page-return (`📬️outcome/📮️return/🧪️tests/🦀️.rs`)
  - `retire_step` with an outstanding supplied page returns `Progress(0)`: the held claim pins the resident return birth (`retirement_is_pinned`). `OriginalRetirementPump` was not at fault.
- context-owner (`⏱️context/📦️owner/🧪️tests/🦀️.rs`)
  - One page action per turn (`payload_page_granted`): the reserve and allocate steps of `begin_staged_page_funded` need separate turns. A `turn` helper opens a fresh context per step.
- construction work law (`🏭️construction/🧪️tests/🦀️.rs`)
  - `bind` now pays scope, source and capture, so the loop expects only `candidateWork` and `candidateItems` plus the attempt.
- construction clock sweep
  - The constructor now births on the second turn after bind.
  - A clock refusal at the checkpoint after birth is `Canceled` and fails the admission with the born candidate still in the slot. It no longer leaves a frame-owned receipt (`pending_recovery`).
  - The test sweeps every turn. For every cut that fails with a born candidate it takes the failure, retires slot and failure, closes the admission and retires the caller. It asserts the window is hit at least once (`exercised > 0`). Ready never appears for those cuts.

## Not verified

- `bun` is not installed here, so the TS twins (`👶️child/🧪️tests/🟦️.ts`, `⏸️park/🧪️tests/🟦️.ts`) were not run. The fixtures were validated against their schemas with ajv in node (both true). The twin sums were checked by hand.
- `⏱️context/🧪️tests/📊️native-progress/retained-charge.rs` contains a `[DEBUG]` print that is not mine and was left alone.
