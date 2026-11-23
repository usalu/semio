# Record Open: Final State (this pass)

## Result

The 3 record-open tests still fail (`semio-os-store-member-open-store --lib`: 1 passed, 3 failed). The earlier "pending decoder requires original recovery" is gone after the revert and the `retain_dispatch()` fixture; the failure is now a different stall.

## Findings

- Test 1 (`genuine_original_drawing_member_...`): `original member open close refused` at the first `close_open` (cut 0 path, close of an open that never ran).
- Tests 2 and 3: `original Drawing open needed more than 1000000 turns`. Debug traces (removed again): the first 5 `receive_step` polls complete (phase 1 and 2 turns copy real bytes, up to 72280 B capacity), then every later Receive turn (phase 6) polls `receiver.advance` to `RetainedWorkPoll::Pending` forever with 0 copied items and the 1 byte control charge. The issued grant is 32 items, 512 KiB, work 1048576, so it is not a funding shortfall.
- Per turn the job context (`🧵️job/⏱️context/🛬️owned`, `original_receiving_outcome`) logs: scope admitted with `admit_work=0`, `wanted_work=existing_work=1048572`, `retirement=false`, `spent=0`, i.e. a scope is claimed anew every turn and the operation returns Pending without running the producer. `ready` is true (fixture clock 1, deadline u64::MAX). The stall is therefore in the receiving effect/Work admission (`prepare_original` -> `recover_grant`/`prepare_work` returning not-ready, or `recover_partial` returning `pending()` in `OriginalJobWorkDriver`, `🧵️job/🛬️receiving/🎟️work`), the area the concurrent agent is editing (host-config effect-run refusal). I did not edit it.
- FillStyle tag (b), static analysis (not reproduced, receive stalls earlier): `FillStyle` is a `DslEnum` under `#[dsl(statements, block)] Option<FillStyle>`; the DSL source (`🗣️dsl/🧬️schema/📐️model/🧳️ownership/🛬️source.rs`) exposes the variant through `variant_tag()` only for `Statements` with exactly one statement. The selector (`🔀️variant`) takes the direct path when `variant_tag()` is Some and only falls to the `kind` scan (phase 2) otherwise, so a "kind missing" refusal means the selector received a source that is not a one-statement `Statements` (for example the payload, or an empty/multi list). No encoder/decoder tag-form mismatch was found: the DSL source never carries a `kind` key, by design. To confirm, log `tree_kind`/`variant_tag` at selector phase 1 once Receive turns progress.
- Own-wins serde parity: not implemented (matcher still reports Ambiguous for a same-name own/flatten key); no Drawing type triggers it, so it does not block the tests.

## Cleanup

All `[DEBUG]` prints I added to `🫴️record/🦀️.rs` are removed (the file diff vs HEAD is only the earlier resuming-funding change and removal of two committed debug prints). Other agents' `[DEBUG]` prints in `🧵️job` remain theirs. Generated logs `seed-rcv.txt` deleted; `seed-rec*.txt` belong to concurrent runs.
