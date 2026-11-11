# Lane testdebtA result (PAUSED by coordinator, handover)

## Step 1: publication-mounted lib (EDITED, NOT RUN)
Root cause of the 5 receipt tests failing "shared allocation birth is not funded": production quote fault.
- `SharedUtf8::admission_demand()` (value `📝️shared-utf8/🦀️.rs`) quoted `copy_bytes: 0`, but `admit_shared_allocation` requires `shared_allocation_value_demand::<String>().copy_bytes`. Fixed: quote now carries that copy demand (24 callers of the quote benefit; unverified for them).
- mounted `🧾️receipt/🦀️.rs` Author phase precheck now also checks `maximum_copy_bytes < demand.copy_bytes` (returns Progress(default), denied charges nothing).
- Test fixtures: added `crate::funded_copy(policy)` (`🧪️tests/⏱️effect/🦀️.rs`, re-exported in mounted `🦀️.rs`) = `policy.max(SharedUtf8::admission_demand().copy_bytes)`; replaced the literal `maximum_copy_bytes:64` by `crate::funded_copy(64)` in receipt, receipt/group, receipt/group/member, members tests. No literal caps added, no mem::forget, no wallet.
- Not yet seen: the 6th test `mounted_member_receipt_refuses_missing_original_metadata_before_acknowledgment` expecting (0,0) vs (64,0) heap (retained-clone testing 🦀️.rs:212); look at it after the rerun.
- Companion (84 `default_*` errors in value `🛬️decode/🧳️owned/🦀️.rs` lines 19-23, `value_codec!` + `derive(NativeReceive)`): derive `🛬️receive/🦀️.rs` emits `default_fields` only for non-required fields; errors say field missing, so generated code references `default_*` for fields the state struct lacks (check lines 97-98/199 vs struct at 109 and `🔀️enum`). Not investigated; recompile first, the list may have changed.

## Rerun (never started past the capacity queue)
```
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
M="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧩️composition/📬️publication/🤝️group/🪟️mounted"
SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-testdebtA-2 "$M/📦️packages/🦀️rust/Cargo.toml" native --lib
SLOT_HANG_SECS=100 "$C/1105b-tools/runtime/tst.sh" W-testdebtA-3 "$M/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml" native --no-run
```
(Queue wait was ~20 min under fleet load.)

## Step 2: pack-json panicking-observer test (NOT STARTED, analysed)
File `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🫳️borrowed-source-laws/🦀️.rs` ~918-1060: tail still uses `record_effect` + `settle_effect`; must move onto `NativeEncodeControl::effect_allocated_with_work` (value `🛫️encode/🎟️effect/🦀️.rs`, catches the panic) with a separate writer control; reference rig `🌱️value/📝️text/📦️paged/🧪️tests/encoder-rig.rs` `Payment`. `tree`/`laws` targets: see `1108-result-jsonwriter.md`.

## Steps 3-4: not started.
No Cargo.toml edited (no LOCK NEEDED). Claims on mounted tests and shared-utf8 admission_demand still held.

STATUS: BLOCKED paused by coordinator; mounted edits unverified (no label run)
