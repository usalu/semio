# testdebtB lane (1108) - PAUSED by coordinator, handover
Runs W-testdebtB-1/2/3 were queued 35+ min and then cancelled (TERM children, then slot wrappers); no output exists, nothing is verified. All edits below are UNCOMPILED and kept.

## Edits (all unverified)
1. job-context contracts (`🧰️framework/🔨️modules/🧵️job/⏱️context/…`):
   - `🧪️contracts/owning.rs`: `pub fn quantum(caller,stock)` = 1 + `original_return_refresh_quote` items (grant derived from quote, no literal).
   - `let q|quantum=RetainedCloneGrant{maximum_items:1,..stock}` replaced by `crate::quantum(&mut caller,stock)` (declared after the caller) in: `📬️outcome/⏸️continuation/🧪️tests/{🎟️origin,📦️capsule,🎟️source}/🦀️.rs`, `⏱️context/🎟️source/{🎟️return,🎟️loan,👶️child/♻️close}/🧪️tests/🦀️.rs`, `♻️retirement/📄️payload/🎟️work/⚠️posting/🧪️tests/🦀️.rs` (`bank`). Check: the loan law asserts `copied_items==1` (was 4 red) and `source_law_2` child-ledger `work<=1` may need separate fixes; `law_4` was 0 vs 1.
2. mutation_apply (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs`): `Receiver::run` loops `RECEIVER_OPEN_TURNS=4` turns (field `born`), asserting the scope opens exactly on the last turn on a fresh Receiver, on turn 1 afterwards. If the measured count differs, set the constant to the measured value.

## Rerun commands (from repo root; C is the ticket folder)
- contracts + receiving-context: `SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-testdebtB-N "🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/Cargo.toml" native --features testkit --test original-return-and-retirement-contracts --test original-native-receiving-context`
- derive uninhabited-enum: `... tst.sh W-testdebtB-N "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust/Cargo.toml" native --test generic_retained_owner`
- decl-views tests check: `"$C/1105b-tools/chk.sh" W-testdebtB-N Cargo.toml native --tests -p semio-plugin-decl-views`
- mutation_apply laws: `1108-mutapply6-laws.sh` (one law at a time with --exact).

## Open (not started)
- receiving-context 4 reds (actionbus3 notes): receipt expectation = whole quoted scope claim; work 0 vs 1; retirement quote via `native_retirement_quote(demand,work)`; cancelled-source quote below the narrow turn.
- suspending-lease fixture for the pending-handback law; PostFixture dev-dep LOCK NEEDED if added.
- decl-views cfg(test) shapes; derive test unrun; task 4 (window-config --tests, space-collection receiving + TS law, puzzle-jobs).

STATUS: BLOCKED paused by coordinator; runs W-testdebtB-1/2/3 cancelled, edits unverified
